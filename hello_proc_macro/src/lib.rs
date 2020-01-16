extern crate proc_macro;

use quote::quote;

#[proc_macro_attribute]
pub fn hello(_attrs: proc_macro::TokenStream, body: proc_macro::TokenStream) -> proc_macro::TokenStream{
    // Parse the string representation
    let ast = syn::parse(body).unwrap();

    // Build the impl
    let gen = impl_hello_world(ast);

    println!(r#"
--------------------- RESULT CODE ---------------------
{}
-------------------------------------------------------"#, gen);
    // Return the generated impl
    gen.into()
}

fn impl_hello_world(item: syn::ItemFn) -> proc_macro2::TokenStream {
    let vis = &item.vis;
    let attrs = &item.attrs;
    let sig = &item.sig;
    let block = &item.block;

    let args = item.sig.inputs.iter()
        .filter_map(maybe_ident);

    quote! {
        #(#attrs)*
        #vis #sig {
            #( 
                println!("{} = {}", stringify!(#args), #args);
            )*
            #block
        }
    }
}

fn maybe_ident(arg: &syn::FnArg) -> Option<&syn::Ident> {
    match arg {
        syn::FnArg::Typed(syn::PatType { pat, .. }) => match pat.as_ref() {
            syn::Pat::Ident(ident) => Some(&ident.ident),
            _ => None,
        },
        _ => None,
    }
}