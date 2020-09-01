//!Frida functions for Interceptor functionality.
//!
//!The functions in this module correspond to the JavaScript functions
//!grouped under
//![https://frida.re/docs/javascript-api/#interceptor](https://frida.re/docs/javascript-api/#interceptor)

use crate::nativepointer::NativePointer;
use frida_rs_sys::utils::this_wrap;
use frida_rs_sys::interceptor;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub use frida_rs_sys::interceptor::{InvocationArgs, InvocationReturnValue};

///Per-invocation object that contains useful properties about the program at
///the point of interception.
///
///This is equivalent to the `this` object accessible in the Interceptor
///callback in the JavaScript API.
///
///The [`get`][get] and [`set`][set] methods can be used to store arbitrary
///Rust objects that will be accessible at any point during the invocation.
///For example, this could be used to store values during `on_enter` that can
///later be retrieved during `on_leave`.
///
///[get]: InvocationContext::get
///[set]: InvocationContext::set
#[derive(Debug)]
pub struct InvocationContext {
    ///Return address of the intercepted function.
    pub return_address: NativePointer,
    ///Current CPU context.
    pub context: crate::cpu::CpuContext,
    ///OS thread ID.
    pub thread_id: u32,
    ///Call depth relative to other invocations.
    pub depth: u32,
    _js: interceptor::InvocationContext,
}

impl InvocationContext {
    ///Get the value stored with the `prop` key.
    pub fn get<T>(&self, prop: &str) -> T
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        self._js.get(prop).into_serde().unwrap()
    }

    ///Stores `val` within the context with the `prop` key. `val` can be any
    ///serializable Rust object.
    pub fn set<T>(&self, prop: &str, val: &T)
    where
        T: serde::Serialize + ?Sized,
    {
        self._js.set(prop, JsValue::from_serde(val).unwrap())
    }
}

impl From<interceptor::InvocationContext> for InvocationContext {
    fn from(m: interceptor::InvocationContext) -> Self {
        InvocationContext {
            return_address: NativePointer::from_sys(m.return_address()),
            context: crate::cpu::CpuContext::from(m.context()),
            thread_id: m.thread_id(),
            depth: m.depth(),
            _js: m,
        }
    }
}

// TODO: This feels like a bad hack... see if there is a better way...
impl wasm_bindgen::describe::WasmDescribe for InvocationContext {
    fn describe() {
        interceptor::InvocationContext::describe()
    }
}

// TODO: This feels like a bad hack... see if there is a better way...
impl wasm_bindgen::convert::FromWasmAbi for InvocationContext {
    type Abi = u32;

    unsafe fn from_abi(js: u32) -> InvocationContext {
        let i = interceptor::InvocationContext::from_abi(js);
        InvocationContext::from(i)
    }
}

pub struct InvocationCallbacks {
    pub on_enter: Option<Box<dyn FnMut(InvocationContext, InvocationArgs)>>,
    pub on_leave: Option<Box<dyn FnMut(InvocationContext, InvocationReturnValue)>>,
}

///Intercept calls to `target`.
///
///This is equivalent to calling `Interceptor.attach` in the JavaScript API.
///
///```
///let callbacks = InvocationCallbacks {
///    on_enter: Some(Box::new(move |this: InvocationContext, args: InvocationArgs| {
///        ...
///    })),
///    on_leave: Some(Box::new(move |this: InvocationContext, retval: InvocationReturnValue| {
///        ...
///    }))
///};
///
///Interceptor::attach(target, callbacks);
///```
pub fn attach(target: NativePointer, callbacks: InvocationCallbacks) {
    let callbacks_object = Object::new();

    if let Some(f) = callbacks.on_enter {
        let on_enter = Closure::wrap(f);
        let on_enter_wrapped = this_wrap(on_enter.as_ref().unchecked_ref());
        js_sys::Reflect::set(
            &callbacks_object,
            &JsValue::from_str("onEnter"),
            &on_enter_wrapped,
        )
        .unwrap();
        on_enter.forget();
    }

    if let Some(f) = callbacks.on_leave {
        let on_leave = Closure::wrap(f);
        let on_leave_wrapped = this_wrap(on_leave.as_ref().unchecked_ref());
        js_sys::Reflect::set(
            &callbacks_object,
            &JsValue::from_str("onLeave"),
            &on_leave_wrapped,
        )
        .unwrap();
        on_leave.forget();
    }

    interceptor::attach(target.to_sys(), callbacks_object);
}
