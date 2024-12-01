use std::collections::HashMap;

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 1)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let (mut left, mut right) = get_lists(&puzzle.get_input());
        left.sort_unstable();
        right.sort_unstable();
        let sum: u32 = left
            .iter()
            .enumerate()
            .map(|(index, value)| value.abs_diff(right[index]))
            .sum();
        sum.into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let (left, right) = get_lists(&puzzle.get_input());
        let right: HashMap<u32, u32> = right.iter().fold(Default::default(), |mut map, value| {
            *map.entry(*value).or_default() += 1;
            map
        });
        let sum: u32 = left
            .iter()
            .map(|value| value * right.get(value).cloned().unwrap_or_default())
            .sum();
        sum.into()
    }
}

fn get_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| l.split_once("   ").expect("Seperate by space"))
        .map(|(a, b)| {
            (
                a.parse::<u32>().expect("Cannot parse to u32"),
                b.parse::<u32>().expect("Cannot parse to u32"),
            )
        })
        .fold((vec![], vec![]), |lists, pair| {
            let (mut a, mut b) = lists;
            a.push(pair.0);
            b.push(pair.1);
            (a, b)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"3   4
4   3
2   5
1   3
3   9
3   3"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 11.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 31.into());
    }
}
