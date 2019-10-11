mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct Node {
    id: String,
    parent: String,
    nonce: String,
    records: Vec<Record>
}
#[derive(Serialize, Deserialize)]
pub struct Record {
    id: String,
    email: String,
    ip_address: String,
    last_name: String,
    first_name: String,
    password: String,
    desc: String,
}
#[wasm_bindgen]
pub struct Stats(pub f64, pub usize);

#[wasm_bindgen(module = "/bind.js")]
extern "C" {
    fn updateState(state: JsValue);
}

#[wasm_bindgen]
pub fn compute(nodes: &JsValue, limit: usize) -> Stats {
    let mut rounds = 0;
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");
    let t = performance.now();
    updateState(JsValue::from_serde(&nodes).unwrap());
    Stats(performance.now() - t, rounds)
}
