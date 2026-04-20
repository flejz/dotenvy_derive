mod codegen;
mod parse;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Derives `impl Default` from `.env` values via `dotenv_codegen`.
/// Add `#[dotenv_static]` to emit `pub const INSTANCE` instead.
///
/// Each field must have `#[dotenv("VAR_NAME")]` and be `&'static str`.
/// Consumer crate must depend on `dotenv_codegen`.
#[proc_macro_derive(Bind, attributes(dotenv, dotenv_static))]
pub fn derive_bind(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let is_static = input
        .attrs
        .iter()
        .any(|a| a.path().is_ident("dotenv_static"));

    let bindings = match parse::parse_derive_input(&input) {
        Ok(b) => b,
        Err(e) => return e.to_compile_error().into(),
    };

    if is_static {
        codegen::emit_static(&input.ident, &bindings).into()
    } else {
        codegen::emit_default(&input.ident, &bindings).into()
    }
}
