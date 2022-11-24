use std::{collections::HashMap, ffi::c_void, mem, ptr};
use windows_sys::{Win32::{
    Foundation::*,
    Graphics::Gdi::{GetStockObject, WHITE_BRUSH},
    UI::WindowsAndMessaging::*,
}, core::PCWSTR};

pub type MessageCallback<T> = fn(&mut BaseWindow<T>, HWND, u32, WPARAM, LPARAM) -> LRESULT;

pub fn to_wstring(str: &str) -> Vec<u16> {
    str.encode_utf16().chain(Some(0)).collect()
}

pub struct BaseWindow<T> {
    hwnd: HWND,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    title: String,
    class_name: String,
    message_map: HashMap<u32, MessageCallback<T>>,
    win_proc : MessageCallback<T>,
    content: T,
}

impl<T> BaseWindow<T> {
    pub fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        class_name: &str,
        title: &str,
        content: T,
    ) -> Self {
        Self {
            hwnd: 0,
            x,
            y,
            width,
            height,
            title: title.to_string(),
            class_name: class_name.to_string(),
            message_map: HashMap::new(),
            win_proc : Self::def_win_proc,
            content,
        }
    }

    pub unsafe fn default_wnd_class(&self, proc : WNDPROC, class_name : PCWSTR) -> WNDCLASSW {
        WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: proc,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0,
            hIcon: LoadIconW(0, IDI_APPLICATION),
            hCursor: LoadCursorW(0, IDC_ARROW),
            hbrBackground: GetStockObject(WHITE_BRUSH) as isize,
            lpszMenuName: ptr::null_mut(),
            lpszClassName: class_name,
        }
    }

    pub fn create_window(&mut self, wnd_class : Option<WNDCLASSW>) -> bool {
        unsafe {
            let name = to_wstring(&self.class_name);
            let mut winc = self.default_wnd_class(Some(Self::win_proc_static), name.as_ptr());

            if let Some(winc_in) = wnd_class {
                winc = winc_in;
            }

            if RegisterClassW(&winc) > 0 {
                self.hwnd = CreateWindowExW(
                    0,
                    name.as_ptr(),
                    to_wstring(&self.title).as_ptr(),
                    WS_OVERLAPPEDWINDOW,
                    self.x,
                    self.y,
                    self.width,
                    self.height,
                    0,
                    0,
                    0,
                    self as *mut Self as *const c_void,
                );

                if self.hwnd != 0 {
                    return true;
                }
            }
            false
        }
    }

    pub fn get_hwnd(&self) -> HWND {
        self.hwnd
    }
    fn set_hwnd(&mut self, hwnd: HWND) {
        self.hwnd = hwnd;
    }

    pub fn set_win_proc(&mut self, proc: MessageCallback<T>) {
        self.win_proc = proc;
    }

    pub fn get_content_mut(&mut self) -> &mut T {
        &mut self.content
    }
    pub fn add_message_map(&mut self, msg: u32, callback: MessageCallback<T>) {
        self.message_map.insert(msg, callback);
    }
    pub fn clear_message_map(&mut self) {
        self.message_map.clear();
    }

    pub fn def_win_proc(&mut self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match msg {
                WM_DESTROY => {
                    PostQuitMessage(0);
                }
                _ => {
                    if let Some(callback) = self.message_map.get(&msg) {
                        // callbackが0以外の場合DefWindowProcWをスキップする
                        if callback(self, hwnd, msg, wparam, lparam) != 0 {
                            return 0;
                        }
                    }
                }
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }

    unsafe extern "system" fn win_proc_static(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == WM_NCCREATE {
            let cs = lparam as *const CREATESTRUCTW;
            let this = (*cs).lpCreateParams as *mut Self;

            (*this).set_hwnd(hwnd);
            #[cfg(target_pointer_width = "64")]
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, this as isize);
            #[cfg(target_pointer_width = "32")]
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, this as i32);
        } else {
            let this = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Self;

            if !this.is_null() {
                return ((*this).win_proc)(&mut (*this), hwnd, msg, wparam, lparam);
            }
        }

        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}
