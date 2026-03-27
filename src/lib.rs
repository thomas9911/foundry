use wit_bindgen::generate;

use crate::exports::example::example::foundry::Guest;
use wasmtime::{Config, Engine, Result, Strategy};

generate!({ generate_all });

struct Foundry;

impl Guest for Foundry {
    fn doit(target_triple: String) -> Result<String, String> {

        let mut config = Config::new();
        // config.strategy(Strategy::Winch);
        config.strategy(Strategy::Cranelift);
        if let Err(error) = config.target(&target_triple) {
        // if let Err(error) = config.target("x86_64-pc-windows") {
        // if let Err(error) = config.target("x86_64") {
            eprintln!(
                "this Wasmtime was not built with the x86_64 Cranelift backend \
             enabled: {error:?}",
            );
            return Err(String::from("error"));
        }

        // Create an `Engine` with that configuration.
        let engine = match Engine::new(&config) {
            Ok(engine) => engine,
            Err(error) => {
                println!("Wasmtime build is incompatible with config: {error:?}");
                return Err(String::from("error"));
            }
        };

        // Pre-compile a Wasm program.
        //
        // Note that passing the Wasm text format, like we are doing here, is only
        // supported when the `wat` cargo feature is enabled.
        let precompiled = engine.precompile_module(
            r#"
            (module
              (func (export "add") (param i32 i32) (result i32)
                (i32.add (local.get 0) (local.get 1))
              )
            )
        "#
            .as_bytes(),
        ).map_err(|e| e.to_string())?;

        std::fs::write("add.cwasm", &precompiled).map_err(|e| e.to_string())?;

        Ok(String::from("hallo"))
    }
}

export!(Foundry);
