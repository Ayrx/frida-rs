use crate::nativepointer::NativePointer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name = ModuleMap)]
    #[derive(Debug)]
    pub type ModuleMap;

    #[wasm_bindgen(constructor)]
    pub fn new() -> ModuleMap;

    #[wasm_bindgen(constructor)]
    pub fn new_with_filter(filter: JsValue) -> ModuleMap;

    #[wasm_bindgen(method)]
    pub fn has(this: &ModuleMap, address: &NativePointer) -> bool;

    #[wasm_bindgen(method, js_name = find)]
    pub fn get_module(this: &ModuleMap, address: &NativePointer) -> crate::module::Module;

    #[wasm_bindgen(method, js_name = findName)]
    pub fn get_name(this: &ModuleMap, address: &NativePointer) -> Option<String>;

    #[wasm_bindgen(method, js_name = findPath)]
    pub fn get_path(this: &ModuleMap, address: &NativePointer) -> Option<String>;

    #[wasm_bindgen(method)]
    pub fn update(this: &ModuleMap);

    #[wasm_bindgen(method)]
    pub fn values(this: &ModuleMap) -> js_sys::Array;
}
