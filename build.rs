extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/streams.c")
        .compile("libstream.a");
    println!("cargo:rustc-link-lib=mpdclient")
}