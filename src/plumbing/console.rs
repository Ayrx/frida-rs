use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    ///Do not use this directly. Use the [`console_log`](console_log) macro
    ///instead.
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    ///Do not use this directly. Use the [`console_warn`](console_warn) macro
    ///instead.
    #[wasm_bindgen(js_namespace = console)]
    pub fn warn(s: &str);

    ///Do not use this directly. Use the [`console_error`](console_error) macro
    ///instead.
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}
