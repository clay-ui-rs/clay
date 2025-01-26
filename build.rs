fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Make sure we re-run the build script if the clay.h file changes
    println!("cargo:rerun-if-changed=clay.h");

    if target_os == "windows" {
        cc::Build::new()
            .file("build.cpp")
            .warnings(false)
            .std("c++20")
            .compile("clay");
    } else {
        cc::Build::new()
            .file("build.c")
            .warnings(false)
            .compile("clay");
    }
}
