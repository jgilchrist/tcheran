use std::path::PathBuf;

fn main() {
    build_fathom();
}

fn build_fathom() {
    let fathom_dir = PathBuf::from("src/engine/tablebases/fathom/src")
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    println!("cargo:rerun-if-changed={fathom_dir}");

    let mut cc = cc::Build::new();
    cc.file(format!("{fathom_dir}/tbprobe.c"));
    cc.include(fathom_dir);

    cc.compile("fathom");
}
