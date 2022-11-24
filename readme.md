# rust_win_xp_gui

Create Rust Window Application for WindowsXP



## Setup Toolchain for WindowsXP

```powershell
rustup install 1.49
rustup default 1.49
rustup target install i686-pc-windows-msvc
```



## Run Example

Clone this repositry

```powershell
.\run.ps1
```

run.ps1

```powershell
rustup default 1.49
$env:CARGO_BUILD_RUSTFLAGS = '-C target-feature=+crt-static -C link-arg=/SUBSYSTEM:WINDOWS,5.01'
cargo run --example simple --release --target=i686-pc-windows-msvc
rustup default stable
```


