extern crate codegen;
extern crate parity_wasm;
extern crate wasmi;

use module::Module;
use std::error::Error;

pub struct Generator {
    module: Module,
}

impl Generator {
    pub fn new(module: Module) -> Generator {
        Generator { module }
    }

    pub fn generate_rust(&self) -> Result<String, Box<Error>> {
        let mut scope = codegen::Scope::new();

        self.generate_prelude(&mut scope);

        let raw_module = self.module.to_rust();
        scope.raw(&raw_module[..]);

        let res = format!(
            "extern crate wasmi;\nextern crate parity_wasm;\nextern crate ruster;\n{}",
            scope.to_string()
        );

        Ok(res)
    }

    fn generate_prelude(&self, scope: &mut codegen::Scope) {
        scope.import("wasmi", "NopExternals");
        scope.import("wasmi", "RuntimeValue");
        scope.import("wasmi", "ModuleRef");
        scope.import("wasmi", "ModuleInstance");
        scope.import("wasmi", "ImportsBuilder");
        scope.import("std", "error");
    }
}
