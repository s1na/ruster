extern crate parity_wasm;
extern crate wasmi;

use parity_wasm::elements::{External, FunctionType, Internal, Type, ValueType};
use std::error::Error;
/*use wasmi::{ImportsBuilder, ModuleInstance, ModuleRef};*/

pub struct Module {
    pwasm_module: parity_wasm::elements::Module,
}

impl Module {
    pub fn new(filename: &str) -> Result<Module, Box<Error>> {
        let module = parity_wasm::deserialize_file(&filename)?;
        let module = module.parse_names().expect("Names to be parsed");

        Ok(Module {
            pwasm_module: module,
        })
    }

    pub fn get_module_name(&self) -> Option<&str> {
        let names_section = self.pwasm_module.names_section()?;

        match names_section {
            parity_wasm::elements::NameSection::Module(mns) => Some(mns.name()),
            _ => None,
        }
    }

    pub fn get_exports(&self) -> &[parity_wasm::elements::ExportEntry] {
        self.pwasm_module
            .export_section()
            .map(|s| s.entries())
            .unwrap_or(&[])
    }

    pub fn get_exported_fns(&self) -> Vec<Function> {
        let mut exported_fns: Vec<Function> = Vec::new();

        for export in self.get_exports().iter() {
            let name = export.field();
            let function_index: usize = match export.internal() {
                &Internal::Function(i) => i as usize,
                _ => panic!("Found non-function export"),
            };

            exported_fns.push(self.get_exported_fn(name, function_index));
        }

        exported_fns
    }

    fn get_exported_fn(&self, name: &str, i: usize) -> Function {
        let imported_fns_len = self.get_imported_fns_len();
        // It's a section with function declarations (which are references to the type section entries)
        let functions = self.pwasm_module
            .function_section()
            .map(|s| s.entries())
            .unwrap_or(&[]);

        // Type section stores function types which are referenced by function_section entries
        let type_section = self.pwasm_module
            .type_section()
            .expect("No type section found");

        // Calculates a function index within module's function section
        let function_index_in_section = i - imported_fns_len;

        // Getting a type reference from a function section entry
        let func_type_ref: usize = functions[function_index_in_section].type_ref() as usize;

        // Use the reference to get an actual function type
        let function_type: &FunctionType = match &type_section.types()[func_type_ref] {
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

    fn get_imported_fns_len(&self) -> usize {
        // We need to count import section entries (functions only!) to subtract it from function_index
        // and obtain the index within the function section
        match self.pwasm_module.import_section() {
            Some(import) => import
                .entries()
                .iter()
                .filter(|entry| match entry.external() {
                    &External::Function(_) => true,
                    _ => false,
                })
                .count(),
            None => 0,
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub arg_types: Vec<ValueType>,
    pub return_type: ValueType,
}
