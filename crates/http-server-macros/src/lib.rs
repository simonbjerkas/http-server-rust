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

    let public_name = func.sig.ident.clone(); // "health"
    let handler_name = format_ident!("__handler_{}", public_name);
    let routable_name = format_ident!("__Routable_{}", public_name);

    // Rename the user's function to the hidden handler name
    func.sig.ident = handler_name.clone();

    let myserver = quote!(::http_server);

    quote! {
        // hidden handler fn (same body user wrote)
        #func

        #[allow(non_camel_case_types)]
        pub struct #routable_name;

        // This is what you pass to `app.service(health)`
        #[allow(non_upper_case_globals)]
        pub const #public_name: #routable_name = #routable_name;

        impl #myserver::Routable for #routable_name {
            fn route() -> #myserver::Route {
                #myserver::Route {
                    method: #myserver::Method::#method_variant,
                    path: #path,
                    handler: #handler_name,
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
