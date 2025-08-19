#!/bin/bash


cargo build --target wasm32-wasip1

wasmtime --dir tmp::/ target/wasm32-wasip1/debug/wasi-sql-error.wasm
