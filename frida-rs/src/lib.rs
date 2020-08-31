//!Write [Frida](https://frida.re/) scripts in Rust thanks to the power of
//!WebAssembly.
//!
//!For an example of how to structure an agent using this crate, please refer
//!to [https://github.com/Ayrx/frida-rs-example](https://github.com/Ayrx/frida-rs-example).
//!
//!This crate is still a work-in-progress. The API is not stable and is
//!subject to breaking changes until the crate reaches 1.0. Use with care.
mod nativepointer;
mod plumbing;

pub mod console;
pub mod cpu;
pub mod interceptor;
pub mod module;
pub mod process;
pub mod range;
pub mod thread;

pub use nativepointer::NativePointer;
pub use plumbing::frida::ArrayBuffer;
pub use plumbing::frida::RecvMessage;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

///Get the current Frida version.
///
///This is equivalent to calling `Frida.version` in the JavaScript API.
pub fn version() -> &'static str {
    &*crate::plumbing::frida::version
}

///Get the current size of Frida's private heap.
///
///This is equivalent to calling `Frida.heapSize` in the JavaScript API.
pub fn heap_size() -> usize {
    *crate::plumbing::frida::heap_size
}

///Get the runtime in use.
///
///This is equivalent to calling `Script.runtime` in the JavaScript API.
pub fn runtime() -> &'static str {
    &*crate::plumbing::script::runtime
}

///Generate a hexdump for the provided `target`.
///
///This is equivalent to calling `hexdump` in the JavaScript API.
pub fn hexdump(target: &nativepointer::NativePointer) -> String {
    crate::plumbing::frida::hexdump(target)
}

///Generate a hexdump for the provided `target`.
///
///This is equivalent to calling `hexdump` in the JavaScript API.
pub fn hexdump_arraybuffer(target: &ArrayBuffer) -> String {
    crate::plumbing::frida::hexdump_arraybuffer(target)
}

///Send a message to your Frida application.
///
///This is equivalent to calling `send` in the JavaScript API.
pub fn send<T>(message: &T)
where
    T: serde::Serialize + ?Sized,
{
    crate::plumbing::frida::send(&JsValue::from_serde(&message).unwrap(), &JsValue::NULL);
}

///Send a message to your Frida application.
///
///This variant allows you to send raw bytes along with your message. This is
///equivalent to calling `send` in the JavaScript API.
pub fn send_with_byte_array<T>(message: &T, data: &[u8])
where
    T: serde::Serialize + ?Sized,
{
    let data = js_sys::Uint8Array::from(data);
    crate::plumbing::frida::send(
        &JsValue::from_serde(&message).unwrap(),
        &data.unchecked_into(),
    );
}

///Receive a message from your Frida application.
///
///This is equivalent to calling `recv` in the JavaScript API.
pub fn recv(callback: Box<dyn FnMut(RecvMessage, Option<ArrayBuffer>)>) {
    let c = Closure::wrap(callback);
    let f: &js_sys::Function = c.as_ref().unchecked_ref();
    crate::plumbing::frida::recv(f);
    c.forget();
}

///Receive a message from your Frida application.
///
///This variant allows you to filter the received messages by the "type" field.
///This is equivalent to calling `recv` in the JavaScript API.
pub fn recv_with_type(
    type_filter: &str,
    callback: Box<dyn FnMut(RecvMessage, Option<ArrayBuffer>)>,
) {
    let c = Closure::wrap(callback);
    let f: &js_sys::Function = c.as_ref().unchecked_ref();
    crate::plumbing::frida::recv_with_type(type_filter, f);
    c.forget();
}
