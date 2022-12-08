use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, AttributeArgs, Ident, Lit, NestedMeta, Result, Token};

fn derive_test(input: Vec<NestedMeta>) -> TokenStream2 {
    let sample = match input.get(0).expect("Must provide input for a test") {
        NestedMeta::Lit(txt) => match txt {
            Lit::Str(s) => s.value(),
            _ => unimplemented!(),
        },
        NestedMeta::Meta(_) => unimplemented!(),
    };

    let expected: u32 = match input.get(1).expect("Must provide input for a test") {
        NestedMeta::Lit(txt) => match txt {
            Lit::Int(s) => s.base10_digits(),
            _ => unimplemented!(),
        },
        NestedMeta::Meta(_) => unimplemented!(),
    }
    .parse()
    .expect("Could not parse your expected output");

    let file_path = format!("/tmp/aoc/sample.txt");

    quote! {

        #[test]
        fn aoc_test() {
            use std::fs::Write;
            let mut file = std::fs::File::create(#file_path).unwrap();
            file.write_all(#sample).unwrap();
            let res = Self::run();

            std::fs::remove_file(#file_path).unwrap();

            assert_eq!(res.unwrap(), #expected);

        }

    }
}

#[proc_macro_attribute]
pub fn aoc_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(attr as AttributeArgs);
    let item = proc_macro2::TokenStream::from(item);

    let test = derive_test(args);

    quote! {
        #test

        #item
    }
    .into()
}
