fn main() {
    build_fathom();
}

fn build_fathom() {
    println!("cargo:rerun-if-changed=src/engine/tablebases/fathom/src");

    cc::Build::new()
        .include("src/engine/tablebases/fathom/src")
        .file("src/engine/tablebases/fathom/src/tbprobe.c")
        .compile("fathom");
}
