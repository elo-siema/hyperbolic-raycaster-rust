#!/bin/bash
#export EMMAKEN_CFLAGS="-s USE_SDL=2"
cargo build --target wasm32-unknown-emscripten --verbose
cp target/wasm32-unknown-emscripten/debug/hyperbolic-raycaster.js -t html
cp target/wasm32-unknown-emscripten/debug/hyperbolic_raycaster.wasm -t html
cp target/wasm32-unknown-emscripten/debug/hyperbolic_raycaster.wasm.map -t html