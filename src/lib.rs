#![doc = include_str!("../README.md")]

use proc_macro::{
    Delimiter,
    TokenStream,
    TokenTree
};

fn separate(item: TokenStream) -> (TokenStream, TokenStream) {
    let mut attributes = Vec::<TokenTree>::new();
    let mut iterator = item.into_iter().fuse();
    loop {
        match [iterator.next(), iterator.next()] {
            [Some(TokenTree::Punct(p)), Some(TokenTree::Group(g))]
                if (p.as_char(), g.delimiter()) == ('#', Delimiter::Bracket) =>
            {
                attributes.extend([p.into(), g.into()])
            }
            mut trees => {
                break (
                    attributes.into_iter().collect(),
                    trees
                        .iter_mut()
                        .flat_map(Option::take)
                        .chain(iterator)
                        .collect()
                );
            }
        };
    }
}

#[proc_macro_attribute]
#[doc = include_str!("../docs/prepend.md")]
pub fn prepend(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let (mut item, rest) = separate(item);
    item.extend(attribute.into_iter().chain(rest));
    item
}

#[proc_macro_attribute]
#[doc = include_str!("../docs/append.md")]
pub fn append(attribute: TokenStream, mut item: TokenStream) -> TokenStream {
    item.extend(attribute);
    item
}
