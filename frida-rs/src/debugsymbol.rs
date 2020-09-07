//!Frida functions for debug symbols.
//!
//!The functions in this module correspond to the JavaScript functions
//!grouped under
//![https://frida.re/docs/javascript-api/#debugsymbol](https://frida.re/docs/javascript-api/#debugsymbol).

use crate::fromsys::FromSys;
use crate::nativepointer::NativePointer;
use std::fmt;

pub struct DebugSymbol {
    ///Address that this symbol is for.
    pub address: NativePointer,

    ///Name of the symbol.
    pub symbol_name: Option<String>,

    ///Name of module owning this symbol.
    pub module_name: Option<String>,

    ///Name of file owning this symbol.
    pub file_name: Option<String>,

    ///Line number in `file_name`
    pub line_number: Option<u32>,

    sys: frida_rs_sys::debugsymbol::DebugSymbol,
}

impl FromSys<frida_rs_sys::debugsymbol::DebugSymbol> for DebugSymbol {
    fn from_sys(m: frida_rs_sys::debugsymbol::DebugSymbol) -> Self {
        Self {
            address: NativePointer::from_sys(m.address()),
            symbol_name: m.symbol_name(),
            module_name: m.module_name(),
            file_name: m.file_name(),
            line_number: m.line_number(),
            sys: m,
        }
    }

    fn into_sys(self) -> frida_rs_sys::debugsymbol::DebugSymbol {
        self.sys
    }
}

impl fmt::Display for DebugSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sys.to_string())
    }
}

impl DebugSymbol {
    ///Find debug symbol for `name`.
    ///
    ///This is equivalent to calling `DebugSymbol.fromName` in the JavaScript
    ///API.
    pub fn from_name(name: &str) -> Self {
        Self::from_sys(frida_rs_sys::debugsymbol::DebugSymbol::from_name(
            name.to_owned(),
        ))
    }

    ///Find debug symbol for `address`.
    ///
    ///This is equivalent to calling `DebugSymbol.fromAddress` in the
    ///JavaScript API.
    pub fn from_address(address: &NativePointer) -> Self {
        Self::from_sys(frida_rs_sys::debugsymbol::DebugSymbol::from_address(
            address.to_sys(),
        ))
    }
}

///Load debug symbols for a specific module.
///
///This is equivalent to calling `DebugSymbol.load` in the JavaScript API.
pub fn load_symbols(path: &str) {
    frida_rs_sys::debugsymbol::DebugSymbol::load(path.to_owned())
}

///Resolve a function name and return its addresses.
///
///This is equivalent to calling `DebugSymbol.findFunctionsNamed` in the
///JavaScript API.
pub fn find_functions(name: &str) -> Vec<NativePointer> {
    frida_rs_sys::debugsymbol::DebugSymbol::find_functions(name.to_owned())
        .iter()
        .map(NativePointer::from_jsvalue)
        .collect()
}

///Resolve function names matching `glob` and return their addresses.
///
///This is equivalent to calling `DebugSymbol.findFunctionsMatching` in the
///JavaScript API.
pub fn find_matching_functions(glob: &str) -> Vec<NativePointer> {
    frida_rs_sys::debugsymbol::DebugSymbol::find_matching_functions(glob.to_owned())
        .iter()
        .map(NativePointer::from_jsvalue)
        .collect()
}
