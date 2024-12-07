use rayon::prelude::*;
use std::collections::{BTreeSet, HashSet};

use aoc_core::{aoc_puzzle, tools::Grid, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 6)]
#[derive(Default)]
pub struct Day;

type GuardMove = ((usize, usize), Direction);

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let (grid, guard_pos) = initialize(puzzle);
        let visited = get_visited_cells(&grid, guard_pos);
        visited
            .into_iter()
            .map(|v| v.0)
            .collect::<HashSet<_>>()
            .len()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let (grid, guard_pos) = initialize(puzzle);
        let size = grid.size();
        let walls = grid
            .iter()
            .filter(|c| c.1 == &'#')
            .map(|c| (*c.0 .0, *c.0 .1))
            .collect::<BTreeSet<_>>();
        let visited = get_visited_cells(&grid, guard_pos);
        visited
            .par_iter()
            .enumerate()
            .skip(1)
            .filter_map(|(index, cell)| {
                let current_visited = &visited[..index];
                if current_visited.iter().any(|c| c.0 == cell.0) {
                    // we have already passed through here, putting an obstacle here now will cause a time paradox.
                    return None;
                }
                let current_guard = *current_visited.last().unwrap();
                let current_visited = current_visited.iter().cloned().collect::<BTreeSet<_>>();
                if find_loop(&walls, &size, &cell.0, current_guard, current_visited) {
                    Some(cell.0)
                } else {
                    None
                }
            })
            .count()
            .into()
    }
}

fn initialize(puzzle: &Puzzle) -> (Grid<usize, char>, GuardMove) {
    let mut grid: Grid<usize, char> = puzzle.get_input().into();
    let guard_pos = grid.iter().find(|c| c.1 == &'^').unwrap().0;
    let guard_pos = (*guard_pos.0, *guard_pos.1);
    grid.insert(guard_pos.0, guard_pos.1, '.');
    (grid, (guard_pos, Direction::Up))
}

fn get_visited_cells(grid: &Grid<usize, char>, mut guard: GuardMove) -> Vec<GuardMove> {
    let mut visited = Vec::from([guard]);
    while let Some(next_pos) = guard.1.move_pos(&guard.0) {
        let cell = match grid.get(next_pos.0, next_pos.1) {
            Some(cell) => cell,
            None => break,
        };
        if *cell == '.' {
            guard.0 = next_pos;
            visited.push(guard);
        } else {
            guard.1 = guard.1.rotate();
        }
    }
    visited
}

fn find_loop(
    walls: &BTreeSet<(usize, usize)>,
    (width, height): &(usize, usize),
    extra_obstacle: &(usize, usize),
    mut guard: GuardMove,
    mut visited: BTreeSet<GuardMove>,
) -> bool {
    while let Some(next_pos) = guard.1.move_pos(&guard.0) {
        if next_pos.0 >= *width || next_pos.1 >= *height {
            break;
        }
        if &next_pos == extra_obstacle || walls.contains(&next_pos) {
            visited.insert(guard);
            guard.1 = guard.1.rotate();
        } else {
            guard.0 = next_pos;
        }
        if visited.contains(&guard) {
            return true;
        }
    }
    false
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    pub fn move_pos(&self, pos: &(usize, usize)) -> Option<(usize, usize)> {
        Some(match self {
            Self::Up => (pos.0, pos.1.checked_sub(1)?),
            Self::Right => (pos.0 + 1, pos.1),
            Self::Down => (pos.0, pos.1 + 1),
            Self::Left => (pos.0.checked_sub(1)?, pos.1),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 41.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 6.into());
    }
}
