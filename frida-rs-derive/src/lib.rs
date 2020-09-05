extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DeriveCpu)]
pub fn derive_cpu(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let data = &input.data;

    let expanded = match data {
        syn::Data::Struct(s) => {
            let fields = match &s.fields {
                syn::Fields::Named(i) => {
                    let mut fields_call = quote! {};
                    for field in i.named.iter() {
                        let name = field.ident.clone().unwrap();

                        if name == "sys" {
                            continue;
                        }

                        fields_call.extend(quote!{
                            #name: crate::NativePointer::from_jsvalue(js_sys::Reflect::get(&m, &wasm_bindgen::prelude::JsValue::from_str(stringify!(#name))).unwrap()),
                        })
                    }
                    fields_call
                }
                _ => unimplemented!(),
            };

            quote! {
                impl crate::fromsys::FromSys<frida_rs_sys::cpu::CpuContext> for #name {
                    fn from_sys(m: frida_rs_sys::cpu::CpuContext) -> Self {
                        Self {
                            #fields
                            sys: m
                        }
                    }

                    fn into_sys(self) -> frida_rs_sys::cpu::CpuContext {
                        self.sys
                    }
                }
            }
        }
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };

    TokenStream::from(expanded)
}
