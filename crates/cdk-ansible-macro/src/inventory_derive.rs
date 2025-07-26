use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    Data, DataStruct, DeriveInput, Error, Fields, FieldsNamed, Type, parse_macro_input,
    spanned::Spanned as _,
};

/// This derive macro adds `inventory_vars` method to a struct.
/// Supposed to be used in [`cdk_ansible::AppL1`], no need in L2.
/// See [`cdk_ansible::HostInventoryVars`] doc for more details.
pub fn vars_gen_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        Data::Struct(_) | Data::Enum(_) | Data::Union(_) => {
            return Error::new(input.ident.span(), "Only structs are supported")
                .into_compile_error()
                .into();
        }
    };

    let field_calls: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            #[expect(clippy::wildcard_enum_match_arm, reason = "don't check here")]
            match &field.ty {
                Type::Path(path) => {
                    match path.path.segments.last().map(|seg| &seg.ident) {
                        // FIXME: may need full path check
                        Some(ident) if ident == "RefCell" => {
                            quote_spanned! {
                                field.span() => ::cdk_ansible::get_host_inventory_vars_ref_cell(&self.#field_name)
                            }
                        }
                        Some(ident) if ident == "Rc" => {
                            quote_spanned! {
                                field.span() => ::cdk_ansible::get_host_inventory_vars_rc(&self.#field_name)
                            }
                        }
                        _ => {
                            quote_spanned! {
                                field.span() => ::cdk_ansible::get_host_inventory_vars(&self.#field_name)
                            }
                        }
                    }
                }
                _ => quote_spanned! {
                    field.span() => ::cdk_ansible::get_host_inventory_vars(&self.#field_name)
                },
            }
        })
        .collect();

    let token = quote! {
        #[automatically_derived]
        impl #name {
            pub fn inventory_vars(&self) -> ::anyhow::Result<Vec<::cdk_ansible::HostInventoryVars>> {
                [#(#field_calls),*].into_iter().collect::<::anyhow::Result<Vec<_>>>()
            }
        }
    };

    token.into()
}
