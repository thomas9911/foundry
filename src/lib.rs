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
            eprintln!(
                "this Wasmtime was not built with the {target_triple} Cranelift backend \
             enabled, you need the all-arch feature on: {error:?}",
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
        let input = "./component/component.wasm";
        let output = "add.cwasm";

        let component_bytes = std::fs::read(input).map_err(|e| e.to_string())?;
        let precompiled = engine.precompile_component(&component_bytes).map_err(|e| e.to_string())?;

        std::fs::write(output, &precompiled).map_err(|e| e.to_string())?;

        Ok(format!("compiled {input} to {output}"))
    }
}

export!(Foundry);
