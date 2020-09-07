//!Frida functions for module map functionality.
//!
//!The functions in this module correspond to the JavaScript functions
//!grouped under
//![https://frida.re/docs/javascript-api/#modulemap](https://frida.re/docs/javascript-api/#modulemap).

use crate::fromsys::FromSys;
use crate::nativepointer::NativePointer;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct ModuleMap(frida_rs_sys::modulemap::ModuleMap);

impl ModuleMap {
    ///Create a new module map object.
    pub fn new() -> Self {
        Self(frida_rs_sys::modulemap::ModuleMap::new())
    }

    ///Create a new filtered module map object.
    ///
    ///`filter` is a closure that takes a [`Module`](crate::module::Module)
    ///object and must return true if the module belongs in the map. The
    ///closure is called for each loaded module every time the map is updated.
    ///
    ///```
    ///ModuleMap::new_with_filter(Box::new(|x: crate::module::Module| -> bool {
    ///   ...
    ///}));
    ///```
    pub fn new_with_filter(filter: Box<dyn Fn(crate::module::Module) -> bool>) -> Self {
        let closure = Closure::wrap(filter);
        Self(frida_rs_sys::modulemap::ModuleMap::new_with_filter(
            closure.into_js_value(),
        ))
    }

    ///Check if `address` belongs to any of the contained modules.
    ///
    ///This is equivalent to calling `ModuleMap.has` in the JavaScript API.
    pub fn has(&self, address: &NativePointer) -> bool {
        self.0.has(address.to_sys())
    }

    ///Get [`Module`](crate::module::Module) that `address` belongs to.
    ///
    ///This is equivalent to calling `ModuleMap.find` / `ModuleMap.get` in the
    ///JavaScript API.
    pub fn get_module(&self, address: &NativePointer) -> Option<crate::module::Module> {
        let ret = self.0.get_module(address.to_sys());

        if ret.is_null() {
            return None;
        }

        Some(crate::module::Module::from_sys(ret))
    }

    ///Get the name of the module that `address` belongs to.
    ///This is equivalent to calling `ModuleMap.findName` / `ModuleMap.getName`
    ///in the JavaScript API.
    pub fn get_name(&self, address: &NativePointer) -> Option<String> {
        self.0.get_name(address.to_sys())
    }

    ///Get the path of the module that `address` belongs to.
    ///
    ///This is equivalent to calling `ModuleMap.findPath` / `ModuleMap.getPath`
    ///in the JavaScript API.
    pub fn get_path(&self, address: &NativePointer) -> Option<String> {
        self.0.get_path(address.to_sys())
    }

    ///Update the module map.
    ///
    ///This is equivalent to calling `ModuleMap.update` in the JavaScript API.
    pub fn update(&self) {
        self.0.update();
    }

    ///Returns an array with the [`Module`](crate::module::Module) objects
    ///currently in the map.
    ///
    ///This is equivalent to calling `ModuleMap.values` in the JavaScript API.
    pub fn values(&self) -> Vec<crate::module::Module> {
        self.0
            .values()
            .iter()
            .map(frida_rs_sys::module::Module::from)
            .map(crate::module::Module::from_sys)
            .collect()
    }
}

impl Default for ModuleMap {
    fn default() -> Self {
        Self::new()
    }
}
