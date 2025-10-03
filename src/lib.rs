mod utils;

use std::collections::HashMap;
use uk_areacodes::api::{self, Place};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    set_panic_hook();

    Ok(())
}

#[wasm_bindgen(getter_with_clone)]
pub struct JSPlace {
    pub area: String,
    pub lat: String,
    pub lon: String,
}

#[wasm_bindgen]
pub fn find_code(numb: &str) -> String {
    let data: HashMap<String, Place> = api::load_as_map();

    let r = api::find_by_code(numb, &data);
    let area = match r {
        Some(p) => &p.area,
        None => "",
    };
    area.to_string()
}

#[wasm_bindgen]
pub fn find_place_by_code(numb: &str) -> JSPlace {
    let data: HashMap<String, Place> = api::load_as_map();

    let r = api::find_by_code(numb, &data);
    match r {
        Some(p) => JSPlace {
            area: p.area.clone(),
            lat: p.lat.clone().unwrap(),
            lon: p.lon.clone().unwrap(),
        },
        None => JSPlace {
            area: "".to_string(),
            lat: "".to_string(),
            lon: "".to_string(),
        },
    }
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
    fn it_returns_find_place_by_code() {
        let place = find_place_by_code("01582");
        assert_eq!(place.area, "Luton");
        assert_eq!(place.lat, "51.8784385");
        assert_eq!(place.lon, "-0.4152837");
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
        println!("Place: {}", place);
    }

    #[test]
    fn it_returns_empty_for_invalid_starts_with_code() {
        let place = starts_with_code("invalid");
        assert_eq!(place, "");
    }
}
