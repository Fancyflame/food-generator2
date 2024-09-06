mod utils;

use std::fmt::Display;

use food_generator2::syntax::SerializeMap;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee-alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Library {
    lib: SerializeMap,
}

#[wasm_bindgen]
impl Library {
    pub fn load_lib(data: &[u8]) -> Result<Library, JsValue> {
        if data.is_empty() {
            return Err("read no data".into());
        }

        let lib = food_generator2::file::read_lib(&data)
            .ok_or_else(|| JsValue::from_str("parse library failed"))?;
        Ok(Library { lib })
    }

    pub fn encode(&self, txt: &str) -> String {
        food_generator2::encode(&self.lib, txt.as_bytes())
    }

    pub fn decode(&self, txt: &str) -> Result<String, String> {
        let bytes = food_generator2::decode(&self.lib, txt).map_err(map_err)?;
        String::from_utf8(bytes).map_err(map_err)
    }
}

fn map_err<E: Display>(err: E) -> String {
    err.to_string()
}
