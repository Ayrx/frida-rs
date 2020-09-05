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
                            #name: NativePointer::from_jsvalue(Reflect::get(&m, &JsValue::from_str(stringify!(#name))).unwrap()),
                        })
                    }
                    fields_call
                }
                _ => unimplemented!(),
            };

            quote! {
                impl FromSys<cpu::CpuContext> for #name {
                    fn from_sys(m: cpu::CpuContext) -> Self {
                        Self {
                            #fields
                            sys: m
                        }
                    }

                    fn into_sys(self) -> cpu::CpuContext {
                        self.sys
                    }
                }
            }
        }
        syn::Data::Enum(s) => unimplemented!(),
        syn::Data::Union(s) => unimplemented!(),
    };

    TokenStream::from(expanded)
}
