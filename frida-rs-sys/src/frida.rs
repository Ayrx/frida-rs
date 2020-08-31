use crate::nativepointer::NativePointer;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Frida)]
    pub static version: String;

    #[wasm_bindgen(js_namespace = Frida, js_name = heapSize)]
    pub static heap_size: usize;

    #[wasm_bindgen(js_name = hexdump)]
    pub fn hexdump(target: &NativePointer) -> String;

    #[wasm_bindgen(js_name = hexdump)]
    pub fn hexdump_arraybuffer(target: &ArrayBuffer) -> String;

    #[wasm_bindgen(js_name = send)]
    pub fn send(message: &JsValue, data: &JsValue);

    #[wasm_bindgen(js_name = recv)]
    pub fn recv(callback: &js_sys::Function);

    #[wasm_bindgen(js_name = recv)]
    pub fn recv_with_type(type_filter: &str, callback: &js_sys::Function);

    ///Generic type representing a message from your Frida application.
    ///
    ///This should be deserialized into an appropriate Rust structure with the
    ///[`into_serde`](RecvMessage::into_serde) method.
    #[wasm_bindgen]
    #[derive(Debug)]
    pub type RecvMessage;

    #[wasm_bindgen(js_name = ArrayBuffer)]
    #[derive(Debug)]
    pub type ArrayBuffer;

    #[wasm_bindgen(method, js_name = unwrap)]
    pub fn unwrap(this: &ArrayBuffer) -> NativePointer;
}

impl RecvMessage {
    pub fn into_serde<T>(self) -> serde_json::Result<T>
    where
        T: for<'a> serde::de::Deserialize<'a>,
    {
        let s = JsValue::unchecked_from_js_ref(&self);
        s.into_serde()
    }
}
