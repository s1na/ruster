extern crate codegen;
extern crate parity_wasm;

use extract::Function;
use parity_wasm::elements::ValueType;

pub fn generate_rust(fns: Vec<Function>) {
    //let mut scope = Scope::new();

    let f = &fns[0];
    let return_type = match f.return_type {
        ValueType::I32 => codegen::Type::from("i32"),
        _ => panic!("Non-i32 type"),
    };
    let mut fn_def = codegen::Function::new(&f.name[..]);
    fn_def.vis("pub");
    fn_def.ret(return_type);
    let mut fn_def_str = String::new();
    fn_def
        .fmt(false, &mut codegen::Formatter::new(&mut fn_def_str))
        .unwrap();

    // scope.raw(fn_def.to_string());

    println!("{:?}", fn_def_str);
}
