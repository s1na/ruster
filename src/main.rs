extern crate clap;
extern crate ruster;

use clap::{App, Arg};
use std::process;

use ruster::run;

fn main() {
    let matches = App::new("ruster")
        .version("0.1.0")
        .about("Convert wasm to importable rust modules")
        .arg(
            Arg::with_name("INPUT")
                .help("Path to wasm file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Path to the resulting rust module")
                .takes_value(true),
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("output").unwrap_or("out.rs");

    if let Err(e) = run(input, output) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
