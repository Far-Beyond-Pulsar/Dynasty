use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Type, Fields, FieldsNamed, Visibility, parse::Parse};

struct InheritArgs {
    parent_type: Type,
}

impl Parse for InheritArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(InheritArgs {
            parent_type: input.parse()?,
        })
    }
}

#[proc_macro_derive(Class)]
pub fn derive_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl crate::traits::Class for #name {
            fn class_info() -> &'static crate::registry::ClassInfo {
                static INFO: once_cell::sync::Lazy<crate::registry::ClassInfo> = 
                    once_cell::sync::Lazy::new(|| {
                        crate::registry::ClassInfo {
                            id: uuid::Uuid::new_v4(),
                            name: stringify!(#name),
                            parent: None,
                            type_id: std::any::TypeId::of::<#name>(),
                            #[cfg(feature = "reflection")]
                            reflection_data: crate::reflection::ReflectionData::new::<#name>(),
                        }
                    });
                &INFO
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn inherit(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as InheritArgs);
    let mut input = parse_macro_input!(input as DeriveInput);
    let parent_type = &args.parent_type;
    let name = &input.ident;

    // Add the base field to the struct definition
    if let syn::Data::Struct(ref mut struct_data) = input.data {
        if let Fields::Named(ref mut fields) = struct_data.fields {
            // Add the base field as the first field
            let base_field = syn::Field {
                attrs: vec![],
                vis: Visibility::Inherited,
                ident: Some(format_ident!("base")),
                colon_token: Some(Default::default()),
                ty: args.parent_type.clone(),
                mutability: syn::FieldMutability::None,
            };
            fields.named.insert(0, base_field);
        }
    }

    let expanded = quote! {
        #input

        impl crate::traits::Class for #name {
            fn class_info() -> &'static crate::registry::ClassInfo {
                static INFO: once_cell::sync::Lazy<crate::registry::ClassInfo> = 
                    once_cell::sync::Lazy::new(|| {
                        crate::registry::ClassInfo {
                            id: uuid::Uuid::new_v4(),
                            name: stringify!(#name),
                            parent: Some(std::any::TypeId::of::<#parent_type>()),
                            type_id: std::any::TypeId::of::<#name>(),
                            #[cfg(feature = "reflection")]
                            reflection_data: crate::reflection::ReflectionData::new::<#name>(),
                        }
                    });
                &INFO
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }

        impl crate::traits::Inherits<#parent_type> for #name {
            fn as_parent(&self) -> &#parent_type {
                &self.base
            }

            fn as_parent_mut(&mut self) -> &mut #parent_type {
                &mut self.base
            }
        }
    };

    TokenStream::from(expanded)
}