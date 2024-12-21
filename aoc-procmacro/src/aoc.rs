use std::sync::Mutex;

use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::TokenStream;

mod attributes;

use attributes::AocAttributes;

lazy_static::lazy_static! {
    static ref AOC_YEAR: Mutex<Option<u32>> = Mutex::new(None);
}

pub fn get_year() -> Option<u32> {
    *AOC_YEAR.lock().unwrap()
}

pub(crate) fn aoc_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args) {
        Ok(v) => v,
        Err(e) => return darling::Error::from(e).write_errors(),
    };
    let args = match AocAttributes::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return e.write_errors(),
    };

    let mut val = AOC_YEAR.lock().unwrap();
    *val = Some(args.year);

    input
}
