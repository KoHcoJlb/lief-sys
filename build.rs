fn main() -> miette::Result<()> {
    let mut cmake_config = cmake::Config::new("LIEF");
    for opt in [
        "LIEF_EXAMPLES",
        "LIEF_C_API",
        "LIEF_PE",
        "LIEF_MACHO",
        "LIEF_DEX",
        "LIEF_ART"
    ] {
        cmake_config.define(opt, "OFF");
    }
    let lief_path = cmake_config.build();

    cxx_build::bridge("src/lib.rs")
        .include("src")
        .include(lief_path.join("include"))
        .std("c++20")
        .compile("lief-sys");

    println!("cargo::rerun-if-changed=src/lib.hpp");
    println!("cargo::rerun-if-changed=src/lib.rs");

    println!("cargo::rustc-link-search={}", lief_path.join("lib").to_str().unwrap());
    println!("cargo::rustc-link-lib=LIEF");

    Ok(())
}
