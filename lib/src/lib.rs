mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Stats(pub f64, pub usize);

#[wasm_bindgen]
pub fn compute(nodes: &JsValue, limit: usize) -> Stats {
    let mut rounds = 0;
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");
    let t = performance.now();
    Stats(performance.now() - t, rounds)
}
