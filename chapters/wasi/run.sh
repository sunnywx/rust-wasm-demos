#!/bin/bash

rustc hello.rs --target wasm32-wasi
wasmtime hello.wasm
