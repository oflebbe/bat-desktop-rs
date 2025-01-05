fn main() {
    cc::Build::new()
        .file("src/meow.c")
        .compile("meow_fft");
    println!("cargo::rerun-if-changed=src/meow.c");
}