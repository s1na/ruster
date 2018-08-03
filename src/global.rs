extern crate codegen;

use parity_wasm::elements::ValueType;

pub struct Global {
    name: String,
    t: ValueType,
}

impl Global {
    pub fn new(name: &str, t: ValueType) -> Global {
        Global {
            name: name.to_string(),
            t,
        }
    }

    pub fn to_rust(&self) -> codegen::Function {
        let mut fn_def = codegen::Function::new(&self.name[..]);

        fn_def.vis("pub");

        fn_def.ret(codegen::Type::from(format!(
            "Result<{}, Box<error::Error>>",
            self.t
        )));
        fn_def.arg_ref_self();

        let block = self.gen_block();
        fn_def.push_block(block);

        fn_def
    }

    fn gen_block(&self) -> codegen::Block {
        let mut b = codegen::Block::new("");

        let invoke = format!(
            "
            self.instance.export_by_name(\"{}\")
                .ok_or(Box::from(ruster::error::Error::new(\"global not found\")))
                .and_then(|e| match e {{
                    wasmi::ExternVal::Global(r) => match r.get() {{
                        RuntimeValue::{:?}(t) => Ok(t),
                        _ => Err(Box::from(ruster::error::Error::new(\"returned value has invalid type\")))
                    }},
                    _ => Err(Box::from(ruster::error::Error::new(\"export is not global\"))),
                }})
            ",
            &self.name[..],
            self.t,
            );

        b.line(invoke);

        b
    }
}
