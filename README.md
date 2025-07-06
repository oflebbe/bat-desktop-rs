# Bat Dectector Analysis in Rust
Part of a series of apps in different programming languages.

Please check www.oflebbe.de/presentations/2025

Sample Data needed can be downloaded there as well

## Note
The program be defaults assumes stereo recording with raw data files at 250kHz Sampling rate (two 2byte little endian).
It does spectral analysis for both chanels and crosscorrelation in the lower part.

## Cross Compiling from Linux

Windows
Fedora: dnf install  mingw64-gcc
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

Apple / M1
podman run --rm --volume "${PWD}":/root/src:Z --workdir /root/src joseluisq/rust-linux-darwin-builder:1.83.0  sh -c "cargo build --release --target aarch64-apple-darwin"

