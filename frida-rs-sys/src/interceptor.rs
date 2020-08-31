use crate::nativepointer::NativePointer;
use std::fmt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Interceptor, js_name = attach)]
    pub fn attach(target: &NativePointer, callbacks: js_sys::Object);

    ///Arguments to an invocation of the intercepted function.
    ///
    ///Use the [`get`][get] method to access the arguments by index. Frida
    ///makes no guarantees about the validty of the index and it is possible
    ///to index into more arguments than the intercepted function accepts.
    ///
    ///[get]: InvocationArgs::get
    #[wasm_bindgen(js_name = InvocationArg)]
    #[derive(Debug)]
    pub type InvocationArgs;

    #[wasm_bindgen(js_name = InvocationContext)]
    #[derive(Debug)]
    pub type InvocationContext;

    #[wasm_bindgen(method, getter, js_name = returnAddress)]
    pub fn return_address(this: &InvocationContext) -> NativePointer;

    #[wasm_bindgen(method, getter, js_name = context)]
    pub fn context(this: &InvocationContext) -> crate::cpu::CpuContext;

    #[wasm_bindgen(method, getter, js_name = threadId)]
    pub fn thread_id(this: &InvocationContext) -> u32;

    #[wasm_bindgen(method, getter, js_name = depth)]
    pub fn depth(this: &InvocationContext) -> u32;

    #[wasm_bindgen(method, structural, indexing_getter)]
    pub fn get(this: &InvocationContext, prop: &str) -> JsValue;

    #[wasm_bindgen(method, structural, indexing_setter)]
    pub fn set(this: &InvocationContext, prop: &str, val: JsValue);

    ///Return value for an invocation of the intercepted function.
    ///
    ///This is an extension of [`NativePointer`][NativePointer] with an
    ///additional [`replace`][replace] method.
    ///
    ///[NativePointer]: NativePointer
    ///[replace]: InvocationReturnValue::replace
    #[wasm_bindgen(js_name = InvocationReturnValue, extends = NativePointer)]
    #[derive(Debug)]
    pub type InvocationReturnValue;

    #[wasm_bindgen(constructor)]
    fn new(s: &str) -> InvocationReturnValue;

    ///Replace the return value for an invocation of the intercepted function.
    #[wasm_bindgen(method, js_name = replace)]
    pub fn replace(this: &InvocationReturnValue, value: NativePointer);
}

impl InvocationArgs {
    pub fn get(&self, index: u32) -> NativePointer {
        let a: &js_sys::Array = &self.unchecked_ref();
        a.get(index).unchecked_into()
    }
}

impl fmt::Display for InvocationReturnValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p: &NativePointer = self.as_ref();
        write!(f, "{}", p.to_string())
    }
}
