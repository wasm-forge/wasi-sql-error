# Demo to reproduce error when creating the temporary file in WASI


## Compilation to Wasm

You will need [`Rust`](https://www.rust-lang.org/tools/install), [`WASI-SDK`](https://github.com/WebAssembly/wasi-sdk/releases), and [`wasmtime`](https://docs.wasmtime.dev/cli-install.html) installed, 

Add Wasi target:
```bash
rustup target add wasm32-wasip1
```

Set the `WASI_SDK` path to the wasi installation folder (in case it is not the default `/opt/wasi-sdk`), make sure the right `clang` is on the path:
```bash
export WASI_SDK=/opt/wasi-sdk
export PATH=$WASI_SDK/bin:$PATH
```

## Test

Compile and run using `wasmtime`:
```bash
cargo 
wasmtime --dir tmp::/ target/wasm32-wasip1/debug/wasi-sql-error.wasm
```
