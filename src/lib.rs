mod utils;

use uk_areacodes::api::{self};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    set_panic_hook();

    Ok(())
}

#[wasm_bindgen]
pub fn find_code(numb: &str) -> String {
    let data: Vec<api::Place> = api::load();

    let r = api::find_by_code(numb, &data);
    let area = match r {
        Some(p) => &p.area,
        None => "",
    };
    area.to_string()
}

pub fn starts_with_code(numb: &str) -> String {
    let data: Vec<api::Place> = api::load();
    let r = api::starts_with_code(numb, &data);
    let area = match r {
        Some(p) => &p.area,
        None => "",
    };
    area.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_find_code() {
        let place = find_code("01582");
        assert_eq!(place, "Luton");
    }

    #[test]
    fn it_returns_empty_for_invalid_code() {
        let place = find_code("invalid");
        assert_eq!(place, "");
    }

    #[test]
    fn it_returns_starts_with() {
        let place = starts_with_code("01582 12345678");
        assert_eq!(place, "Luton");
    }

    #[test]
    fn it_returns_empty_for_invalid_starts_with_code() {
        let place = starts_with_code("invalid");
        assert_eq!(place, "");
    }
}
