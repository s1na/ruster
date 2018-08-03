extern crate codegen;

use itertools::Itertools;
use parity_wasm::elements::ValueType;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub arg_types: Vec<ValueType>,
    pub return_type: ValueType,
}

impl Function {
    pub fn to_rust(&self) -> codegen::Function {
        let mut fn_def = codegen::Function::new(&self.name[..]);

        fn_def.vis("pub");

        let return_type = self.gen_return_type();
        fn_def.ret(return_type);

        fn_def.arg_ref_self();
        let arg_types = self.arg_types
            .iter()
            .map(|t| codegen::Type::from(format!("{}", t)));
        for (i, t) in arg_types.enumerate() {
            fn_def.arg(&format!("a{}", i)[..], t);
        }

        let block = self.gen_block();
        fn_def.push_block(block);

        fn_def
    }

    fn gen_return_type(&self) -> codegen::Type {
        codegen::Type::from(format!("Result<{}, Box<error::Error>>", self.return_type))
    }

    fn gen_block(&self) -> codegen::Block {
        let mut b = codegen::Block::new("");

        let params = self.arg_types
            .iter()
            .map(|t| format!("{}", t).to_ascii_uppercase())
            .enumerate()
            .map(|(i, t)| format!("RuntimeValue::{}(a{})", t, i))
            .join(", ");
        let l = format!("let params = [{}];", params);
        b.line(l);

        let invoke = format!(
            "
            self.instance.invoke_export(\"{}\", &params, &mut NopExternals)?
            .ok_or(Box::from(Error::new(\"returned value is empty\")))
            .and_then(|v| match v {{
                RuntimeValue::{:?}(t) => Ok(t),
                _ => Err(Box::from(Error::new(\"returned value has invalid type\")))
            }})",
            self.name, self.return_type
        );

        b.line(invoke);

        b
    }
}
