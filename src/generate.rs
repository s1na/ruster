extern crate codegen;
extern crate parity_wasm;
extern crate wasmi;

use module::Function;
use parity_wasm::elements::ValueType;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn generate_rust(fns: Vec<Function>, out_path: &str) {
    let mut scope = codegen::Scope::new();

    scope.import("wasmi", "NopExternals");
    scope.import("wasmi", "RuntimeValue");
    scope.import("wasmi", "ModuleRef");
    scope.import("wasmi", "ModuleInstance");
    scope.import("wasmi", "ImportsBuilder");
    scope.import("std::error", "Error");

    let mut mod_struct = codegen::Struct::new("Module");
    mod_struct.vis("pub");
    mod_struct.field("instance", "ModuleRef");
    let scope = scope.push_struct(mod_struct);

    let mut mod_impl = codegen::Impl::new("Module");
    let mut mod_new = codegen::Function::new("new");
    mod_new.vis("pub");
    mod_new.arg("filename", "&str");
    mod_new.ret("Result<Module, Box<Error>>");
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

    let f = &fns[0];
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

    mod_impl.push_fn(fn_def);
    scope.push_impl(mod_impl);
    /*let mut fn_def_str = String::new();
    fn_def
        .fmt(false, &mut codegen::Formatter::new(&mut fn_def_str))
        .unwrap();

    scope.raw(&fn_def_str[..]);*/

    let path = Path::new(out_path);
    let mut file = File::create(&path).expect("File to be created");
    file.write_all("extern crate wasmi;\nextern crate parity_wasm;\n".as_bytes());
    file.write_all(scope.to_string().as_bytes())
        .expect("Content to be written to file");
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
        "let res = self.instance.invoke_export(\"{}\", &params, &mut NopExternals);",
        f.name
    );

    let rest = "
        match res {
            Ok(r) => match r {
                Some(runtime_value) => match runtime_value {
                    RuntimeValue::I32(v) => Ok(v),
                    _ => Err(\"invalid type\"),
                },
                None => Err(\"emtpy result\"),
            },
            Err(_) => Err(\"wasmi error\"),
        }";

    b.line(invoke);
    b.line(rest);

    b
}

fn get_type(t: ValueType, result: bool) -> codegen::Type {
    let mut template = String::from("{}");
    if result {
        template = String::from("Result<{}, &'static str>")
    }

    match t {
        ValueType::I32 => codegen::Type::from(template.replace("{}", "i32")),
        ValueType::I64 => codegen::Type::from(template.replace("{}", "i64")),
        ValueType::F32 => codegen::Type::from(template.replace("{}", "f32")),
        ValueType::F64 => codegen::Type::from(template.replace("{}", "f64")),
    }
}
