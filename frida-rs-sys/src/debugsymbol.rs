use crate::nativepointer::NativePointer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name = DebugSymbol)]
    pub type DebugSymbol;

    #[wasm_bindgen(method, getter)]
    pub fn address(this: &DebugSymbol) -> NativePointer;

    #[wasm_bindgen(method, getter, js_name = name)]
    pub fn symbol_name(this: &DebugSymbol) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = moduleName)]
    pub fn module_name(this: &DebugSymbol) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = fileName)]
    pub fn file_name(this: &DebugSymbol) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = lineNumber)]
    pub fn line_number(this: &DebugSymbol) -> Option<u32>;

    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &DebugSymbol) -> String;

    #[wasm_bindgen(static_method_of = DebugSymbol, js_name = fromName)]
    pub fn from_name(name: String) -> DebugSymbol;

    #[wasm_bindgen(static_method_of = DebugSymbol, js_name = fromAddress)]
    pub fn from_address(address: &NativePointer) -> DebugSymbol;

    #[wasm_bindgen(static_method_of = DebugSymbol)]
    pub fn load(path: String);

    #[wasm_bindgen(static_method_of = DebugSymbol, js_name = findFunctionsNamed)]
    pub fn find_functions(name: String) -> js_sys::Array;

    #[wasm_bindgen(static_method_of = DebugSymbol, js_name = findFunctionsMatching)]
    pub fn find_matching_functions(name: String) -> js_sys::Array;
}
