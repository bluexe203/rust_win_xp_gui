#![windows_subsystem = "windows"]
use std::{mem, ptr};

use rust_win_xp_gui::base_window::*;
use windows_sys::{Win32::{UI::WindowsAndMessaging::*, Graphics::Gdi::*, Foundation::*}};

struct TestStruct {
    pub param1: u32,
    pub param2: u32,
    pub param3: u32,
}

fn main() {
    unsafe {
        let mut test = TestStruct {
            param1: 0,
            param2: 0,
            param3: 0,
        };
        let mut window = BaseWindow::new(0, 0, 640, 480, "base_window_class", "test", test);
        if window.create_window() {
            // メッセージマップの登録(WM_PAINT, WM_LBUTTONUP)
            window.add_message_map(WM_PAINT, on_paint);
            window.add_message_map(WM_LBUTTONUP, on_lbutton_up);

            let hwnd = window.get_hwnd();
            ShowWindow(hwnd, SW_NORMAL);
            UpdateWindow(hwnd);
            let mut msg = mem::zeroed::<MSG>();
            loop {
                if GetMessageW(&mut msg, 0, 0, 0) == 0 {
                    return;
                }
                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);
            }
        }
    }
}

fn on_paint(
    window: &mut BaseWindow<TestStruct>,
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> bool {
    let mut content = window.get_content_mut();
    unsafe {
        let mut ps = mem::zeroed::<PAINTSTRUCT>();
        let hdc = BeginPaint(hwnd , &mut ps);
        let str = format!("ウィンドウをクリックしてください。クリック回数={}", &content.param1);
        let wstr = to_wstring(&str);
        TextOutW(hdc, 100, 100, wstr.as_ptr(), wstr.len() as i32 - 1);
        EndPaint(hwnd , &mut ps);
    }
    false // falseの場合DefWindowProcWを呼び出す。
}

fn on_lbutton_up(
    window: &mut BaseWindow<TestStruct>,
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> bool {
    let mut content = window.get_content_mut();
    content.param1 = content.param1 + 1;
    unsafe {
        let str = format!("Call on_lbutton_up {}", &content.param1);
        MessageBoxW(
            hwnd,
            to_wstring(&str).as_ptr(),
            to_wstring("title").as_ptr(),
            MB_OK,
        );
        InvalidateRect(hwnd, ptr::null_mut(), 0);
    }
    false // falseの場合DefWindowProcWを呼び出す。
}