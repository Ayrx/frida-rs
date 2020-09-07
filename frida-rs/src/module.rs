//!Frida functions for module-level functionality.
//!
//!The functions in this module correspond to the JavaScript functions
//!grouped under
//![https://frida.re/docs/javascript-api/#module](https://frida.re/docs/javascript-api/#module).
use crate::fromsys::FromSys;
use crate::nativepointer::NativePointer;
use crate::range::RangeDetails;
use frida_rs_sys::module;
use wasm_bindgen::{JsCast, JsValue};

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

impl Module {
    ///Get all exports of the module.
    ///
    ///This is the equivalent to calling `enumerateExports()` in the
    ///JavaScript API.
    pub fn enumerate_exports(&self) -> Vec<ExportDetails> {
        let mut export_details = Vec::new();

        let m: module::Module = self.sys.clone().unchecked_into();
        let exports = m.enumerate_exports();

        for export in exports.iter() {
            let i = ExportDetails::from(module::ExportDetails::from(export));
            export_details.push(i);
        }

        export_details
    }

    ///Get all imports of the module.
    ///
    ///This is the equivalent to calling `enumerateImports()` in the
    ///JavaScript API.
    pub fn enumerate_imports(&self) -> Vec<ImportDetails> {
        let mut import_details = Vec::new();

        let m: module::Module = self.sys.clone().unchecked_into();
        let imports = m.enumerate_imports();

        for import in imports.iter() {
            let i = ImportDetails::from(module::ImportDetails::from(import));
            import_details.push(i);
        }

        import_details
    }

    ///Get all symbols of the module.
    ///
    ///This is the equivalent to calling `enumerateSymbols()` in the
    ///JavaScript API.
    pub fn enumerate_symbols(&self) -> Vec<SymbolDetails> {
        let mut symbol_details = Vec::new();

        let m: module::Module = self.sys.clone().unchecked_into();
        let symbols = m.enumerate_symbols();

        for symbol in symbols.iter() {
            let i = SymbolDetails::from(module::SymbolDetails::from(symbol));
            symbol_details.push(i);
        }

        symbol_details
    }

    ///Get all memory ranges satisfying `protection`.
    ///
    ///`protection` is a string with the form "rwx" where "rw-" means "must be
    ///at least readable and writable."
    ///
    ///This is the equivalent to calling `enumerateRanges()` in the
    ///JavaScript API.
    pub fn enumerate_ranges(&self, protection: &str) -> Vec<RangeDetails> {
        let mut range_details = Vec::new();

        let m: module::Module = self.sys.clone().unchecked_into();
        let ranges = m.enumerate_ranges(protection);

        for range in ranges.iter() {
            let i = RangeDetails::from(frida_rs_sys::range::RangeDetails::from(range));
            range_details.push(i);
        }

        range_details
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
