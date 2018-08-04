extern crate codegen;
extern crate parity_wasm;
extern crate wabt;
extern crate wasmi;
extern crate itertools;

mod interpreter;
mod generator;
mod module;
mod function;
mod global;
pub mod error;

use std::error::Error;
use self::generator::Generator;
use interpreter::Interpreter;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn run(filename: &str, out_path: &str) -> Result<(), Box<Error>> {
    let interpreter = Interpreter::new(&filename)?;
    let module = interpreter.create_module()?;
    let generator = Generator::new(module);
    let output = generator.generate_rust()?;

    let path = Path::new(out_path);
    let mut file = File::create(&path)?;
    file.write_all(output.as_bytes())?;

    Ok(())
}
