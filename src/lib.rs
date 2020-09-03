extern crate proc_macro;

use image::open;
use proc_macro::TokenStream;
use quote::quote;
use std::env::var;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use syn::{Expr, Lit};

#[proc_macro]
pub fn include_rgba(input: TokenStream) -> TokenStream {
    let path = match syn::parse_macro_input!(input as Expr) {
        Expr::Lit(lit) => match lit.lit {
            Lit::Str(lit) => lit.value(),
            _ => panic!("expected a string literal"),
        },
        _ => panic!("expected a string literal"),
    };

    let image = match open(&path) {
        Ok(i) => i,
        Err(e) => {
            panic!("Can't open image {:?}: {}", path, e);
        }
    };

    let dimensions = image.dimensions();

    let bytes = image.into_rgba().into_raw();

    TokenStream::from(quote! {{
        (
            #dimensions,
            [
                #( #bytes, )*
            ]
        )
    }})
}
