#![feature(proc_macro)]
extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn hello(_attrs: TokenStream, body: TokenStream) -> TokenStream{
    // Parse the string representation
    let ast = syn::parse(body).unwrap();

    // Build the impl
    let gen = impl_hello_world(ast);

    // Return the generated impl
    println!("RESULT CODE -> {}", gen);
    gen.into()
}

fn impl_hello_world(mut ast: syn::Item) -> quote::Tokens {
    if let syn::Item::Fn(ref mut item_fn) = ast {
        let args = &item_fn.decl.inputs;
        let stmt = format!(
            r#"println!("Hello Word!!! :  args tokens = {:?}");"#,
            args.clone().into_tokens()
        );

        item_fn.block.stmts.insert(0, syn::parse_str(&stmt).unwrap());
    }
    ast.into_tokens()
}
