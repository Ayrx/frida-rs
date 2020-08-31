use crate::nativepointer::NativePointer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = RangeDetails)]
    pub type RangeDetails;

    #[wasm_bindgen(method, getter)]
    pub fn base(this: &RangeDetails) -> NativePointer;

    #[wasm_bindgen(method, getter)]
    pub fn size(this: &RangeDetails) -> usize;

    #[wasm_bindgen(method, getter)]
    pub fn protection(this: &RangeDetails) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn file(this: &RangeDetails) -> Option<FileMapping>;

    #[wasm_bindgen(js_name = FileMapping)]
    pub type FileMapping;

    #[wasm_bindgen(method, getter)]
    pub fn path(this: &FileMapping) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn offset(this: &FileMapping) -> usize;

    #[wasm_bindgen(method, getter)]
    pub fn size(this: &FileMapping) -> usize;
}
