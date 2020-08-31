use std::fmt;

#[derive(Debug)]
pub struct NativePointer(frida_rs_sys::nativepointer::NativePointer);

impl NativePointer {
    pub fn new(s: &str) -> Self {
        Self(frida_rs_sys::nativepointer::NativePointer::new(s))
    }

    pub fn from_i32(s: i32) -> Self {
        Self(frida_rs_sys::nativepointer::NativePointer::from_i32(s))
    }

    pub fn read_u8(&self) -> u8 {
        self.0.read_u8()
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub(crate) fn from_sys(s: frida_rs_sys::nativepointer::NativePointer) -> Self {
        Self(s)
    }

    pub(crate) fn to_sys(&self) -> &frida_rs_sys::nativepointer::NativePointer {
        &self.0
    }

    pub(crate) fn from_jsvalue(s: wasm_bindgen::JsValue) -> Self {
        Self(frida_rs_sys::nativepointer::NativePointer::from(s))
    }
}

impl fmt::Display for NativePointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
