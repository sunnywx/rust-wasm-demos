#!/bin/bash

# cargo install wasm-gc

cargo build --target wasm32-unknown-unknown

#wasm-gc target/wasm32-unknown-unknown/debug/begin_wasm.wasm app.wasm

#cargo install https

http
