use syn::{DeriveInput, Error, Field, Fields, LitStr, Result};

pub struct FieldBinding {
    pub ident: syn::Ident,
    pub env_key: String,
}

pub fn parse_derive_input(input: &DeriveInput) -> Result<Vec<FieldBinding>> {
    let fields = match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            Fields::Named(named) => &named.named,
            _ => {
                return Err(Error::new_spanned(
                    &input.ident,
                    "Bind: only named-field structs are supported",
                ));
            }
        },
        _ => {
            return Err(Error::new_spanned(
                &input.ident,
                "Bind: only structs are supported",
            ));
        }
    };

    fields.iter().map(parse_field).collect()
}

fn parse_field(field: &Field) -> Result<FieldBinding> {
    let ident = field
        .ident
        .clone()
        .ok_or_else(|| Error::new_spanned(field, "Bind: unnamed fields not supported"))?;

    let dotenv_attr = field
        .attrs
        .iter()
        .find(|a| a.path().is_ident("dotenv"))
        .ok_or_else(|| {
            Error::new_spanned(
                &ident,
                format!(
                    "Bind: field `{}` missing #[dotenv(\"VAR\")] attribute",
                    ident
                ),
            )
        })?;

    let lit: LitStr = dotenv_attr.parse_args().map_err(|_| {
        Error::new_spanned(
            dotenv_attr,
            "Bind: #[dotenv(...)] expects a string literal, e.g. #[dotenv(\"VAR_NAME\")]",
        )
    })?;

    Ok(FieldBinding {
        ident,
        env_key: lit.value(),
    })
}
