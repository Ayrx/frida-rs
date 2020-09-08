use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name = ApiResolver)]
    #[derive(Debug)]
    pub type ApiResolver;

    #[wasm_bindgen(constructor)]
    pub fn new(resolver_type: String) -> ApiResolver;

    #[wasm_bindgen(method, js_name = enumerateMatches, catch)]
    pub fn enumerate_matches(this: &ApiResolver, query: String) -> Result<js_sys::Array, JsValue>;
}
