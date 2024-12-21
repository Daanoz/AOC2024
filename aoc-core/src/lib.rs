mod answer;
pub use answer::*;
mod puzzle;
pub use puzzle::*;
mod solution_collection;
pub use solution_collection::*;
mod solution_wrapper;
pub use solution_wrapper::*;
mod timer;

pub mod tools;

extern crate aoc_procmacro;
pub use aoc_procmacro::{aoc, aoc_puzzle};
pub use aoc_procmacro_internals::public::*;
