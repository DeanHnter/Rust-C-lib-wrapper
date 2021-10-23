use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=lib");

    // pkg_config::Config::new()
    //.atleast_version("1.2") use to check if particular commands/envs exist
    //.probe("clang")
    //.unwrap();

    let src = [
        "include/main.c",
    ];


    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("include")
        .flag("-Wno-unused-parameter")
        .define("USE_ZLIB", None);

        build.compile("lib");
}
