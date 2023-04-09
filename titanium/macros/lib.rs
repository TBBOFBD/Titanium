//! This crate contains the proc macros used by the titanium crate.

#![deny(missing_docs,missing_debug_implementations,unused,clippy::all)]

#[doc(hidden)]
#[allow(unused_extern_crates)]
extern crate proc_macro;
#[doc(hidden)]
use proc_macro::TokenStream;
#[doc(hidden)]
use quote::{quote, quote_spanned};
#[doc(hidden)]
use syn::spanned::Spanned;

/// Allows you to have a main function that is async.
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    if name != "main" {
        return TokenStream::from(quote_spanned! { name.span() =>
            compile_error!("only the main function can be tagged with #[async_std::main]"),
        });
    }

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the async keyword is missing from the function declaration"),
        });
    }

    let result = quote! {
        #vis fn main() #ret {
            #(#attrs)*
            async fn main(#inputs) #ret {
                #body
            }

            titanium::__internals__::run_main(async {
                main().await
            })
        }

    };

    result.into()
}