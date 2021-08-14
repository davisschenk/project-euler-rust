#![feature(proc_macro_span)]
extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use quote::{quote, format_ident};
use syn::{self, Ident, Lit, parse::Parse, parse::ParseStream, Token};

type Result<T> = std::result::Result<T, syn::Error>;

#[derive(Debug)]
struct Problem {
    answer: Lit,
    functions: Vec<Ident>
}

impl Parse for Problem {
    fn parse(input: ParseStream) -> Result<Self> {
        let answer: Lit = input.parse()?;
        let mut functions: Vec<Ident> = vec![];

        if input.peek(Token!(,)) {
            input.parse::<Token!(,)>()?;
        }

        
        while let Ok(t) = input.parse::<Ident>() {
            println!("{:?}", t);
            functions.push(t);


            if input.peek(Token!(,)) {
                input.parse::<Token!(,)>()?;
            }
        }
        
        Ok(Self {
            answer,
            functions
        })
    }
}

#[proc_macro]
pub fn problem(ast: TokenStream) -> TokenStream {
    let problem: Problem = syn::parse(ast).unwrap();

    let test_names = problem.functions.iter().map(|f| format_ident!("test_{}", f));
    let bench_names = problem.functions.iter().map(|f| format_ident!("bench_{}", f));
    let answer = problem.answer;
    let functions = &problem.functions;

    let span = Span::call_site();
    let source = span.source_file();
    let name = source.path().file_stem().unwrap().to_str().unwrap().to_string();
    let mod_name = format_ident!("test_{}", name);

    let expanded = quote! {
        fn main() {
            println!("Answer: {}", #answer);
            #(
                println!("{}: {}", stringify!(#functions), #functions());
            )*
        }

        #[cfg(test)]
        mod #mod_name {
            use super::*;
            use test::Bencher;

            #(
                #[test]
                fn #test_names() {
                    assert_eq!(#functions(), #answer);
                }
                
                #[bench]
                fn #bench_names(b: &mut Bencher) {
                    b.iter(|| test::black_box(#functions()))
                }
            )*
        }
    };

    return expanded.into();
}
