use std::{collections::{HashSet, VecDeque}, str::FromStr};

use aoc_core::{aoc_puzzle, tools::Grid, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 10)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let grid: Grid<usize, u32> = Grid::from_str(puzzle.input_as_str()).expect("Grid");
        grid
            .iter()
            .filter(|(_, &v)| v == 0)
            .map(|(trail_head, _)| find_trail_ends(&grid, &trail_head))
            .sum::<u32>()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let grid: Grid<usize, u32> = Grid::from_str(puzzle.input_as_str()).expect("Grid");
        grid
            .iter()
            .filter(|(_, &v)| v == 0)
            .map(|(trail_head, _)| find_trails(&grid, &trail_head))
            .sum::<u32>()
            .into()    
    }
}

fn find_trail_ends(grid: &Grid<usize, u32>, trail_head: &(&usize, &usize)) -> u32 {
    let mut trail_ends = HashSet::<(usize, usize)>::new();
    let mut queue = VecDeque::from([(*trail_head.0, *trail_head.1, 0)]);
    while let Some(position) = queue.pop_front() {
        let (x, y, z) = position;
        for (nx, ny) in grid.neighbors(x, y) {
            let nz = match grid.get(nx, ny) {
                Some(c) => c,
                None => continue,
            };
            if z + 1 == *nz {
                if *nz == 9 {
                    trail_ends.insert((nx, ny));
                    continue;
                } else {
                    queue.push_back((nx, ny, *nz));
                }
            }
        }
    }
    trail_ends.len() as u32
}


fn find_trails(grid: &Grid<usize, u32>, trail_head: &(&usize, &usize)) -> u32 {
    let mut trail_ends = HashSet::<Vec<(usize, usize)>>::new();
    let mut queue = VecDeque::from([
        vec![(*trail_head.0, *trail_head.1, 0)],
    ]);
    while let Some(path) = queue.pop_front() {
        let (x, y, z) = path.last().unwrap();
        for (nx, ny) in grid.neighbors(*x, *y) {
            let nz = match grid.get(nx, ny) {
                Some(c) => c,
                None => continue,
            };
            if z + 1 == *nz {
                if *nz == 9 {
                    let mut path: Vec<(usize, usize)> = path.iter().map(|(x, y, _)| (*x, *y)).collect();
                    path.push((nx, ny));
                    trail_ends.insert(path);
                    continue;
                } else {
                    let mut new_path = path.clone();
                    new_path.push((nx, ny, *nz));
                    queue.push_back(new_path);
                }
            }
        }
    }
    trail_ends.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#,
        )
    }

    fn get_puzzle_simple() -> Puzzle {
        Puzzle::from(
            r#"8880888
8881888
8882888
6543456
7111117
8166618
9166619"#,
        )
    }

    #[test]
    fn simple() {
        let result = Day::default().part1(&get_puzzle_simple());
        assert_eq!(result, 2.into());
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 36.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 81.into());
    }
}
