use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, LitStr, parse_macro_input};

fn expand_method_route(
    method_variant: proc_macro2::TokenStream,
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let path = parse_macro_input!(attr as LitStr);
    let mut func = parse_macro_input!(item as ItemFn);

    let public_name = func.sig.ident.clone();
    let handler_name = format_ident!("__handler_{}", public_name);

    func.sig.ident = handler_name.clone();

    quote! {
        #func

        #[allow(non_camel_case_types)]
        pub struct #public_name;

        impl ::http_server::IntoRoute for #public_name {
            fn into_route(self) -> ::http_server::Route {
                ::http_server::Route {
                    method: ::http_server::Method::#method_variant,
                    path: String::from(#path),
                    handler: #handler_name,
                    middleware: Vec::new(),
                }
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_method_route(quote!(Get), attr, item)
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_method_route(quote!(Post), attr, item)
}
