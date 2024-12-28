use proc_macro::{TokenStream, TokenTree};
use quote::*;

#[proc_macro_attribute]
pub fn aoc_day(_params: TokenStream, mut input: TokenStream) -> TokenStream {
    let mut it = input.clone().into_iter();
    let function_name = 'ok: {
        for item in it.by_ref() {
            if let TokenTree::Ident(ident) = item {
                if ident.to_string() == "fn" {
                    let TokenTree::Ident(next_ident) = it.next().unwrap() else {
                        break;
                    };
                    break 'ok proc_macro2::Ident::new(
                        &next_ident.to_string(),
                        next_ident.span().into(),
                    );
                }
            }
        }

        return quote! { compile_error!("Couldn't parse function name") }.into();
    };

    let main = quote! {
        fn main() -> ::anyhow::Result<()> {
            let r = #function_name (open_input(current_day!())?).result()?;
            println!("{} - {}", r.0, r.1);
            Ok(())
        }
    };

    input.extend(TokenStream::from(main));
    input
}
