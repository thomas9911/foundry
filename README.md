Asking the age old question of can a wasm component compile a wasm component? (to native byte code)

copied a lot of code from: https://docs.wasmtime.dev/examples-pre-compiling-wasm.html

just run:
```
just build
just doit
just runit
```

or 

```
just all
```

You probably have to switch the target triple to your own (because i used stinky windows):

```
rustup target list --installed
```

then pick the not wasm one and put it as the argument of the wasm component invoke that is in the Justfile doit command


# NEW

why compile a small boring component when you can compile yourself!

```
just all2
```

# NEW NEW

why compile using a normal wasm component. You can build yourself using yourself to build yourself!

```
just all2
just me2
```
