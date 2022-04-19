//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use uk_areacodes::api::find_by_code;
use uk_areacodes_wasm::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn find() {
    assert_eq!(find_code("01582"), "Luton");
}
