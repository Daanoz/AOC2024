use std::collections::HashSet;

use aoc_core::{aoc_puzzle, Answer, tools::Grid, Puzzle, PuzzleSolution};
use regex::Regex;

#[aoc_puzzle(day = 4)]
#[derive(Default)]
pub struct Day;

lazy_static::lazy_static! {
    static ref PART1: Regex = Regex::new(r"(?:XMAS|SAMX)").unwrap();
    static ref PART2: Regex = Regex::new(r"(?:MAS|SAM)").unwrap();
}

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let mut grid: Grid<usize, char> = puzzle.get_input().into();
        let mut count = 0;
        for l in 0..2 {
            for y in grid.y_range().unwrap() {
                let row: String = grid.row_sorted(y).map(|(_, c)| c).collect();
                let mut start = 0;
                while let Some(m) = PART1.find_at(&row, start) {
                    count += 1;
                    start = m.end() - 1; // Last character of the match could also be the first of the next
                }              
            }
            for x in grid.x_range().unwrap() {
                let column: String = grid.column_sorted(x).map(|(_, c)| c).collect();
                let mut start = 0;
                while let Some(m) = PART1.find_at(&column, start) {
                    count += 1;
                    start = m.end() - 1; // Last character of the match could also be the first of the next
                }              
            }
            if l == 0 {
                grid.to_diagonal();
            }
        }
        count.into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let mut grid: Grid<usize, char> = puzzle.get_input().into();
        grid.to_diagonal();
        let mut centers = HashSet::<(usize, usize)>::new();
        for y in grid.y_range().unwrap() {
            let row = grid.row_sorted(y);
            let (row_index, row_str): (Vec<_>, String) = row.fold((vec![], String::new()), |(mut row_index, mut row_str), (index, c) | {
                row_index.push(index);
                row_str.push(*c);
                (row_index, row_str)
            });
            let mut start = 0;
            while let Some(m) = PART2.find_at(&row_str, start) {
                centers.insert((*row_index[m.start() + 1], y));
                start = m.end() - 1; // Last character of the match could also be the first of the next
            }              
        }
        let mut count = 0;
        for x in grid.x_range().unwrap() {
            let column = grid.column_sorted(x);
            let (column_index, column_str): (Vec<_>, String) = column.fold((vec![], String::new()), |(mut column_index, mut column_str), (index, c) | {
                column_index.push(index);
                column_str.push(*c);
                (column_index, column_str)
            });
            let mut start = 0;
            while let Some(m) = PART2.find_at(&column_str, start) {
                if centers.contains(&(x, *column_index[m.start() + 1])) {
                    count += 1;
                }
                start = m.end() - 1; // Last character of the match could also be the first of the next
            }              
        }
        count.into()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#)
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 18.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 9.into());
    }
}
