default:
  @just --choose

build:
    cargo build --release --target wasm32-wasip2

doit:
    @just --justfile ./component/Justfile build
    wasmtime run --dir .::. --invoke 'doit("x86_64-pc-windows")' ./target/wasm32-wasip2/release/foundry.wasm

runit:
    wasmtime run --allow-precompiled --invoke 'add(9,12)' add.cwasm

all: build doit runit

clean:
    rm component/component.wasm
    rm add.cwasm
