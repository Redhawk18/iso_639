// what we want
// proc macro that can parse a tsv
// take 2 or 3 code followed by name
// impl display, seralization, to string, maybe from string
//

use std::path::Path;

use csv::ReaderBuilder;
use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;
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
    let path = path.value();
    let path = Path::new(&path);

    println!("{:?}", &path);
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_path(path)
        .expect("Failed to open the file.");

    #[derive(Deserialize)]
    struct Language {
        uri: String,
        code: String,
        english_name: String,
        _french_name: String,
    }

    let items: Vec<Language> = rdr.deserialize().map(|result| result.unwrap()).collect();
    let mut variants = proc_macro2::TokenStream::new();
    for item in items {
        let doc = format!(
            "The {} Language with the code of `{}`. [More infomation can be found on the Library of Congress website]({})",
            item.english_name, item.code, item.uri
        );

        let code = item.code;
        let english_name = item.english_name;

        variants.extend(quote! {
            #[doc = #doc]
            // #[cfg_attr(feature = serde, rename = #code)]
            #english_name,
        });
    }

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum Language {
            #variants
        }
    }
    .into()
}

//         impl std::fmt::Display for Input {
//             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                 write!(f, "{}", "")
//             }
//         }
