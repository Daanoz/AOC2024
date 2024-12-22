use std::collections::{HashMap, VecDeque};

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 22)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        puzzle
            .get_input()
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .map(|mut input| {
                for _ in 0..2000 {
                    input = next(input);
                }
                input
            })
            .sum::<usize>()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let mut price_lists: Vec<HashMap<(i8, i8, i8, i8), u8>> = vec![];
        puzzle
            .get_input()
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .for_each(|mut input| {
                let mut map = HashMap::<(i8, i8, i8, i8), u8>::new();
                let mut last_price = input % 10;
                let mut rolling_list: VecDeque<i8> = VecDeque::<i8>::from([last_price as i8]);
                for _ in 0..2000 {
                    input = next(input);

                    let price = input % 10;
                    rolling_list.push_back(price as i8 - last_price as i8);
                    if rolling_list.len() > 4 {
                        rolling_list.pop_front();
                    }
                    if rolling_list.len() == 4 {
                        map.entry((
                            rolling_list[0],
                            rolling_list[1],
                            rolling_list[2],
                            rolling_list[3],
                        ))
                        .or_insert(price as u8);
                    }
                    last_price = price
                }
                price_lists.push(map);
            });

        let mut totals = HashMap::new();
        for price_list in price_lists {
            for (key, value) in price_list {
                *totals.entry(key).or_insert(0) += value as usize;
            }
        }
        totals.values().max().into()
    }
}

const MOD_VALUE: usize = 16777216;
const STEP_1_MUL: usize = 64;
const STEP_2_DIV: usize = 32;
const STEP_3_MUL: usize = 2048;
fn next(input: usize) -> usize {
    // Step 1
    let input = ((input * STEP_1_MUL) ^ input) % MOD_VALUE;
    // Step 2
    let input = ((input / STEP_2_DIV) ^ input) % MOD_VALUE;
    // Step 3
    ((input * STEP_3_MUL) ^ input) % MOD_VALUE
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle_p1() -> Puzzle {
        Puzzle::from(
            r#"1
10
100
2024"#,
        )
    }
    fn get_puzzle_p2() -> Puzzle {
        Puzzle::from(
            r#"1
2
3
2024"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle_p1());
        assert_eq!(result, 37327623.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle_p2());
        assert_eq!(result, 23.into());
    }
}
