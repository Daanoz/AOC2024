use std::str::FromStr;

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 14)]
pub struct Day {
    space: (i32, i32),
}
impl Default for Day {
    fn default() -> Self {
        Self { space: (101, 103) }
    }
}

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let (center_x, center_y) = ((self.space.0 - 1) / 2, (self.space.1 - 1) / 2);

        let (top_left, top_right, bottom_left, bottom_right) = puzzle
            .get_input()
            .lines()
            .map(|s| s.parse::<Guard>().unwrap())
            .map(|g| g.get_position_at(100, self.space))
            .fold(
                (vec![], vec![], vec![], vec![]),
                |(mut top_left, mut top_right, mut bottom_left, mut bottom_right), pos| {
                    match (
                        pos.0 < center_x,
                        pos.0 > center_x,
                        pos.1 < center_y,
                        pos.1 > center_y,
                    ) {
                        (true, false, true, false) => top_left.push(pos),
                        (false, true, true, false) => top_right.push(pos),
                        (true, false, false, true) => bottom_left.push(pos),
                        (false, true, false, true) => bottom_right.push(pos),
                        _ => (), // these are on the center line
                    }
                    (top_left, top_right, bottom_left, bottom_right)
                },
            );

        (top_left.len() * top_right.len() * bottom_left.len() * bottom_right.len()).into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let guards = puzzle
            .get_input()
            .lines()
            .map(|s| s.parse::<Guard>().unwrap())
            .collect::<Vec<_>>();
        let (center_x, center_y) = ((self.space.0 - 1) / 2, (self.space.1 - 1) / 2);
        // lets find the where most guards are closest to the center, no clue if this always works
        let lowest = (0..10000)
            .map(|i| {
                let positions = guards
                    .iter()
                    .map(|g| g.get_position_at(i, self.space))
                    .collect::<Vec<_>>();
                let center_sum_x: usize = positions
                    .iter()
                    .map(|p| center_x.abs_diff(p.0) as usize)
                    .sum::<usize>()
                    / positions.len();
                let center_sum_y: usize = positions
                    .iter()
                    .map(|p| center_y.abs_diff(p.1) as usize)
                    .sum::<usize>()
                    / positions.len();
                (i, center_sum_x + center_sum_y)
            })
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        // let mut grid: aoc_core::collections::HashGrid<i32, char> = Default::default();
        // guards
        //     .iter()
        //     .map(|g| g.get_position_at(lowest.0, self.space))
        //     .for_each(|(x, y)| { grid.insert(x, y, '#'); });
        // grid.fill_empty('.');
        // print!("{}\n", grid.to_string());
        lowest.0.into()
    }
}

struct Guard {
    velocity: (i32, i32),
    position: (i32, i32),
}

impl Guard {
    fn get_position_at(&self, second: i32, max: (i32, i32)) -> (i32, i32) {
        (
            (self.position.0 + (self.velocity.0 * second)).rem_euclid(max.0),
            (self.position.1 + (self.velocity.1 * second)).rem_euclid(max.1),
        )
    }
}

impl FromStr for Guard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.strip_prefix("p=").unwrap();
        let (position, velocity) = input.split_once(" v=").unwrap();
        let position = position.split_once(",").unwrap();
        let velocity = velocity.split_once(",").unwrap();
        Ok(Self {
            velocity: (velocity.0.parse().unwrap(), velocity.1.parse().unwrap()),
            position: (position.0.parse().unwrap(), position.1.parse().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#,
        )
    }

    #[test]
    fn part1() {
        let day = Day { space: (11, 7) };
        let result = day.part1(&get_puzzle());
        assert_eq!(result, 12.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
         // Just the outcome for the test for my solution
        assert_eq!(result, 1976.into());
    }
}
