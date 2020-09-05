use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Process)]
    pub static id: u32;

    #[wasm_bindgen(js_namespace = Process)]
    pub static arch: String;

    #[wasm_bindgen(js_namespace = Process)]
    pub static platform: String;

    #[wasm_bindgen(js_namespace = Process, js_name = pageSize)]
    pub static page_size: usize;

    #[wasm_bindgen(js_namespace = Process, js_name = pointerSize)]
    pub static pointer_size: usize;

    #[wasm_bindgen(js_namespace = Process, js_name = codeSigningPolicy)]
    pub static code_signing_policy: String;

    #[wasm_bindgen(js_namespace = Process, js_name = isDebuggerAttached)]
    pub fn is_debugger_attached() -> bool;

    #[wasm_bindgen(js_namespace = Process, js_name = getCurrentThreadId)]
    pub fn get_current_thread_id() -> u32;

    #[wasm_bindgen(js_namespace = Process, js_name = enumerateThreads)]
    pub fn enumerate_threads() -> js_sys::Array;

    #[wasm_bindgen(js_namespace = Process, js_name = enumerateModules)]
    pub fn enumerate_modules() -> js_sys::Array;

    #[wasm_bindgen(js_namespace = Process, js_name = findModuleByName)]
    pub fn get_module_by_name(name: &str) -> crate::module::Module;

    #[wasm_bindgen(js_namespace = Process, js_name = findModuleByAddress)]
    pub fn get_module_by_address(
        address: &crate::nativepointer::NativePointer,
    ) -> crate::module::Module;

    #[wasm_bindgen(js_namespace = Process, js_name = findRangeByAddress)]
    pub fn get_range_by_address(address: &crate::nativepointer::NativePointer) -> JsValue;

    #[wasm_bindgen(js_namespace = Process, js_name = enumerateRanges)]
    pub fn enumerate_ranges(protection: &str) -> js_sys::Array;

    #[wasm_bindgen(js_namespace = Process, js_name = enumerateMallocRanges)]
    pub fn enumerate_malloc_ranges() -> js_sys::Array;
}
