extern crate codegen;
extern crate parity_wasm;
extern crate wasmi;

use module::Module;
use parity_wasm::elements::ValueType;
use std::error::Error;

pub fn generate_rust(module: Module) -> Result<String, Box<Error>> {
    let mut scope = codegen::Scope::new();
    generate_prelude(&mut scope);
    generate_error_type(&mut scope);
    let mut mod_impl = generate_module(&mut scope);

    for f in module.get_exported_fns().iter() {
        let fn_def = f.to_rust();
        mod_impl.push_fn(fn_def);
    }

    scope.push_impl(mod_impl);

    let res = format!(
        "extern crate wasmi;\nextern crate parity_wasm;\n{}",
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

fn generate_module(scope: &mut codegen::Scope) -> codegen::Impl {
    let mut mod_struct = codegen::Struct::new("Module");
    mod_struct.vis("pub");
    mod_struct.field("instance", "ModuleRef");
    scope.push_struct(mod_struct);

    let mut mod_impl = codegen::Impl::new("Module");
    let mut mod_new = codegen::Function::new("new");
    mod_new.vis("pub");
    mod_new.arg("filename", "&str");
    mod_new.ret("Result<Module, Box<error::Error>>");
    let mut mod_new_block = codegen::Block::new("");
    mod_new_block.line(
        "
        let module = parity_wasm::deserialize_file(&filename)?;
        let module = wasmi::Module::from_parity_wasm_module(module)?;
        let instance = ModuleInstance::new(&module, &ImportsBuilder::default())?
            .assert_no_start();

        Ok(Module { instance })
        ",
    );
    mod_new.push_block(mod_new_block);
    mod_impl.push_fn(mod_new);

    mod_impl
}

fn generate_error_type(scope: &mut codegen::Scope) {
    let code = "
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    msg: String
}

impl Error {
    fn new(m: &str) -> Error {
        Error{msg: m.to_string()}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, \"{}\", self.msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
";
    scope.raw(code);
}
