extern crate wabt;
extern crate wasmi;

use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

fn main() {
    let wasm_binary: Vec<u8> = wabt::wat2wasm(
        r#"
            (module
                (func (export "test") (result i32)
                    i32.const 1337
                )
            )
            "#,
    ).expect("failed to parse wat");

    let module = wasmi::Module::from_buffer(&wasm_binary).expect("failed to load wasm");

    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    assert_eq!(
        instance
            .invoke_export("test", &[], &mut NopExternals)
            .expect("failed to execute export"),
        Some(RuntimeValue::I32(1337)),
    );
}
