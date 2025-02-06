use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, ImplItem, ItemImpl, ItemStruct, LitStr};

// MIT License
// Copyright (c) 2021-2023 Astral Sh
// https://github.com/astral-sh/uv/blob/cfd1e670ddb803f4e67d4abd069fad271e1d1c7f/crates/uv-macros/src/lib.rs
fn get_doc_comment(attrs: &[Attribute]) -> String {
    attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(meta) = &attr.meta {
                    if let syn::Expr::Lit(expr) = &meta.value {
                        if let syn::Lit::Str(str) = &expr.lit {
                            return Some(str.value().trim().to_string());
                        }
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// MIT License
// Copyright (c) 2021-2023 Astral Sh
// https://github.com/astral-sh/uv/blob/cfd1e670ddb803f4e67d4abd069fad271e1d1c7f/crates/uv-macros/src/lib.rs
fn get_env_var_pattern_from_attr(attrs: &[Attribute]) -> Option<String> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("attr_env_var_pattern"))
        .and_then(|attr| attr.parse_args::<LitStr>().ok())
        .map(|lit_str| lit_str.value())
}

// MIT License
// Copyright (c) 2021-2023 Astral Sh
// https://github.com/astral-sh/uv/blob/cfd1e670ddb803f4e67d4abd069fad271e1d1c7f/crates/uv-macros/src/lib.rs
fn is_hidden(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("attr_hidden"))
}

// MIT License
// Copyright (c) 2021-2023 Astral Sh
// https://github.com/astral-sh/uv/blob/cfd1e670ddb803f4e67d4abd069fad271e1d1c7f/crates/uv-macros/src/lib.rs
//
/// This attribute is used to generate environment variables metadata for [`cdk_ansible_static::EnvVars`].
#[proc_macro_attribute]
pub fn attribute_env_vars_metadata(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemImpl);

    let constants: Vec<_> = ast
        .items
        .iter()
        .filter_map(|item| match item {
            ImplItem::Const(item) if !is_hidden(&item.attrs) => {
                let name = item.ident.to_string();
                let doc = get_doc_comment(&item.attrs);
                Some((name, doc))
            }
            ImplItem::Fn(item) if !is_hidden(&item.attrs) => {
                // Extract the environment variable patterns.
                if let Some(pattern) = get_env_var_pattern_from_attr(&item.attrs) {
                    let doc = get_doc_comment(&item.attrs);
                    Some((pattern, doc))
                } else {
                    None // Skip if pattern extraction fails.
                }
            }
            _ => None,
        })
        .collect();

    let struct_name = &ast.self_ty;
    let pairs = constants.iter().map(|(name, doc)| {
        quote! {
            (#name, #doc)
        }
    });

    let expanded = quote! {
        #ast

        impl #struct_name {
            /// Returns a list of pairs of env var and their documentation defined in this impl block.
            pub fn metadata<'a>() -> &'a [(&'static str, &'static str)] {
                &[#(#pairs),*]
            }
        }
    };

    expanded.into()
}

// https://stackoverflow.com/questions/54177438/how-to-programmatically-get-the-number-of-fields-of-a-struct
#[proc_macro_derive(FieldCount)]
pub fn derive_field_count(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let field_count = input.fields.iter().count();
    let name = &input.ident;
    let output = quote! {
      impl #name {
        pub fn field_count() -> usize {
          #field_count
        }
      }
    };
    TokenStream::from(output)
}
