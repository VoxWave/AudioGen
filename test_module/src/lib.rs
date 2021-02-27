#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[link(wasm_import_module = "heligen")]
extern "C" { fn output(o: u64); }

#[no_mangle]
fn heligen_start() {
    for i in 0..10000 {
        unsafe{output(i)};
    }
}