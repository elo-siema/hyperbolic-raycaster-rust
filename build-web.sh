#!/bin/bash
export EMMAKEN_CFLAGS="-s USE_SDL=2"
cargo build --target wasm32-unknown-emscripten
cp target/wasm32-unknown-emscripten/debug/raycaster.js -t html
cp target/wasm32-unknown-emscripten/debug/raycaster.wasm -t html