# Chapters with Rust examples

## 1. Usage

### 1.1 Build with rustc

Enter directory, if there's no `Cargo.toml`, exec `rustc main.rs && ./main`

### 1.2 Build with cargo 

Enter directory, if there's `Cargo.toml`, so that dir is a package.

Check `readme.md` in each dir to build or run that package.

For lib package, often use `cargo build --target <target-name>`.

For bin package, often use `cargo run`.

To see what target names rust supports, run
```shell script
rustc --print target-list
```

or

```shell script
rustup target list
```
