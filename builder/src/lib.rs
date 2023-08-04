use proc_macro::TokenStream;

use syn::{DeriveInput, parse_macro_input};
use syn::spanned::Spanned;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {

    // eprintln!("input: {:#?}", input);

    let derive_input: DeriveInput = parse_macro_input!(input);
    // eprintln!("derive_input: {:#?}", derive_input);

    match _gen_empty_builder(&derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.into_compile_error().into()
    }
}

// part 1
fn _gen_empty_token_stream(_input: &TokenStream) -> TokenStream {
    TokenStream::new()
}

// part 2
fn _gen_empty_builder(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    //原来的struct名称
    let ident = &input.ident;
    let struct_name = ident.to_string();
    eprintln!("{:#?}", struct_name);
    //拼接Builder XXXBuilder
    let struct_builder_name = format!("{}Builder", struct_name);
    let builder_name_ident = syn::Ident::new(&struct_builder_name, input.span());

    //创建struct
    let ret = quote::quote!(

        pub struct #builder_name_ident{

        }

        impl #struct_name{
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident{

                }
          }
        }
    );
    Ok(ret)
}
