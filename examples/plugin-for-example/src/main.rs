extern "C" {
    fn it_works() -> i32;
}

pub fn wasm_fn1() {
    let foo = vec![1, 2, 3];
    println!("{}", foo[5]);
}

pub fn wasm_fn2() {
    let foo = vec![1, 2, 3];
    println!("{}", foo[5]);
}

#[no_mangle]
pub fn plugin_entrypoint(n: i32) -> i32 {
    println!("Hello from inside WASI");
    let result = unsafe { it_works() };

    let fns = [
        Box::new(wasm_fn1) as Box<dyn Fn()>,
        Box::new(wasm_fn2) as Box<dyn Fn()>,
    ];

    (fns[n as usize % 2])();
    result + n
}

pub fn main() {}
