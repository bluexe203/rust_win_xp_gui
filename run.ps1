rustup default 1.49
$env:CARGO_BUILD_RUSTFLAGS = '-C target-feature=+crt-static -C link-arg=/SUBSYSTEM:WINDOWS,5.01'
cargo run --example simple --release --target=i686-pc-windows-msvc
rustup default stable
