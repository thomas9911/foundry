Asking the age old question of can a wasm component compile a wasm component? (to native byte code)

copied a lot of code from: https://docs.wasmtime.dev/examples-pre-compiling-wasm.html

just run:
```
just build
just doit
just runit
```

You probably have to switch the target triple to your own (because i used stinky windows):

```
rustup target list --installed
```

then pick the not wasm one

