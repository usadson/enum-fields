// Copyright (C) 2023 - 2025 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! # enum-fields
//! Quickly access shared enum fields in Rust.
//!
//! ## Example
//! The following example showcases an enum `Entity`, which contains two
//! variants: `Company` and `Person`.
//!
//! ```rs
//! /// An entity that can be either a `Company` or a `Person`.
//! #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, enum_fields::EnumFields)]
//! pub enum Entity {
//!     Company {
//!         name: String,
//!         ceo: String,
//!     },
//!
//!     Person {
//!         name: String,
//!     }
//! }
//! ```
//!
//! ### Field Accessor Functions (Getters)
//! Since `Entity` derives from [`enum_fields::EnumFields`], it now contains
//! two field accessor functions (getters): `Entity::name()` and
//! `Entity::ceo()`.
//!
//! ```rs
//! let mut company = Entity::Company {
//!     name: "Apple".into(),
//!     ceo: "Tim Cook".into()
//! };
//!
//! let person = Entity::Person {
//!     name: "Tim Berners-Lee".into()
//! };
//!
//! println!("Company with CEO: {} named: {}",
//!     company.ceo().unwrap(),
//!     company.name()
//! );
//!
//! println!("Person named: {}", person.name());
//! ```
//!
//! ### Shared Fields
//! Note that both `Company` and `Person` have a field named `name`. This
//! enforces `enum-fields` to let `Entity::name()` return the type directly.
//!
//! ```rs
//! // Since [`Entity`] has two variants that both have the `name` field,
//! // `Entity::name(&self)` returns the `&String`.
//! assert_eq!(company.name(), "Apple");
//! assert_eq!(person.name(), "Tim Berners-Lee");
//! ```
//!
//! ### Shared Fields (Optional)
//! However, only `Company` has field `ceo`, which therefore makes
//! `Entity::ceo()` return an optional getter: `Option<&String>`.
//!
//! ```rs
//! // Only `Company` has field `ceo`, so it returns an `Option<&String>`,
//! // since a `Person` returns [`None`].
//! assert_eq!(company.ceo(), Some(&"Tim Cook".into()));
//! assert_eq!(person.ceo(), None);
//!
//! if let Some(ceo) = company.ceo_mut() {
//!     ceo.push_str(" ?!");
//! }
//! assert_eq!(company.ceo(), Some(&"Tim Cook ?!".into()));
//!
//! *company.name_mut() = "Microsoft".into();
//! assert_eq!(company.name(), "Microsoft");
//! ```

use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn;

#[proc_macro_derive(EnumFields)]
pub fn enum_fields_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_for_input(&ast)
}

fn collect_available_fields(enum_data: &syn::DataEnum) -> HashMap<String, Vec<&syn::Field>> {
    let mut fields = HashMap::new();

    for variant in &enum_data.variants {
        for field in &variant.fields {
            if let Some(field_ident) = &field.ident {
                let ident = field_ident.to_string();
                fields.entry(ident)
                    .or_insert(Vec::new())
                    .push(field);
            }
        }
    }

    fields
}

fn impl_for_input(ast: &syn::DeriveInput) -> TokenStream {
    let fail_message = "`EnumFields` is only applicable to `enum`s";
    match &ast.data {
        syn::Data::Enum(data_enum) => impl_for_enum(ast, &data_enum),
        syn::Data::Union(data_union) => syn::Error::new(data_union.union_token.span, fail_message).to_compile_error().into(),
        syn::Data::Struct(data_struct) => syn::Error::new(data_struct.struct_token.span, fail_message).to_compile_error().into(),
    }
}

fn impl_for_enum(ast: &syn::DeriveInput, enum_data: &syn::DataEnum) -> TokenStream {
    let name = &ast.ident;

    // Collect available fields
    let fields = collect_available_fields(enum_data);

    let mut data = proc_macro2::TokenStream::new();


    for (field_name, fields) in fields {
        let field_present_everywhere = fields.len() == enum_data.variants.len()
            && fields.iter().all(|x| x.ty == fields[0].ty);

        let generics = &ast.generics;
        let field_type = &fields[0].ty;
        let field_name_ident = Ident::new(&field_name, Span::call_site());
        let field_name_ident_mut = Ident::new(&format!("{field_name}_mut"), Span::call_site());

        let mut variants = proc_macro2::TokenStream::new();

        for variant in &enum_data.variants {
            let name = &variant.ident;

            let variant_field = variant.fields.iter()
                .find(|variant_field| {
                    if let Some(variant_field_ident) = &variant_field.ident {
                        if variant_field_ident.to_string() == field_name {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });

            let variant_field_ident = variant_field.as_ref().and_then(|field| field.ident.as_ref());

            match variant_field_ident {
                Some(variant_field_ident) => {
                    variants.extend(quote! {
                        Self::#name{ #variant_field_ident, .. } => (#variant_field_ident).into(),
                    });
                }

                None => {
                    // Field not present in field list.
                    if let Some(first_field) = variant.fields.iter().next() {
                        if first_field.ident.is_some() {
                            variants.extend(quote! {
                                Self::#name{ .. } => None,
                            });
                        } else {
                            variants.extend(quote! {
                                Self::#name(..) => None,
                            });
                        }
                    } else {
                        variants.extend(quote! {
                            Self::#name => None,
                        });
                    }
                }
            }
        }

        let ty = if field_present_everywhere {
            quote! {
                & #field_type
            }
        } else {
            quote! {
                Option<& #field_type>
            }
        };

        let ty_mut = if field_present_everywhere {
            quote! {
                &mut #field_type
            }
        } else {
            quote! {
                Option<&mut #field_type>
            }
        };

        data.extend(quote! {
            impl #generics #name #generics {
                pub fn #field_name_ident(&self) -> #ty {
                    //! Get the property of this enum discriminant if it's available
                    match self {
                        #variants
                    }
                }

                 pub fn #field_name_ident_mut(&mut self) -> #ty_mut {
                    //! Get the mutable property of this enum discriminant if it's available
                    match self {
                        #variants
                    }
                }
            }
        });
    }

    data.into()
}
