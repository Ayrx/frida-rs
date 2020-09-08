use crate::nativepointer::NativePointer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Memory, js_name = scanSync)]
    pub fn scan(address: &NativePointer, size: usize, pattern: String) -> js_sys::Array;
}
