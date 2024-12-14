use std::str::FromStr;

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 13)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        puzzle
            .get_input()
            .split("\n\n")
            .map(|s| s.parse::<Game>().unwrap())
            .filter_map(|g| g.solve())
            .sum::<isize>()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        puzzle
            .get_input()
            .split("\n\n")
            .map(|s| s.parse::<Game>().unwrap())
            .map(|g| g.convert_unit())
            .filter_map(|g| g.solve())
            .sum::<isize>()
            .into()
    }
}

#[derive(Debug)]
struct Game {
    prize: (isize, isize),
    a: (isize, isize),
    b: (isize, isize),
}

impl Game {
    fn solve(&self) -> Option<isize> {
        let deter = self.a.0 * self.b.1 - self.a.1 * self.b.0;
        let count_a = (self.prize.0 * self.b.1 - self.prize.1 * self.b.0) / deter;
        let count_b = (self.a.0 * self.prize.1 - self.a.1 * self.prize.0) / deter;
        if self.prize
            == (
                self.a.0 * count_a + self.b.0 * count_b,
                self.a.1 * count_a + self.b.1 * count_b,
            )
        {
            Some(count_a * 3 + count_b)
        } else {
            None
        }
    }
    fn convert_unit(self) -> Self {
        Self {
            a: self.a,
            b: self.b,
            prize: (self.prize.0 + 10000000000000, self.prize.1 + 10000000000000),
        }
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let a = get_button_values(lines.next().unwrap());
        let b = get_button_values(lines.next().unwrap());

        let (_, vals) = lines.next().unwrap().split_once("X=").unwrap();
        let (x, y) = vals.split_once(", Y=").unwrap();
        let prize = (x.parse().unwrap(), y.parse().unwrap());
        Ok(Game { prize, a, b })
    }
}

fn get_button_values(val: &str) -> (isize, isize) {
    let (_, vals) = val.split_once("X+").unwrap();
    let (x, y) = vals.split_once(", Y+").unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 480.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 875318608908_usize.into());
    }
}
