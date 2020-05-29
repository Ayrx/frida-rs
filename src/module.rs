//!Frida functions for module-level functionality.
//!
//!The functions in this module correspond to the JavaScript functions
//!grouped under
//![https://frida.re/docs/javascript-api/#module](https://frida.re/docs/javascript-api/#module).

use crate::nativepointer::NativePointer;
use crate::plumbing;
use crate::range::RangeDetails;
use wasm_bindgen::{JsCast, JsValue};

///Get the base address of the module named `name`.
///
///This is the equivalent to calling `Module.findBaseAddress()` /
///`Module.getBaseAddress()` in the JavaScript API.
pub fn get_base_address(name: &str) -> Option<NativePointer> {
    let ret = plumbing::module::get_base_address(name);

    if ret.is_null() {
        return None;
    }

    Some(ret)
}

///Get the absolute address of the export named `export_name`.
///
///This is the equivalent to calling `Module.findExportByName()` /
///`Module.getExportByName()` in the JavaScript API.
pub fn get_export(export_name: &str) -> Option<NativePointer> {
    let ret = plumbing::module::get_export(JsValue::NULL, export_name);

    if ret.is_null() {
        return None;
    }

    Some(ret)
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

    _ref: plumbing::module::Module,
}

impl From<plumbing::module::Module> for Module {
    fn from(m: plumbing::module::Module) -> Self {
        Module {
            name: m.name(),
            base: m.base(),
            size: m.size(),
            path: m.path(),
            _ref: m,
        }
    }
}

impl Module {
    ///Get all exports of the module.
    ///
    ///This is the equivalent to calling `enumerateExports()` in the
    ///JavaScript API.
    pub fn enumerate_exports(&self) -> Vec<ExportDetails> {
        let mut export_details = Vec::new();

        let m: plumbing::module::Module = self._ref.clone().unchecked_into();
        let exports = m.enumerate_exports();

        for export in exports.iter() {
            let i = ExportDetails::from(plumbing::module::ExportDetails::from(export));
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

        let m: plumbing::module::Module = self._ref.clone().unchecked_into();
        let imports = m.enumerate_imports();

        for import in imports.iter() {
            let i = ImportDetails::from(plumbing::module::ImportDetails::from(import));
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

        let m: plumbing::module::Module = self._ref.clone().unchecked_into();
        let symbols = m.enumerate_symbols();

        for symbol in symbols.iter() {
            let i = SymbolDetails::from(plumbing::module::SymbolDetails::from(symbol));
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

        let m: plumbing::module::Module = self._ref.clone().unchecked_into();
        let ranges = m.enumerate_ranges(protection);

        for range in ranges.iter() {
            let i = RangeDetails::from(plumbing::range::RangeDetails::from(range));
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

impl From<plumbing::module::ExportDetails> for ExportDetails {
    fn from(m: plumbing::module::ExportDetails) -> Self {
        ExportDetails {
            export_type: m.export_type(),
            name: m.name(),
            address: m.address(),
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

impl From<plumbing::module::ImportDetails> for ImportDetails {
    fn from(m: plumbing::module::ImportDetails) -> Self {
        ImportDetails {
            import_type: m.import_type(),
            name: m.name(),
            module: m.module(),
            address: m.address(),
            slot: m.slot(),
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

impl From<plumbing::module::SymbolDetails> for SymbolDetails {
    fn from(m: plumbing::module::SymbolDetails) -> Self {
        SymbolDetails {
            is_global: m.is_global(),
            symbol_type: m.symbol_type(),
            section: m.section().map(|s| SymbolSectionDetails::from(s)),
            name: m.name(),
            address: m.address(),
            size: m.size(),
        }
    }
}

pub struct SymbolSectionDetails {
    pub id: String,
    pub protection: String,
}

impl From<plumbing::module::SymbolSectionDetails> for SymbolSectionDetails {
    fn from(m: plumbing::module::SymbolSectionDetails) -> Self {
        SymbolSectionDetails {
            id: m.id(),
            protection: m.protection(),
        }
    }
}