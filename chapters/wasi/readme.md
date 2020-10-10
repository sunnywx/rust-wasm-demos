wasi (wasm system interface)
===

> run wasm outside browser

## Intro

- [wasmtime official site](https://wasmtime.dev/)

- [wasmtime github](https://github.com/bytecodealliance/wasmtime)

## Build

```shell script
rustup target add wasm32-wasi
rustc hello.rs --target wasm32-wasi

wasmtime hello.wasm
```
