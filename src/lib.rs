mod utils;

use wasm_bindgen::prelude::*;
use rand::Rng;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: u64);
}

#[wasm_bindgen]
pub fn greet() {
    let mut rng = rand::thread_rng();

    let big_number: u64 = rng.gen();

    alert(big_number);
}
