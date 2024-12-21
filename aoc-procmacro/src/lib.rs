use proc_macro::TokenStream;

mod aoc;
mod aoc_puzzle;

/// Derive macro for the AocPuzzles
///
/// Retrieves puzzle input for a day of the puzzle
///
/// Usage:
/// ```no_compile
/// #[aoc_puzzle(day = 1)]
/// #[derive(Default)]
/// struct Day;
///
/// impl PuzzleSolution for Day {
///     fn part1(&self, puzzle: &Puzzle) -> Answer {
///         ().into()
///     }
///     fn part2(&self, puzzle: &Puzzle) -> Answer {
///         ().into()
///     }
/// }
/// ```
///
/// Before you can call this macro in your code, you need to set `#[aoc(year = 2024)] on your main func`.
#[proc_macro_attribute]
pub fn aoc_puzzle(args: TokenStream, input: TokenStream) -> TokenStream {
    aoc_puzzle::aoc_puzzle_impl(args.into(), input.into()).into()
}

/// Derive macro for the Aoc Solution
///
/// Sets the current year for the AocPuzzles
///
/// Usage:
/// ```no_compile
/// #[aoc(year = 2024)]
/// fn main() {
/// }
/// ```
#[proc_macro_attribute]
pub fn aoc(args: TokenStream, input: TokenStream) -> TokenStream {
    aoc::aoc_impl(args.into(), input.into()).into()
}
