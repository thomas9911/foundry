build:
    cargo build --release --target wasm32-wasip2

doit:
    wasmtime run --dir ./runner::. --invoke 'doit("x86_64-pc-windows")' ./target/wasm32-wasip2/release/foundry.wasm

runit:
    cd runner && cargo run --release
