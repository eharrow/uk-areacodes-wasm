mod utils;

use uk_areacodes::api;
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
pub fn find_code(numb: &str) {
    let data: Vec<api::Place> = api::init();

    if let Some(p) = api::find_by_code(numb, &data) {
        alert(&format!("area is {:#?}", p.area));
    }

    alert("Hello, uk-areacodes-wasm!");
}
