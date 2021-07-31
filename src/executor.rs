use anyhow::Result;
use wasmtime::*;

pub fn execute(x: i32, y: i32) -> Result<i32> {

    println!("Compiling module...");
    let engine = Engine::default();
    let module = Module::from_file(&engine, "./example/target/wasm32-unknown-unknown/debug/example.wasm")?;

    println!("initializing function...");
    let mut store = Store::new(&engine, ());

    println!("Instantiating module...");
    let instance = Instance::new(&mut store, &module, &[])?;

    println!("Extracting export...");
    let add = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, "add")?;

    println!("Calling export...");
    let response = add.call(&mut store, (x, y))?;
    println!("add(6, 27) = {}", response);

    Ok(response)
}
