mod codegen;
mod parse;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Derives struct initialization from `.env` values at compile time.
///
/// Each field must carry `#[dotenv("VAR_NAME")]` and be typed `&'static str`.
/// The consumer crate must also depend on `dotenv_codegen`.
///
/// # Modes
///
/// ## `impl Default` (default)
///
/// ```rust,ignore
/// #[derive(Bind)]
/// pub struct ZoomConfig {
///     #[dotenv("ZOOM_APP_KEY")]
///     pub app_key: &'static str,
///     #[dotenv("ZOOM_APP_SECRET")]
///     pub app_secret: &'static str,
/// }
///
/// let cfg = ZoomConfig::default();
/// ```
///
/// ## `pub const INSTANCE` (with `#[dotenv_static]`)
///
/// Add `#[dotenv_static]` to emit a compile-time constant usable in `const` contexts:
///
/// ```rust,ignore
/// #[derive(Bind)]
/// #[dotenv_static]
/// pub struct ZoomConfig {
///     #[dotenv("ZOOM_APP_KEY")]
///     pub app_key: &'static str,
/// }
///
/// pub const CONFIG: AppConfig = AppConfig {
///     zoom: ZoomConfig::INSTANCE,
/// };
/// ```
///
/// # Errors
///
/// Compile error if:
/// - Applied to an enum, union, tuple struct, or unit struct
/// - Any field is missing `#[dotenv("VAR_NAME")]`
/// - `#[dotenv(...)]` argument is not a string literal
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
