// what we want
// proc macro that can parse a tsv
// take 2 or 3 code followed by name
// impl display, seralization, to string, maybe from string
//

use csv::ReaderBuilder;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use serde::Deserialize;
use std::path::Path;
use syn::{
    LitStr,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Comma,
};

struct Input {
    path: LitStr,
}

impl Parse for Input {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let path = input.parse()?;
        let _: Option<Comma> = input.parse()?;

        Ok(Self { path })
    }
}

#[proc_macro]
pub fn language(input: TokenStream) -> TokenStream {
    let Input { path } = parse_macro_input!(input as Input);
    // let path = path.span().local;
    let path = path
        .span()
        .local_file()
        .expect("Failed to find file.")
        .parent()
        .expect("Failed to get parent.")
        .join(path.value());

    println!("path {:?}", &path);
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_path(path)
        .expect("Failed to open the file.");

    #[derive(Debug, Deserialize)]
    struct Language {
        #[serde(rename = "URI")]
        uri: String,
        code: String,
        #[serde(rename = "Label (English)")]
        english_name: String,
        #[serde(rename = "Label (French)")]
        _french_name: String,
    }

    let items: Vec<Language> = rdr
        .deserialize()
        .map(|result| {
            // dbg!("{}", &result);
            result.unwrap()
        })
        .collect();
    let mut variants = proc_macro2::TokenStream::new();
    let mut write = proc_macro2::TokenStream::new();
    for item in items {
        let doc = format!(
            " The {} language with the code of `{}`. [More infomation can be found on the Library of Congress website]({})",
            item.english_name.replace("|", "or"),
            item.code,
            item.uri
        );

        let code = item.code;
        let english_name = item
            .english_name
            .replace("ca.", "")
            .replace("'", "")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "")
            .replace(".", "")
            .replace(" ", "") // TODO capitial each word with space.
            .replace("-", "_")
            // Non-standard `-`
            .replace("‑based", "Based")
            .replace("|", "Or");
        // dbg!("{}", &english_name);

        let ident = Ident::new(&english_name, Span::call_site());
        variants.extend(quote! {
            #[doc = #doc]
            #[serde(rename = #code)]
            #ident,
        });

        write.extend(quote! {
            Language::#ident => #code,
        });
    }

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        pub enum Language {
            #variants
        }

        impl std::fmt::Display for Language {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    #write
                })
            }
        }

    }
    .into()
}
