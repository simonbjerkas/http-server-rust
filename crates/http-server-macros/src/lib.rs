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

/// Create a GET route
///
/// add macro to a function to create a route for your service.
///
/// # Example
///
/// ```
/// #[get("/echo/:msg")]
/// fn message(req: Request, app: &App) -> Response {
///     let Some(msg) = req.param("msg") else {
///         return Response::not_found();
///     };
///
///     let mut headers = Headers::new();
///     headers.set_content_type(ContentType::Text);
///
///     Response::ok(headers, msg)
/// }
/// ```
#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_method_route(quote!(Get), attr, item)
}

/// Create a POST route
///
/// add macro to a function to create a route for your service.
///
/// # Example
///
/// ```
/// #[post("/files/:path")]
/// fn message(req: Request, app: &App) -> Response {
///     let Some(path) = req.param("path") else {
///         return Response::not_found();
///     };
///
///     let path = PathBuf::from(path);
///     let Ok(mut file) = fs::File::create(file_path) else {
///         eprintln!("fails to create file");
///         return Response::bad();
///     };
///
///     if let Err(_) = file.write_all(&req.body) {
///         eprintln!("fails to write file");
///         return Response::bad();
///     }
///
///     Response::created()
/// }
/// ```
#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_method_route(quote!(Post), attr, item)
}

/// Create a middleware
#[proc_macro_attribute]
pub fn middleware(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_inputs = &input.sig.inputs;
    let fn_vis = &input.vis;

    quote! {
        #fn_vis fn #fn_name(#fn_inputs) -> Response #fn_block

        const _: fn(
            ::http_server::Request,
            &::http_server::App,
            ::http_server::middleware::Next<'_>
        ) -> ::http_server::Response = #fn_name;
    }
    .into()
}
