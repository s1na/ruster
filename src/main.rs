extern crate ruster;

use std::env;
use std::process;

use ruster::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    if let Err(e) = run(&args[1]) {
        println!("error: {}", e);
        process::exit(1);
    }
}
