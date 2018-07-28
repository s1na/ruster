extern crate codegen;
extern crate parity_wasm;
extern crate wabt;
extern crate wasmi;

mod generate;
mod module;

use std::error::Error;
use self::generate::generate_rust;
use module::{Module};

pub fn run(filename: &str, out_path: &str) -> Result<(), Box<Error>> {
    let module = Module::new(&filename)?;
    let exported_fns = module.get_exported_fns();
    generate_rust(exported_fns, out_path);

    Ok(())
}
