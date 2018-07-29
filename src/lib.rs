extern crate codegen;
extern crate parity_wasm;
extern crate wabt;
extern crate wasmi;

mod generate;
mod module;

use std::error::Error;
use self::generate::generate_rust;
use module::{Module};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn run(filename: &str, out_path: &str) -> Result<(), Box<Error>> {
    let module = Module::new(&filename)?;
    let output = generate_rust(module)?;

    let path = Path::new(out_path);
    let mut file = File::create(&path)?;
    file.write_all(output.as_bytes())?;

    Ok(())
}
