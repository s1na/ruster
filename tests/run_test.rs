extern crate wasmi;

mod out;

#[test]
fn it_adds() {
    let module = out::Module::new("tests/fixture/add.wasm").expect("to instantiate module");
    let res = module.add(1, 2).expect("to add");
    assert_eq!(res, 3);
}
