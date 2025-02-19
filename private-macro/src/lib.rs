use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{DataStruct, DeriveInput, FieldsNamed, Ident, parse_macro_input};

fn generated_methods(ast: &DeriveInput) -> Vec<TokenStream2> {
    let named_fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Only works for structs with named fields"),
    };

    named_fields
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().take().unwrap();
            let type_name = &f.ty;
            let method_name = Ident::new(&format!("get_{field_name}"), Span::call_site());

            quote!(
            fn #method_name(&self)->&#type_name{
                &self.#field_name
                }
            )
        })
        .collect()
}

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let item_as_stream: TokenStream2 = item.clone().into();
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let methods = generated_methods(&ast);

    quote! (
        #item_as_stream
        impl #name{
            #(#methods)*
        }
    )
    .into()
}
