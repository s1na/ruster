extern crate codegen;
extern crate parity_wasm;
extern crate wasmi;

use module::{Function, Module};
use parity_wasm::elements::ValueType;
use std::error::Error;

pub fn generate_rust(module: Module) -> Result<String, Box<Error>> {
    let mut scope = codegen::Scope::new();
    generate_prelude(&mut scope);
    generate_error_type(&mut scope);
    let mut mod_impl = generate_module(&mut scope);

    for f in module.get_exported_fns().iter() {
        let fn_def = generate_fn(f);
        mod_impl.push_fn(fn_def);
    }

    scope.push_impl(mod_impl);

    let res = format!(
        "extern crate wasmi;\nextern crate parity_wasm;\n{}",
        scope.to_string()
    );

    Ok(res)
}

fn generate_fn(f: &Function) -> codegen::Function {
    let return_type = get_type(f.return_type, true);
    let arg_types = f.arg_types.iter().map(|t| get_type(*t, false));
    let block = generate_block(f);

    let mut fn_def = codegen::Function::new(&f.name[..]);
    fn_def.vis("pub");
    fn_def.ret(return_type);
    fn_def.arg_ref_self();

    for (i, t) in arg_types.enumerate() {
        fn_def.arg(&format!("a{}", i)[..], t);
    }

    fn_def.push_block(block);

    fn_def
}

fn generate_block(f: &Function) -> codegen::Block {
    let mut b = codegen::Block::new("");

    let mut params = Vec::new();

    for (i, t) in f.arg_types.iter().enumerate() {
        let t_str = match t {
            ValueType::I32 => "I32",
            ValueType::I64 => "I64",
            ValueType::F32 => "F32",
            ValueType::F64 => "F64",
        };

        params.push(format!("RuntimeValue::{}(a{})", t_str, i));
    }

    let l = format!("let params = [{}];", params.join(", "));
    b.line(l);

    let invoke = format!(
        "
        self.instance.invoke_export(\"{}\", &params, &mut NopExternals)?
        .ok_or(Box::from(Error::new(\"returned value is empty\")))
        .and_then(|v| match v {{
            RuntimeValue::{:?}(t) => Ok(t),
            _ => Err(Box::from(Error::new(\"returned value has invalid type\")))
        }})",
        f.name, f.return_type
    );

    b.line(invoke);

    b
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

fn get_type(t: ValueType, result: bool) -> codegen::Type {
    let mut template = String::from("{}");
    if result {
        template = String::from("Result<{}, Box<error::Error>>")
    }

    match t {
        ValueType::I32 => codegen::Type::from(template.replace("{}", "i32")),
        ValueType::I64 => codegen::Type::from(template.replace("{}", "i64")),
        ValueType::F32 => codegen::Type::from(template.replace("{}", "f32")),
        ValueType::F64 => codegen::Type::from(template.replace("{}", "f64")),
    }
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
