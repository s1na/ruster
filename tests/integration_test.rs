extern crate ruster;

#[test]
fn it_runs() {
    if let Err(e) = ruster::run("tests/fixture/add.wasm", "tests/out.rs") {
        panic!("run failed: {}", e);
    }
}
