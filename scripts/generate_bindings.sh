#! /bin/sh

if ! command -v bindgen 2>&1 >/dev/null; then
    echo "bindgen not found. Install it using `cargo install bindgen-cli`"
    exit 1
fi

: "${CLAY_HEADER_PATH:=clay.h}"

COMMON_BINDGEN_FLAGS="--use-core --blocklist-function qecvt --blocklist-function qgcvt --blocklist-function qecvt_r --blocklist-function qfcvt --blocklist-function qfcvt_r --blocklist-function strtold"

bindgen $CLAY_HEADER_PATH -o src/bindings/bindings.rs $COMMON_BINDGEN_FLAGS 
echo "Generated src/bindings/bindings.rs"
bindgen $CLAY_HEADER_PATH -o src/bindings/bindings_debug.rs $COMMON_BINDGEN_FLAGS -- -DCLAY_DEBUG
echo "Generated src/bindings/bindings_debug.rs"
