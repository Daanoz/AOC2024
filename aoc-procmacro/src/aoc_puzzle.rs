use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;

use aoc_procmacro_internals::{get_aoc_data, AocDataType};

use super::aoc::get_year;

mod attributes;

use attributes::{validate_day, AocAttributes};

const MISSING_DAY_ERROR: &str =
    "Could not determine puzzle day. Use one of these methods to define the day:
- use day attribute `#[aoc_puzzle(day = 1)]`
- set a day suffix `struct MySolution01`
";

pub(crate) fn aoc_puzzle_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let input: syn::ItemStruct = match syn::parse2::<syn::ItemStruct>(input) {
        Ok(is) => is,
        Err(e) => return darling::Error::from(e).write_errors(),
    };
    let attr_args = match NestedMeta::parse_meta_list(args) {
        Ok(v) => v,
        Err(e) => return darling::Error::from(e).write_errors(),
    };
    let args = match AocAttributes::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return e.write_errors(),
    };

    let (aoc_day, aoc_year) = match extract_day_year(&input, args.day) {
        Ok(dy) => dy,
        Err(e) => return e.write_errors(),
    };

    let puzzle_description = match get_aoc_data(AocDataType::Text, aoc_day, aoc_year) {
        Ok(description) => description,
        Err(e) => format!(
            "Failed to get puzzle description for day {} ({}): {}",
            aoc_day, aoc_year, e
        ),
    };

    let ident = &input.ident;
    let doc_text = puzzle_description.replace("```", "```text");

    let expanded = quote! {
        #[doc = #doc_text]
        #input

        pub fn register_solution(solutions: &mut aoc_core::SolutionCollection) {
            let wrapper = aoc_core::SolutionWrapper::new(
                #ident::default(),
                aoc_core::SolutionProps {
                    day: #aoc_day,
                    year: #aoc_year,
                }
            );

            solutions.register_solution(Box::new(wrapper));
        }
    };

    expanded
}

fn extract_day_year(
    input: &syn::ItemStruct,
    day_from_arg: Option<u32>,
) -> Result<(u32, u32), darling::Error> {
    let aoc_year = get_aoc_year()?;
    let aoc_day = match validate_day(day_from_arg.or_else(|| get_day_from_name(input)))? {
        Some(day) => day,
        None => return Err(darling::Error::custom(MISSING_DAY_ERROR)),
    };
    Ok((aoc_day, aoc_year))
}

// Attempt to parse the day from the suffix in the name of the struct
fn get_day_from_name(input: &syn::ItemStruct) -> Option<u32> {
    let name = input.ident.to_string();
    // Reverse the string, take all the digits from the end, reverse it back and parse it as u32
    let day = name
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .skip_while(|c| c == &'0')
        .collect::<String>();
    if day.is_empty() {
        return None;
    }
    day.parse().ok()
}

fn get_aoc_year() -> Result<u32, darling::Error> {
    match get_year() {
        Some(year) => Ok(year),
        _ => Err(darling::Error::custom(
            "AOC year not set, call `aoc_core::set_year()`".to_string(),
        )),
    }
}
