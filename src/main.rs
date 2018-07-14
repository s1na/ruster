extern crate wabt;
extern crate wasmi;

use wasmi::{ImportsBuilder, ModuleInstance, ModuleRef, NopExternals, RuntimeValue};

fn main() {
    let module = new_module();
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

fn new_module() -> ModuleRef {
    let wasm_binary: Vec<u8> = wabt::wat2wasm(
        r#"
            (module
                (func $add (param i32 i32) (result i32)
                    get_local 0
                    get_local 1
                    i32.add)
                (export "add" (func $add)))
         "#,
    ).expect("failed to parse wat");

    let module = wasmi::Module::from_buffer(&wasm_binary).expect("failed to load wasm");

    ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .assert_no_start()
}
