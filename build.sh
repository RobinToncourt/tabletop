#!/bin/bash

set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/tabletop.wasm
OUTPUT=www/tabletop_bg.wasm

echo "Building to $TARGET..."
cargo build --target $TARGET --release

echo "Create www folder..."
mkdir -p www

echo "Generate wasm bindgen..."
/home/robin/.cargo/bin/wasm-bindgen --no-typescript --target web --out-dir ./www/ --out-name "tabletop" $BINARY

echo "Striping..."
~/Tools/wabt/bin/wasm-strip $OUTPUT

echo "Optimizing..."
~/Tools/binaryen/bin/wasm-opt -Oz --enable-bulk-memory --enable-nontrapping-float-to-int --enable-reference-types -o www/tabletop_bg.wasm $OUTPUT

echo "Compressing..."
gzip --keep $OUTPUT

echo "Done!"
