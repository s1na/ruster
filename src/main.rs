extern crate wabt;
extern crate wasmi;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use wasmi::{ImportsBuilder, ModuleInstance, ModuleRef, NopExternals, RuntimeValue};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let path = &args[1];
    let mut f = File::open(path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("reading file failed");

    let module = new_module(&contents[..]);
    let res = add(module, 3, 5);

    println!("Result of add: {:?}", res);
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

fn new_module(wat: &str) -> ModuleRef {
    let wasm_binary: Vec<u8> = wabt::wat2wasm(wat).expect("failed to parse wat");

    let module = wasmi::Module::from_buffer(&wasm_binary).expect("failed to load wasm");

    ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .assert_no_start()
}
