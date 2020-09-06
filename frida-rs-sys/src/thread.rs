use crate::cpu::CpuContext;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = ThreadDetails)]
    pub type ThreadDetails;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &ThreadDetails) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn state(this: &ThreadDetails) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn context(this: &ThreadDetails) -> CpuContext;

    #[wasm_bindgen(js_namespace = Thread, js_name = sleep)]
    pub fn sleep(delay: JsValue);

    #[wasm_bindgen(js_namespace = Thread, js_name = backtrace)]
    pub fn backtrace(ctx: Option<CpuContext>, backtracer: &JsValue) -> js_sys::Array;

    #[wasm_bindgen(js_namespace = Backtracer, js_name = ACCURATE)]
    pub static BacktracerAccurate: JsValue;

    #[wasm_bindgen(js_namespace = Backtracer, js_name = FUZZY)]
    pub static BacktracerFuzzy: JsValue;
}
