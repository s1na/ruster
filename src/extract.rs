extern crate parity_wasm;

use parity_wasm::elements::{External, FunctionType, Internal, Type, ValueType};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub arg_types: Vec<ValueType>,
    pub return_type: ValueType,
}

pub fn get_exported_fns(module: &parity_wasm::elements::Module) -> Vec<Function> {
    let mut exported_fns: Vec<Function> = Vec::new();

    // Export section has an entry with a func_name with an index inside a module
    let export_section = module.export_section().expect("No export section found");
    // It's a section with function declarations (which are references to the type section entries)
    let function_section = module
        .function_section()
        .expect("No function section found");
    // Type section stores function types which are referenced by function_section entries
    let type_section = module.type_section().expect("No type section found");
    let imported_fns_len = get_imported_fns_len(&module);

    for export in export_section.entries() {
        let name = export.field();
        let function_index: usize = match export.internal() {
            &Internal::Function(i) => i as usize,
            _ => panic!("Found non-function export"),
        };

        // Calculates a function index within module's function section
        let function_index_in_section = function_index - imported_fns_len;

        // Getting a type reference from a function section entry
        let func_type_ref: usize =
            function_section.entries()[function_index_in_section].type_ref() as usize;

        // Use the reference to get an actual function type
        let function_type: &FunctionType = match &type_section.types()[func_type_ref] {
            &Type::Function(ref func_type) => func_type,
        };

        // Parses arguments and constructs runtime values in correspondence of their types
        let arg_types = Vec::from(function_type.params());
        let return_type = function_type.return_type().unwrap();

        exported_fns.push(Function {
            name: name.to_string(),
            arg_types,
            return_type,
        });
    }

    exported_fns
}

fn get_imported_fns_len(module: &parity_wasm::elements::Module) -> usize {
    // We need to count import section entries (functions only!) to subtract it from function_index
    // and obtain the index within the function section
    match module.import_section() {
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
