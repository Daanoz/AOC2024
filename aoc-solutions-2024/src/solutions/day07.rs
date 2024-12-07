use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};
use rayon::prelude::*;

#[aoc_puzzle(day = 7)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        puzzle
            .get_input()
            .lines()
            .par_bridge()
            .map( Calibration::from)
            .filter(|c| c.is_valid())
            .map(|c| c.result)
            .sum::<u64>()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        puzzle
            .get_input()
            .lines()
            .par_bridge()
            .map(Calibration::from)
            .filter(|c| c.is_valid_with_concat())
            .map(|c| c.result)
            .sum::<u64>()
            .into()
    }
}

struct Calibration {
    result: u64,
    factors: Vec<u64>,
}

impl Calibration {
    fn is_valid(&self) -> bool {
        let outcomes = self.get_all_outcomes(false);
        outcomes.contains(&self.result)
    }
    fn is_valid_with_concat(&self) -> bool {
        let outcomes = self.get_all_outcomes( true);
        outcomes.contains(&self.result)
    }

    fn get_all_outcomes(&self, with_concat: bool) -> Vec<u64> {
        self.factors.iter().fold(Vec::new(), |outcomes, factor| {
            if outcomes.is_empty() {
                Vec::from([*factor])
            } else {
                let mut new_outcomes = Vec::new();
                for outcome in outcomes {
                    let sum = outcome + factor;
                    if sum <= self.result {
                        new_outcomes.push(sum);
                    }
                    let mul = outcome * factor;
                    if mul <= self.result {
                        new_outcomes.push(mul);
                    }
                    if with_concat {
                        let concat = concat_factors(&outcome, factor);
                        if concat <= self.result {
                            new_outcomes.push(concat);
                        }
                    }
                }
                new_outcomes
            }
        })
    }
}

fn concat_factors(a: &u64, b: &u64) -> u64 {
    (a * next_power_of(*b)) + b
}

fn next_power_of(n: u64) -> u64 {
    let log10 = ((n + 1) as f64).log10();
    let next_power = log10.ceil();
    10_u64.pow(next_power as u32)
}

impl From<&str> for Calibration {
    fn from(s: &str) -> Self {
        let (result, factors) = s.split_once(": ").unwrap();
        Self {
            result: result.parse().unwrap(),
            factors: factors.split(' ').map(|f| f.parse().unwrap()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 3749.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 11387.into());
    }
}
