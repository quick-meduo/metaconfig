//! Provides a derive macro that implements `ConfigTrait` trait.
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;


use proc_macro::TokenStream;
use std::io::Error;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, DeriveInput, Field, Fields, Ident, Lit, Meta, NestedMeta};

#[proc_macro_derive(MetaConfig, attributes(value))]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_meta_config(&derive_input);
    gen.into()
}

fn impl_meta_config(input: &DeriveInput) -> proc_macro2::TokenStream {
    use syn::Data::*;
    let struct_name = &input.ident;

    let inner_impl = match input.data {
        Struct(ref ds) => match ds.fields {
            Fields::Named(ref fields) => impl_meta_config_for_struct(struct_name, &fields.named),
            _ => panic!("MetaConfig supports only named fields"),
        },
        _ => panic!("MetaConfig only supports non-tuple structs"),
    };

    quote!(#inner_impl)
}

fn impl_meta_config_for_struct(
    struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {

    quote! {
        impl MetaConfig for #struct_name {
            fn init() -> ::std::result::Result<Self, Error> {
                Ok(self)
            }
        }
    }
}

// fn gen_field_assign(field: &Field, source: Source) -> proc_macro2::TokenStream {
//     let attr = fetch_envconfig_attr_from_field(field);
//
//     if let Some(attr) = attr {
//         // if #[value(...)] is there
//         let list = fetch_list_from_attribute(field, attr);
//
//         // If nested attribute is present
//         let nested_value_opt = find_item_in_list(field, &list, "nested");
//         if nested_value_opt.is_some() {
//             return gen_field_assign_for_struct_type(field, source);
//         }
//
//         let opt_default = find_item_in_list(field, &list, "default");
//
//         let from_opt = find_item_in_list(field, &list, "from");
//         let env_var = match from_opt {
//             Some(v) => quote! { #v },
//             None => field_to_env_var(field),
//         };
//
//         geen(field, env_var, opt_default, source)
//     } else {
//         // if #[value(...)] is not present
//         let env_var = field_to_env_var(field);
//         geen(field, env_var, None, source)
//     }
// }

// fn field_to_env_var(field: &Field) -> proc_macro2::TokenStream {
//     let field_name = field.clone().ident.unwrap().to_string().to_uppercase();
//     quote! { #field_name }
// }
//
// fn geen(
//     field: &Field,
//     from: proc_macro2::TokenStream,
//     opt_default: Option<&Lit>,
//     source: Source,
// ) -> proc_macro2::TokenStream {
//     let field_type = &field.ty;
//     if to_s(field_type).starts_with("Option ") {
//         gen_field_assign_for_optional_type(field, from, opt_default, source)
//     } else {
//         gen_field_assign_for_non_optional_type(field, from, opt_default, source)
//     }
// }
//
// fn gen_field_assign_for_struct_type(field: &Field, source: Source) -> proc_macro2::TokenStream {
//     let ident = &field.ident;
//     match &field.ty {
//         syn::Type::Path(path) => match source {
//             Source::Environment => quote! {
//                 #ident: #path :: init_from_env()?
//             },
//             Source::HashMap => quote! {
//                 #ident: #path :: init_from_hashmap(hashmap)?
//             },
//         },
//         _ => panic!("Expected field type to be a path: {:?}", ident),
//     }
// }
//
// fn gen_field_assign_for_optional_type(
//     field: &Field,
//     from: proc_macro2::TokenStream,
//     opt_default: Option<&Lit>,
//     source: Source,
// ) -> proc_macro2::TokenStream {
//     let ident = &field.ident;
//
//     if opt_default.is_some() {
//         panic!("Optional type on field `{}` with default value does not make sense and therefore is not allowed", to_s(ident));
//     }
//
//     match source {
//         Source::Environment => quote! {
//                 #ident: ::envconfig::load_optional_var(#from, None)?
//         },
//         Source::HashMap => quote! {
//             #ident: ::envconfig::load_optional_var(#from, Some(hashmap))?
//         },
//     }
// }
//
// fn gen_field_assign_for_non_optional_type(
//     field: &Field,
//     from: proc_macro2::TokenStream,
//     opt_default: Option<&Lit>,
//     source: Source,
// ) -> proc_macro2::TokenStream {
//     let ident = &field.ident;
//
//     if let Some(default) = opt_default {
//         match source {
//             Source::Environment => quote! {
//                 #ident: ::envconfig::load_var_with_default(#from, None, #default)?
//             },
//             Source::HashMap => quote! {
//                 #ident: ::envconfig::load_var_with_default(#from, Some(hashmap), #default)?
//             },
//         }
//     } else {
//         match source {
//             Source::Environment => quote! {
//                 #ident: ::envconfig::load_var(#from, None)?
//             },
//             Source::HashMap => quote! {
//                 #ident: ::envconfig::load_var(#from, Some(hashmap))?
//             },
//         }
//     }
// }
//
// fn fetch_envconfig_attr_from_field(field: &Field) -> Option<&Attribute> {
//     field.attrs.iter().find(|a| {
//         let path = &a.path;
//         let name = quote!(#path).to_string();
//         name == "envconfig"
//     })
// }

// fn fetch_list_from_attribute(field: &Field, attr: &Attribute) -> Punctuated<NestedMeta, Comma> {
//     let opt_meta = attr.parse_meta().unwrap_or_else(|err| {
//         panic!(
//             "Can not interpret meta of `value` attribute on field `{}`: {}",
//             field_name(field),
//             err
//         )
//     });
//
//     match opt_meta {
//         Meta::List(l) => l.nested,
//         _ => panic!(
//             "`value` attribute on field `{}` must contain a list",
//             field_name(field)
//         ),
//     }
// }

// fn find_item_in_list<'l, 'n>(
//     field: &Field,
//     list: &'l Punctuated<NestedMeta, Comma>,
//     item_name: &'n str,
// ) -> Option<&'l Lit> {
//     list.iter()
//         .map(|item| match item {
//             NestedMeta::Meta(meta) => match meta {
//                 Meta::NameValue(name_value) => name_value,
//                 _ => panic!(
//                     "`envconfig` attribute on field `{}` must contain name/value item",
//                     field_name(field)
//                 ),
//             },
//             _ => panic!(
//                 "Failed to process `envconfig` attribute on field `{}`",
//                 field_name(field)
//             ),
//         })
//         .find(|name_value| name_value.path.is_ident(item_name))
//         .map(|item| &item.lit)
// }
//
// fn field_name(field: &Field) -> String {
//     to_s(&field.ident)
// }
//
// fn to_s<T: quote::ToTokens>(node: &T) -> String {
//     quote!(#node).to_string()
// }
