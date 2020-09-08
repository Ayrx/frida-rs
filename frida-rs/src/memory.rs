use crate::nativepointer::NativePointer;

///Matched values returned by [`scan`](crate::memory::scan).
pub struct Match {
    pub address: NativePointer,
    pub size: usize,
}

///Scan for occurences of `pattern` in memory range given by `address` and
///`size`.
///
///This is equivalent to calling `Memory.scan` in the JavaScript API.
pub fn scan(address: &NativePointer, size: usize, pattern: &str) -> Vec<Match> {
    frida_rs_sys::memory::scan(address.to_sys(), size, pattern.to_owned())
        .iter()
        .map(|x| {
            let address = NativePointer::from_jsvalue(
                js_sys::Reflect::get(&x, &wasm_bindgen::prelude::JsValue::from_str("address"))
                    .unwrap(),
            );
            let size: usize =
                js_sys::Reflect::get(&x, &wasm_bindgen::prelude::JsValue::from_str("size"))
                    .unwrap()
                    .as_f64()
                    .unwrap() as usize;

            Match { address, size }
        })
        .collect()
}
