mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let a: i32 = 3;
    let b: i32 = 2;
    let c: i32 = a + b;
    alert(&c.to_string());
    alert("Hello, what is new?");
}

#[wasm_bindgen]
pub fn adder(a: usize, b: usize) {
    let c = a + b;
    greet();
}
