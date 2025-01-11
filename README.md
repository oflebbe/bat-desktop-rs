Cross Compiling from Linux

Windows
Fedora: dnf install  mingw64-gcc
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

Apple / M1
podman run --rm --volume "${PWD}":/root/src:Z --workdir /root/src joseluisq/rust-linux-darwin-builder:1.83.0  sh -c "cargo build --release --target aarch64-apple-darwin"

