extern crate codegen;
extern crate parity_wasm;
extern crate wasmi;

use module::Module;
use std::error::Error;

pub fn generate_rust(module: Module) -> Result<String, Box<Error>> {
    let mut scope = codegen::Scope::new();

    generate_prelude(&mut scope);

    let raw_module = module.to_rust();
    scope.raw(&raw_module[..]);

    let res = format!(
        "extern crate wasmi;\nextern crate parity_wasm;\nextern crate ruster;\n{}",
        scope.to_string()
    );

    Ok(res)
}

fn generate_prelude(scope: &mut codegen::Scope) {
    scope.import("wasmi", "NopExternals");
    scope.import("wasmi", "RuntimeValue");
    scope.import("wasmi", "ModuleRef");
    scope.import("wasmi", "ModuleInstance");
    scope.import("wasmi", "ImportsBuilder");
    scope.import("std", "error");
}
