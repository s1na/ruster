extern crate parity_wasm;
extern crate wasmi;

use error::Error;
use function::Function;
use global::Global;
use module::Module;
use parity_wasm::elements::{External, FunctionType, Internal, Type};
use std::error;

pub struct Interpreter {
    pwasm_module: parity_wasm::elements::Module,
}

impl Interpreter {
    pub fn new(filename: &str) -> Result<Interpreter, Box<error::Error>> {
        let module = parity_wasm::deserialize_file(&filename)?;
        let module = module.parse_names().expect("Names to be parsed");

        Ok(Interpreter {
            pwasm_module: module,
        })
    }

    pub fn create_module(&self) -> Result<Module, Box<error::Error>> {
        let name = self.get_module_name();
        let exports = self.get_exports();
        let (fn_imports, global_imports) = self.get_imported_lens();
        let mut fns: Vec<Function> = vec![];
        let mut globals: Vec<Global> = vec![];

        for export in exports.iter() {
            match export.internal() {
                &Internal::Function(r) => {
                    fns.push(self.get_exported_fn(export.field(), r - fn_imports))
                }
                &Internal::Global(r) => {
                    globals.push(self.get_exported_global(export.field(), r - global_imports))
                }
                _ => return Err(Box::from(Error::new("unexpected export"))),
            }
        }

        Ok(Module::new(name, fns, globals))
    }

    fn get_module_name(&self) -> Option<String> {
        let names_section = self.pwasm_module.names_section()?;

        match names_section {
            parity_wasm::elements::NameSection::Module(mns) => Some(mns.name().to_string()),
            _ => None,
        }
    }

    fn get_exports(&self) -> &[parity_wasm::elements::ExportEntry] {
        self.pwasm_module
            .export_section()
            .map(|s| s.entries())
            .unwrap_or(&[])
    }

    fn get_exported_global(&self, name: &str, i: u32) -> Global {
        let globals = self.pwasm_module
            .global_section()
            .map(|s| s.entries())
            .unwrap_or(&[]);

        let global_type = globals[i as usize].global_type().content_type();

        Global::new(name, global_type)
    }

    fn get_exported_fn(&self, name: &str, i: u32) -> Function {
        // It's a section with function declarations (which are references to the type section entries)
        let functions = self.pwasm_module
            .function_section()
            .map(|s| s.entries())
            .unwrap_or(&[]);

        // Type section stores function types which are referenced by function_section entries
        let type_section = self.pwasm_module
            .type_section()
            .expect("No type section found");

        // Getting a type reference from a function section entry
        let func_type_ref = functions[i as usize].type_ref();

        // Use the reference to get an actual function type
        let function_type: &FunctionType = match &type_section.types()[func_type_ref as usize] {
            &Type::Function(ref func_type) => func_type,
        };

        // Parses arguments and constructs runtime values in correspondence of their types
        let arg_types = Vec::from(function_type.params());
        let return_type = function_type.return_type().unwrap();

        Function {
            name: name.to_string(),
            arg_types,
            return_type,
        }
    }

    fn get_imported_lens(&self) -> (u32, u32) {
        // We need to count import section entries (functions only!) to subtract it from function_index
        // and obtain the index within the function section
        match self.pwasm_module.import_section() {
            Some(import) => import
                .entries()
                .iter()
                .map(|entry| match entry.external() {
                    &External::Function(_) => (1, 0),
                    &External::Global(_) => (0, 1),
                    _ => (0, 0),
                })
                .fold((0, 0), |acc, (f, g)| (acc.0 + f, acc.1 + g)),
            None => return (0, 0),
        }
    }
}
