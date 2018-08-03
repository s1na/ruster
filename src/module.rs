extern crate codegen;
extern crate parity_wasm;
extern crate wasmi;

use function::Function;
use global::Global;

pub struct Module {
    name: Option<String>,
    fns: Vec<Function>,
    globals: Vec<Global>,
}

impl Module {
    pub fn new(name: Option<String>, fns: Vec<Function>, globals: Vec<Global>) -> Module {
        Module {
            name: name,
            fns,
            globals,
        }
    }

    pub fn to_rust(&self) -> String {
        let mut scope = codegen::Scope::new();

        let name = self.name.clone().unwrap_or("Module".to_string());
        let mut mod_struct = codegen::Struct::new(&name[..]);

        mod_struct.vis("pub");
        mod_struct.field("instance", "ModuleRef");
        scope.push_struct(mod_struct);

        let module_impl = self.gen_impl();
        scope.push_impl(module_impl);

        scope.to_string()
    }

    fn gen_impl(&self) -> codegen::Impl {
        let mut mod_impl = codegen::Impl::new("Module");
        mod_impl.push_fn(self.gen_new());

        for f in self.fns.iter() {
            let fn_def = f.to_rust();
            mod_impl.push_fn(fn_def);
        }

        for g in self.globals.iter() {
            let fn_def = g.to_rust();
            mod_impl.push_fn(fn_def);
        }

        mod_impl
    }

    fn gen_new(&self) -> codegen::Function {
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

        mod_new
    }
}
