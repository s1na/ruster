# Ruster
Ruster is an experiment to convert wasm modules into importable rust modules. 
At the moment it uses `parity_wasm` to extract information, such as signature of
exported functions from the wasm module, and produces a rust file, which can be imported
to invoke those functions or access the exported globals. Invoking those functions is handled
by `wasmi`.

In addition to the aforementioned approach to run wasm code via an interpreter, it is being
considered and research whether compiling a wasm source file to a rust file fully (so that no
runtime interpretation is necessary, i.e. converting also the instructions) is possible and desirable.

NOTE: This software is in very early stages, and is subject to change.

## Usage
Given a wasm source file such as the ones located in `tests/fixture`, you can run
`ruster tests/fixture/add.wasm -o add.rs` which produces the `add.rs` rust file. In order
to interact with it, you'll need to instantiate the `Module` struct located in the produced
rust file, providing `tests/fixture/add.wasm` as a parameter. If this succeeds, you can
invoke the exported functions and access exported globals by calling the methods on
this `Module` instance:

```rust
let module = add::Module::new("tests/fixture/add.wasm").expect("to instantiate module");
let res = module.add(1, 2).expect("to add");
assert_eq!(res, 3);
```

Please note that because something could go wrong during interpretation, all of the methods
return a `Result`.

If you're interested, the output, `add.rs` would look something like this:
```rust
extern crate wasmi;
extern crate parity_wasm;
extern crate ruster;

use wasmi::{NopExternals, RuntimeValue, ModuleRef, ModuleInstance, ImportsBuilder};
use std::error;

pub struct Module {
    instance: ModuleRef,
}

impl Module {
    pub fn new(filename: &str) -> Result<Module, Box<error::Error>> {
        {
            let module = parity_wasm::deserialize_file(&filename)?;
            let module = wasmi::Module::from_parity_wasm_module(module)?;
            let instance = ModuleInstance::new(&module, &ImportsBuilder::default())?
                .assert_no_start();

            Ok(Module { instance })
        }
    }

    pub fn add(&self, a0: i32, a1: i32) -> Result<i32, Box<error::Error>> {
        {
            let params = [RuntimeValue::I32(a0), RuntimeValue::I32(a1)];

            self.instance.invoke_export("add", &params, &mut NopExternals)?
            .ok_or(Box::from(ruster::error::Error::new("returned value is empty")))
            .and_then(|v| match v {
                RuntimeValue::I32(t) => Ok(t),
                _ => Err(Box::from(ruster::error::Error::new("returned value has invalid type")))
            })
        }
    }
}
```

## Testing
```bash
cargo run tests/fixture/add.wasm
# Please note, currently running the code is required for running
# the tests, in order to generate a out.rs, which the test imports.
cargo test
```

## Contributing
Please feel free to add issues for bugs or proposals, I'd love to hear your feedback.
I'm just learning Rust, so the code can might not be in the best shape.
However, if you have suggestions on how to improve it, please let me know, or even better, create a PR :)
