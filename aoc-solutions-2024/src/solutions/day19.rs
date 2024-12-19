use std::collections::HashMap;

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 19)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let (towels, designs) = parse(puzzle.input_as_str());
        let mut counter = DesignCounter::new();
        designs
            .into_iter()
            .filter_map(|design| (counter.count_designs(design, &towels) > 0).then_some(()))
            .count()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let (towels, designs) = parse(puzzle.input_as_str());
        let mut counter = DesignCounter::new();
        designs
            .into_iter()
            .map(|design| counter.count_designs(design, &towels))
            .sum::<usize>()
            .into()
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels
        .split(", ")
        .map(|t| t.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let designs = designs
        .lines()
        .map(|t| t.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (towels, designs)
}

struct DesignCounter {
    memo: HashMap<Vec<char>, usize>,
}

impl DesignCounter {
    fn new () -> Self {
        Self {
            memo: HashMap::new(),
        }
    }
    fn count_designs(&mut self, full_design: Vec<char>, towels: &Vec<Vec<char>>) -> usize {
        self.count_designs_recursive(&full_design, towels)
    }

    #[inline]
    fn count_designs_recursive(&mut self, design: &[char], towels: &Vec<Vec<char>>) -> usize {
        if design.is_empty() {
            return 1;
        }
        if let Some(c) = self.memo.get(design) {
            return *c;
        }
        let count = towels
            .iter()
            .filter_map(|towel| {
                if towel.len() <= design.len() && *towel == design[..towel.len()] {
                    Some(self.count_designs_recursive(&design[towel.len()..], towels))
                } else {
                    None
                }
            })
            .sum();
        self.memo.insert(design.to_vec(), count);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 6.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 16.into());
    }
}
