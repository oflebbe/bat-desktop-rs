#[cfg(feature = "meow")]
fn main() {
    println!("cargo::rerun-if-changed=src/meow.c");
    cc::Build::new()
        .file("src/meow.c")
        .compile("meow");
}

#[cfg(not(feature = "meow"))]
fn main() {}