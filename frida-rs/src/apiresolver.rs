//!Frida functions for resolving function names.
//!
//!The functions in this module correspond to the JavaScript functions
//!grouped under
//![https://frida.re/docs/javascript-api/#apiresolver](https://frida.re/docs/javascript-api/#apiresolver).

use crate::nativepointer::NativePointer;
use std::fmt;

///Resolver type used to initiate [`ApiResolver`](crate::apiresolver::ApiResolver).
pub enum Type {
    ///Resolves exported and imported functions of currently loaded shared
    ///libraries. This is always available.
    Module,
    ///Resolves Objective-C methods of classes currently loaded.
    Objc,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Module => write!(f, "module"),
            Self::Objc => write!(f, "objc"),
        }
    }
}

///Matched values returned by the [`ApiResolver`](crate::apiresolver::ApiResolver).
pub struct Match {
    pub name: String,
    pub address: NativePointer,
}

///ApiResolver object used to find functions by name.
///
///It is recommended to recreate this object immediately before use to avoid
///querying stale data.
#[derive(Debug)]
pub struct ApiResolver(frida_rs_sys::apiresolver::ApiResolver);

impl ApiResolver {
    ///Create a new API resolver object.
    pub fn new(resolver_type: Type) -> Self {
        Self(frida_rs_sys::apiresolver::ApiResolver::new(
            resolver_type.to_string(),
        ))
    }

    ///Resolves the `query` string and returns the matching APIs.
    ///
    ///The syntax for the `query` string depends on the type the resolver is
    ///initiated with. Refer to the Frida documentation for details about the
    ///syntax.
    ///
    ///This is equivalent to calling `ApiResolver.enumerateMatches` in the
    ///JavaScript API.
    pub fn enumerate_matches(&self, query: &str) -> Result<Vec<Match>, js_sys::Error> {
        let s = self.0.enumerate_matches(query.to_owned());
        let s = match s {
            Ok(s) => s,
            Err(e) => {
                // TODO: See if we can wrap this into a custom error type.
                // Probably do not want to expose `js_sys` structs to the
                // public API.
                return Err(js_sys::Error::from(e));
            }
        };

        Ok(s.iter()
            .map(|x| {
                let name =
                    js_sys::Reflect::get(&x, &wasm_bindgen::prelude::JsValue::from_str("name"))
                        .unwrap()
                        .as_string()
                        .unwrap();
                let address = NativePointer::from_jsvalue(
                    js_sys::Reflect::get(&x, &wasm_bindgen::prelude::JsValue::from_str("address"))
                        .unwrap(),
                );
                Match { name, address }
            })
            .collect())
    }
}
