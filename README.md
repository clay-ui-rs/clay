[![Build Status](https://github.com/clay-ui-rs/clay/actions/workflows/ci.yaml/badge.svg)](https://github.com/clay-ui-rs/clay/actions?workflow=Rust%20CI)
[![Crates.io](https://img.shields.io/crates/v/clay-layout.svg)](https://crates.io/crates/clay-layout)
[![Documentation](https://docs.rs/clay-layout/badge.svg)](https://docs.rs/clay-layout)

# Clay Rust Bindings

Rust bindings for [Clay](https://github.com/nicbarker/clay), a UI layout library written in C.

## Support

**O - In Progress, X - Done**

- (O) Elements
  - (X) Rectangle
  - (O) Text (Waiting on an update of memory handling for text on clay part)
  - (X) Image
  - (X) Floating Container
  - (X) Border Container
  - (X) Scroll Container
  - (X) Custom Elements
- (X) Text Measuring
- (X) Element Ids
- (X) Interactions
- (X) Debug Tools
- (X) Render Commands
- (O) Full Test Coverrage
- ( ) Examples

## Build bindings

To build bindings you need to use the `generate_bindings` script. \
It needs `bindgen` installed as a CLI, you can install it with `cargo install bindgen`. \
Calling it will use the `clay.h` in the project root, or any `clay.h` file provided with `CLAY_HEADER_PATH`. \
Using the clay header it will generate `src/bindings/bindings.rs` and `src/bindings/bindings_debug.rs`.

## Examples

Examples can be found in the `examples/` directory. They can be ran using `cargo`:

```sh
cargo run --example basic_rectangle
cargo run --example raylib_renderer --features raylib-renderer
```
