use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
    quote_spanned,
};
use syn::{
    spanned::Spanned,
    Data,
    DataStruct,
    Field,
    Fields,
};

pub fn accessors(attrs: TokenStream, s: synstructure::Structure) -> TokenStream {
    let trait_ident = attrs;

    let struct_ident = s.ast().ident.clone();

    let item = match s.ast().data.clone() {
        Data::Struct(struct_item) => generate_struct(&s, struct_item),
        _ => panic!("Only structs are supported"),
    };

    let fields: Vec<_> = extract_get_fields(s.clone());

    let get_impls = fields.iter().map(|field| {
        let field_ident = field.ident.clone().unwrap();
        let method_ident = format_ident!("get_{}", field_ident);
        let field_type = field.ty.clone();
        let span = field.span();

        quote_spanned! {span =>
            #[ink(message)]
            fn #method_ident(&self) -> #field_type {
                self.data().#field_ident
            }
        }
    });

    let fields: Vec<_> = extract_set_fields(s.clone());

    let set_impls = fields.iter().map(|field| {
        let field_ident = field.ident.clone().unwrap();
        let method_ident = format_ident!("set_{}", field_ident);
        let field_type = field.ty.clone();
        let span = field.span();

        quote_spanned! {span =>
            #[ink(message)]
            fn #method_ident(&mut self, value: #field_type) {
                self.data().#field_ident = value;
            }
        }
    });

    let result = quote! {
        #item

        #[openbrush::trait_definition]
        pub trait #trait_ident : Storage<#struct_ident>{
            #(#get_impls)*
            #(#set_impls)*
        }
    };

    result
}

fn generate_struct(s: &synstructure::Structure, struct_item: DataStruct) -> TokenStream {
    let struct_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let attrs = s.ast().attrs.clone();
    let types = s.ast().generics.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let fields = struct_item
        .clone()
        .fields
        .into_iter()
        .map(|mut field| consume_attrs(&mut field));

    match struct_item.fields {
        Fields::Unnamed(_) => {
            quote! {
                #(#attrs)*
                #vis struct #struct_ident #types #where_closure (
                    #(#fields),*
                );
            }
        }
        _ => {
            quote! {
                #(#attrs)*
                #vis struct #struct_ident #types #where_closure {
                    #(#fields),*
                }
            }
        }
    }
}

fn consume_attrs(field: &mut syn::Field) -> Field {
    let attr = field
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("get") && !a.path.is_ident("set"))
        .cloned()
        .collect();

    field.attrs = attr;

    field.clone()
}

fn extract_get_fields(s: synstructure::Structure) -> Vec<Field> {
    let struct_item = match s.ast().data.clone() {
        Data::Struct(struct_item) => struct_item,
        _ => panic!("Only structs are supported"),
    };

    struct_item
        .fields
        .iter()
        .filter(|field| field.attrs.iter().any(|a| a.path.is_ident("get")))
        .cloned()
        .collect::<Vec<_>>()
}

fn extract_set_fields(s: synstructure::Structure) -> Vec<Field> {
    let struct_item = match s.ast().data.clone() {
        Data::Struct(struct_item) => struct_item,
        _ => panic!("Only structs are supported"),
    };

    struct_item
        .fields
        .iter()
        .filter(|field| field.attrs.iter().any(|a| a.path.is_ident("set")))
        .cloned()
        .collect::<Vec<_>>()
}
