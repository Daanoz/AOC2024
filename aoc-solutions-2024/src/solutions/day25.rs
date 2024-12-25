use std::ops::Deref;

use aoc_core::{aoc_puzzle, tools::Grid, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 25)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let (keys, locks): (Vec<KeyLock>, Vec<KeyLock>) = puzzle
            .input_as_str()
            .split("\n\n")
            .map(|s| {
                let grid: Grid<usize, char> = s.to_string().into();
                let digits = grid
                    .x_range()
                    .unwrap()
                    .map(|x| grid.column(x).filter(|(_, c)| **c == '#').count() as u8)
                    .collect::<Vec<_>>();
                if *grid.get(0, 0).unwrap() == '#' {
                    KeyLock::Key(digits)
                } else {
                    KeyLock::Lock(digits)
                }
            })
            .partition(|kl| match kl {
                KeyLock::Lock(_) => true,
                KeyLock::Key(_) => false,
            });
        let mut count = 0;
        for key in keys {
            for lock in &locks {
                if key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 7) {
                    count += 1;
                }
            }
        }
        count.into()
    }

    fn part2(&self, _puzzle: &Puzzle) -> Answer {
        0.into()
    }
}

#[derive(Debug)]
enum KeyLock {
    Key(Vec<u8>),
    Lock(Vec<u8>),
}

impl Deref for KeyLock {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        match self {
            KeyLock::Key(v) => v,
            KeyLock::Lock(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 3.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 0.into());
    }
}
