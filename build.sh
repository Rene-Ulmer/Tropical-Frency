#!/bin/sh
set -e
OUT_PATH=target/wasm32-unknown-unknown/release/nostd.wasm

mkdir -p build

cargo build --release --target wasm32-unknown-unknown || exit 1
cp $OUT_PATH build/0.wasm

# 1st optimization level: remove dead code
wasm-gc build/0.wasm build/1.wasm

# 2nd: rename functions etc
./wasm-opt build/1.wasm -o build/2.wasm -Oz

# 3rd: put it into a png to make use of png compression
python2 tools/bin_to_png.py build/2.wasm build/3.png

ls -l build/

