extern crate codegen;
extern crate parity_wasm;
extern crate wabt;
extern crate wasmi;

mod generate;
mod extract;

use std::error::Error;
use self::generate::generate_rust;
use self::extract::{get_exported_fns};
use wasmi::{ImportsBuilder, ModuleInstance, ModuleRef, NopExternals, RuntimeValue};

pub fn run(filename: &str) -> Result<(), Box<Error>> {
    let module = parity_wasm::deserialize_file(&filename).expect("File to be deserialized");
    let module = module.parse_names().expect("Names to be parsed");
    let exported_fns = get_exported_fns(&module);
    println!("Exported functions {:?}", exported_fns);
    generate_rust(exported_fns);

    let module = new_module(module);
    let res = add(module, 3, 5);
    println!("Result of add: {:?}", res);

    Ok(())
}

fn add(instance: ModuleRef, a: i32, b: i32) -> Result<i32, &'static str> {
    let params = [RuntimeValue::I32(a), RuntimeValue::I32(b)];

    match instance.invoke_export("add", &params, &mut NopExternals) {
        Ok(r) => match r {
            Some(runtime_value) => match runtime_value {
                RuntimeValue::I32(v) => Ok(v),
                _ => Err("invalid type"),
            },
            None => Err("emtpy result"),
        },
        Err(_) => Err("wasmi error"),
    }
}

fn new_module(module: parity_wasm::elements::Module) -> ModuleRef {
    let module = wasmi::Module::from_parity_wasm_module(module).expect("failed to load wasm");

    ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .assert_no_start()
}
