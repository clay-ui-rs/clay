fn main() {
    cc::Build::new()
        .file("build.c")
        .extra_warnings(false)
        .compile("clay");
}
