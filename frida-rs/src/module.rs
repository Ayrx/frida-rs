//!Frida functions for module-level functionality.
//!
//!The functions in this module correspond to the JavaScript functions
//!grouped under
//![https://frida.re/docs/javascript-api/#module](https://frida.re/docs/javascript-api/#module).
use crate::fromsys::FromSys;
use crate::nativepointer::NativePointer;
use crate::range::RangeDetails;
use frida_rs_sys::module;
use wasm_bindgen::JsValue;

///Get the base address of the module named `name`.
///
///This is the equivalent to calling `Module.findBaseAddress()` /
///`Module.getBaseAddress()` in the JavaScript API.
pub fn get_base_address(name: &str) -> Option<NativePointer> {
    let ret = module::get_base_address(name);

    if ret.is_null() {
        return None;
    }

    Some(NativePointer::from_sys(ret))
}

///Get the absolute address of the export named `export_name`.
///
///This is the equivalent to calling `Module.findExportByName()` /
///`Module.getExportByName()` in the JavaScript API.
pub fn get_export(export_name: &str) -> Option<NativePointer> {
    let ret = module::get_export(JsValue::NULL, export_name);

    if ret.is_null() {
        return None;
    }

    Some(NativePointer::from_sys(ret))
}

//// // TODO: This is not actually finding any exports and appears to be a Frida
//// // bug. Revisit in the future...
//// pub fn get_export_in_module(module_name: &str, export_name: &str) -> Option<NativePointer> {
////     let ret = _get_export(JsValue::from_str(module_name), export_name);

////     if ret.is_null() {
////         return None;
////     }

////     let p: NativePointer = ret.into_serde().unwrap();
////     Some(p)
//// }

pub struct Module {
    ///Canonical module name.
    pub name: String,

    ///Base address of module.
    pub base: NativePointer,

    ///Size of module in bytes.
    pub size: usize,

    ///Full filesystem path of module.
    pub path: String,

    sys: frida_rs_sys::module::Module,
}

impl FromSys<frida_rs_sys::module::Module> for Module {
    fn from_sys(m: frida_rs_sys::module::Module) -> Self {
        Self {
            name: m.name(),
            base: NativePointer::from_sys(m.base()),
            size: m.size(),
            path: m.path(),
            sys: m,
        }
    }

    fn into_sys(self) -> frida_rs_sys::module::Module {
        self.sys
    }
}

impl From<frida_rs_sys::module::Module> for Module {
    fn from(m: frida_rs_sys::module::Module) -> Self {
        Self::from_sys(m)
    }
}

// TODO: This feels like a bad hack... see if there is a better way...
impl wasm_bindgen::describe::WasmDescribe for Module {
    fn describe() {
        module::Module::describe()
    }
}

// TODO: This feels like a bad hack... see if there is a better way...
impl wasm_bindgen::convert::FromWasmAbi for Module {
    type Abi = u32;

    unsafe fn from_abi(js: u32) -> Module {
        let i = module::Module::from_abi(js);
        Module::from_sys(i)
    }
}

impl Module {
    ///Get all exports of the module.
    ///
    ///This is the equivalent to calling `enumerateExports()` in the
    ///JavaScript API.
    pub fn enumerate_exports(&self) -> Vec<ExportDetails> {
        self.sys
            .enumerate_exports()
            .iter()
            .map(frida_rs_sys::module::ExportDetails::from)
            .map(ExportDetails::from)
            .collect()
    }

    ///Get all imports of the module.
    ///
    ///This is the equivalent to calling `enumerateImports()` in the
    ///JavaScript API.
    pub fn enumerate_imports(&self) -> Vec<ImportDetails> {
        self.sys
            .enumerate_imports()
            .iter()
            .map(frida_rs_sys::module::ImportDetails::from)
            .map(ImportDetails::from)
            .collect()
    }

    ///Get all symbols of the module.
    ///
    ///This is the equivalent to calling `enumerateSymbols()` in the
    ///JavaScript API.
    pub fn enumerate_symbols(&self) -> Vec<SymbolDetails> {
        self.sys
            .enumerate_symbols()
            .iter()
            .map(frida_rs_sys::module::SymbolDetails::from)
            .map(SymbolDetails::from)
            .collect()
    }

    ///Get all memory ranges satisfying `protection`.
    ///
    ///`protection` is a string with the form "rwx" where "rw-" means "must be
    ///at least readable and writable."
    ///
    ///This is the equivalent to calling `enumerateRanges()` in the
    ///JavaScript API.
    pub fn enumerate_ranges(&self, protection: &str) -> Vec<RangeDetails> {
        self.sys
            .enumerate_ranges(protection)
            .iter()
            .map(frida_rs_sys::range::RangeDetails::from)
            .map(RangeDetails::from)
            .collect()
    }

    //    // // TODO: This is not actually finding any exports and appears to be a Frida
    //    // // bug. Revisit in the future...
    //    // pub fn get_export(&self, name: &str) -> Option<NativePointer> {
    //    //     let m: M = self._ref.clone().unchecked_into();
    //    //     m._get_export(name)
    //    // }
}

pub struct ExportDetails {
    pub export_type: String, // TODO: This should be a Enum type
    pub name: String,
    pub address: NativePointer,
}

impl From<module::ExportDetails> for ExportDetails {
    fn from(m: module::ExportDetails) -> Self {
        ExportDetails {
            export_type: m.export_type(),
            name: m.name(),
            address: NativePointer::from_sys(m.address()),
        }
    }
}

pub struct ImportDetails {
    pub import_type: Option<String>, // TODO: This should be a Enum type
    pub name: String,
    pub module: Option<String>,
    pub address: Option<NativePointer>,
    pub slot: Option<NativePointer>,
}

impl From<module::ImportDetails> for ImportDetails {
    fn from(m: module::ImportDetails) -> Self {
        ImportDetails {
            import_type: m.import_type(),
            name: m.name(),
            module: m.module(),
            address: m.address().map(NativePointer::from_sys),
            slot: m.slot().map(NativePointer::from_sys),
        }
    }
}

pub struct SymbolDetails {
    pub is_global: bool,
    pub symbol_type: String, // TODO: This should be a Enum type
    pub section: Option<SymbolSectionDetails>,
    pub name: String,
    pub address: NativePointer,
    pub size: Option<usize>,
}

impl From<module::SymbolDetails> for SymbolDetails {
    fn from(m: module::SymbolDetails) -> Self {
        SymbolDetails {
            is_global: m.is_global(),
            symbol_type: m.symbol_type(),
            section: m.section().map(SymbolSectionDetails::from),
            name: m.name(),
            address: NativePointer::from_sys(m.address()),
            size: m.size(),
        }
    }
}

pub struct SymbolSectionDetails {
    pub id: String,
    pub protection: String,
}

impl From<module::SymbolSectionDetails> for SymbolSectionDetails {
    fn from(m: module::SymbolSectionDetails) -> Self {
        SymbolSectionDetails {
            id: m.id(),
            protection: m.protection(),
        }
    }
}
