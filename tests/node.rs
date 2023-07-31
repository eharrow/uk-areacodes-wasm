//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

use uk_areacodes_wasm::{find_code, starts_with_code};

#[wasm_bindgen_test]
fn find() {
    let string = find_code("01582");
    assert_eq!(string, "Luton");
}

#[wasm_bindgen_test]
fn find_by_starts_with() {
    let string = starts_with_code("01582 12345678");
    assert_eq!(string, "Luton");
}
