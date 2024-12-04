use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};
use regex::Regex;

#[aoc_puzzle(day = 3)]
#[derive(Default)]
pub struct Day;

lazy_static::lazy_static! {
    static ref PART1: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    static ref PART2: Regex = Regex::new(r"(?:(?:(mul)\((\d+),(\d+)\))|(do)\(\)|(don't)\(\))").unwrap();
}

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        PART1.captures_iter(puzzle.input_as_str()).map(|cap| {
            let a = cap[1].parse::<u32>().unwrap();
            let b = cap[2].parse::<u32>().unwrap();
            a * b
        }).sum::<u32>().into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let mut enabled = true;
        PART2.captures_iter(puzzle.input_as_str()).map(|cap| {
            let cmd = cap.get(1).or(cap.get(4)).or(cap.get(5)).expect("Command");
            if cmd.as_str() == "mul" && enabled {
                let a = cap[2].parse::<u32>().unwrap();
                let b = cap[3].parse::<u32>().unwrap();
                return a * b
            } else if cmd.as_str() == "do" {
                enabled = true;
            } else if cmd.as_str() == "don't" {
                enabled = false;
            } 
            0
        }).sum::<u32>().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        let result = Day::default().part1(&Puzzle::from(r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#));
        assert_eq!(result, 161.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&Puzzle::from(r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#));
        assert_eq!(result, 48.into());
    }
}