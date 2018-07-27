# ruster
Ruster is an experiment to convert wasm modules into importable rust modules.
At the moment it uses `wasmi` and `parity_wasm` to extract needed information from the wasm
module and generate a rust file which exports a `Module`. It needs to be initialized
with the path to the wasm file, and then its methods can be invoked which invoke the `wasmi`
interpreter to execute the function.

Compiling a wasm source file to a rust file, without the need for an interpreter is also being considered.

NOTE: This software is in very early stages, and is just an experiment.

# testing
```bash
cargo run tests/fixture/add.wasm
# Please note, currently running the code is required for running
# the tests, in order to generate a out.rs, which the test imports.
cargo test
```

# contributing
I'm just learning Rust, so the code can be ugly and inefficient. However, if you have suggestions on how to improve it, please let me know, or even better, create a PR :)
