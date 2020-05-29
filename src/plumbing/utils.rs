use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn this_wrap(f: &Function) -> Function;
}
