use crate::plumbing::nativepointer::NativePointer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name = Module)]
    pub type Module;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Module) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn base(this: &Module) -> NativePointer;

    #[wasm_bindgen(method, getter)]
    pub fn size(this: &Module) -> usize;

    #[wasm_bindgen(method, getter)]
    pub fn path(this: &Module) -> String;

    #[wasm_bindgen(method, js_name = enumerateExports)]
    pub fn enumerate_exports(this: &Module) -> js_sys::Array;

    #[wasm_bindgen(method, js_name = enumerateImports)]
    pub fn enumerate_imports(this: &Module) -> js_sys::Array;

    #[wasm_bindgen(method, js_name = enumerateSymbols)]
    pub fn enumerate_symbols(this: &Module) -> js_sys::Array;

    #[wasm_bindgen(method, js_name = enumerateRanges)]
    pub fn enumerate_ranges(this: &Module, protection: &str) -> js_sys::Array;

    #[wasm_bindgen(js_name = ModuleExportDetails)]
    pub type ExportDetails;

    #[wasm_bindgen(method, getter, js_name = type)]
    pub fn export_type(this: &ExportDetails) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &ExportDetails) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn address(this: &ExportDetails) -> NativePointer;

    #[wasm_bindgen(js_name = ModuleImportDetails)]
    pub type ImportDetails;

    #[wasm_bindgen(method, getter, js_name = type)]
    pub fn import_type(this: &ImportDetails) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &ImportDetails) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn module(this: &ImportDetails) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn address(this: &ImportDetails) -> Option<NativePointer>;

    #[wasm_bindgen(method, getter)]
    pub fn slot(this: &ImportDetails) -> Option<NativePointer>;

    #[wasm_bindgen(js_name = ModuleSymbolDetails)]
    pub type SymbolDetails;

    #[wasm_bindgen(method, getter, js_name = isGlobal)]
    pub fn is_global(this: &SymbolDetails) -> bool;

    #[wasm_bindgen(method, getter, js_name = type)]
    pub fn symbol_type(this: &SymbolDetails) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn section(this: &SymbolDetails) -> Option<SymbolSectionDetails>;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &SymbolDetails) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn address(this: &SymbolDetails) -> NativePointer;

    #[wasm_bindgen(method, getter)]
    pub fn size(this: &SymbolDetails) -> Option<usize>;

    #[wasm_bindgen(js_name = ModuleSymbolSectionDetails)]
    pub type SymbolSectionDetails;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &SymbolSectionDetails) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn protection(this: &SymbolSectionDetails) -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Module, js_name = findBaseAddress)]
    pub fn get_base_address(name: &str) -> NativePointer;

    #[wasm_bindgen(js_namespace = Module, js_name = findExportByName)]
    pub fn get_export(module_name: JsValue, export_name: &str) -> NativePointer;
}
