use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    ///Wrapper for a pointer on the instrumented target.
    ///
    ///This is equivalent to the `NativePointer` class in the JavaScript API.
    #[wasm_bindgen(js_name = NativePointer)]
    #[derive(Debug)]
    pub type NativePointer;

    #[wasm_bindgen(constructor)]
    pub fn new(s: &str) -> NativePointer;

    #[wasm_bindgen(constructor)]
    pub fn from_i32(s: i32) -> NativePointer;

    #[wasm_bindgen(method, js_name = readU8)]
    pub fn read_u8(this: &NativePointer) -> u8;

    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &NativePointer) -> String;
}

impl fmt::Display for NativePointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
