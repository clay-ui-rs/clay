# Clay Rust Bindings

Rust bindings for [Clay](https://github.com/nicbarker/clay), a UI layout library written in C.


## Support

**O - In Progress, X - Done**

- (O) Elements
    - (X) Rectangle
    - (O) Text
    - (X) Image
    - (X) Floating Container
    - (X) Border Container
    - (X) Scroll Container
    - (X) Custom Elements
- (X) Text Measuring
- (X) Element Ids
- (O) Interactions
- ( ) Debug Tools
- ( ) Render Commands
- (O) Full Test Coverrage
- ( ) Examples

## Build bindings

To build bindings you need to use the `generate_bindings` script. \
It needs `bindgen` installed as a CLI, you can install it with `cargo install bindgen`. \
Calling it will use the `clay.h` in the project root, or any `clay.h` file provided with `CLAY_HEADER_PATH`. \
Using the clay header it will generate `src/bindings/bindings.rs` and `src/bindings/bindings_debug.rs`.
