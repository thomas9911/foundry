default:
  @just --choose

build:
    cargo build --release --target wasm32-wasip2

component:
    @just --justfile ./component/Justfile build

doit:
    @just component
    wasmtime run --dir .::. --invoke 'doit("x86_64-pc-linux", "./component/component.wasm", "./add.cwasm")' ./target/wasm32-wasip2/release/foundry.wasm

me:
    wasmtime run --dir .::. --invoke 'doit("x86_64-pc-linux", "./target/wasm32-wasip2/release/foundry.wasm", "./me.cwasm")' ./target/wasm32-wasip2/release/foundry.wasm

doit2:
    @just component
    wasmtime run --allow-precompiled --dir .::. --invoke 'doit("x86_64-pc-linux", "./component/component.wasm", "./add.cwasm")' ./me.cwasm

runit:
    wasmtime run --allow-precompiled --invoke 'add(9,12)' add.cwasm

me2:
    wasmtime run --allow-precompiled --dir .::. --invoke 'doit("x86_64-pc-linux", "./target/wasm32-wasip2/release/foundry.wasm", "./me2.cwasm")' ./me.cwasm

all: build doit runit
all2: build me doit2 runit

clean:
    rm -f component/component.wasm
    rm -f add.cwasm
    rm -f me.cwasm
    rm -f me2.cwasm
