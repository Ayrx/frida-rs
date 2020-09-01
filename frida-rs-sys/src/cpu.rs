use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = CpuContext)]
    #[derive(Debug)]
    pub type CpuContext;
}
