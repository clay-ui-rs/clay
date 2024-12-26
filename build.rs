fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "windows" {
        cc::Build::new()
            .file("build.cpp")
            .std("c++20")
            .compile("clay");
    } else {
        cc::Build::new()
            .file("build.c")
            .extra_warnings(false)
            .compile("clay");
    }
}
