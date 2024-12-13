use std::collections::{BTreeSet, HashSet};

use aoc_core::{aoc_puzzle, tools::Grid, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 12)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let grid: Grid<usize, char> = Grid::from(puzzle.get_input());
        let mut visited = HashSet::new();
        let mut price = 0;
        for ((x, y), c) in grid.iter() {
            if visited.contains(&(*x, *y)) {
                continue;
            }
            let mut area = HashSet::new();
            let mut queue = vec![(*x, *y)];
            while let Some((x, y)) = queue.pop() {
                visited.insert((x, y));
                area.insert((x, y));
                for (nx, ny) in grid.neighbors(x, y) {
                    if let Some(nc) = grid.get(nx, ny) {
                        if nc == c && !visited.contains(&(nx, ny)) {
                            queue.push((nx, ny));
                        }
                    }
                }
            }
            // Fences are anchored by the top left corner of the cell
            let mut fences = HashSet::new();
            for (x, y) in area.iter() {
                if let Some(lx) = x.checked_sub(1) {
                    if !area.contains(&(lx, *y)) {
                        fences.insert((*x, *y, Fence::Vertical));
                    }
                } else {
                    fences.insert((*x, *y, Fence::Vertical));
                }
                if let Some(ty) = y.checked_sub(1) {
                    if !area.contains(&(*x, ty)) {
                        fences.insert((*x, *y, Fence::Horizontal));
                    }
                } else {
                    fences.insert((*x, *y, Fence::Horizontal));
                }
                if !area.contains(&(x + 1, *y)) {
                    fences.insert((x + 1, *y, Fence::Vertical));
                }
                if !area.contains(&(*x, y + 1)) {
                    fences.insert((*x, y + 1, Fence::Horizontal));
                }
            }
            price += fences.len() * area.len();
        }
        price.into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let grid: Grid<usize, char> = Grid::from(puzzle.get_input());
        let mut visited = HashSet::new();
        let mut price = 0;
        for ((x, y), c) in grid.iter() {
            if visited.contains(&(*x, *y)) {
                continue;
            }
            let mut area = HashSet::new();
            let mut queue = vec![(*x, *y)];
            while let Some((x, y)) = queue.pop() {
                visited.insert((x, y));
                area.insert((x, y));
                for (nx, ny) in grid.neighbors(x, y) {
                    if let Some(nc) = grid.get(nx, ny) {
                        if nc == c && !visited.contains(&(nx, ny)) {
                            queue.push((nx, ny));
                        }
                    }
                }
            }
            // Fences are anchored by the top left corner of the cell
            let mut fences = BTreeSet::new();
            for (x, y) in area.iter() {
                if let Some(lx) = x.checked_sub(1) {
                    if !area.contains(&(lx, *y)) {
                        fences.insert((*x, *y, SingleSidedFence::Left));
                    }
                } else {
                    fences.insert((*x, *y, SingleSidedFence::Left));
                }
                if let Some(ty) = y.checked_sub(1) {
                    if !area.contains(&(*x, ty)) {
                        fences.insert((*x, *y, SingleSidedFence::Top));
                    }
                } else {
                    fences.insert((*x, *y, SingleSidedFence::Top));
                }
                if !area.contains(&(x + 1, *y)) {
                    fences.insert((x + 1, *y, SingleSidedFence::Right));
                }
                if !area.contains(&(*x, y + 1)) {
                    fences.insert((*x, y + 1, SingleSidedFence::Bottom));
                }
            }
            let mut new_fences = HashSet::new();
            while let Some((x, y, fence)) = fences.pop_first() {
                let move_delta = fence.delta();
                let (mut cx, mut cy) = (x, y);
                while let (Some(lx), Some(ty)) = (cx.checked_sub(move_delta.0), cy.checked_sub(move_delta.1)) {
                    if !fences.remove(&(lx, ty, fence)) {
                        break;
                    }
                    cx = lx;
                    cy = ty;
                }               
                let (mut cx, mut cy) = (x, y);
                loop {
                    let (rx, by) = (cx + move_delta.0, cy + move_delta.1);
                    if !fences.remove(&(rx, by, fence)) {
                        break;
                    }
                    cx = rx;
                    cy = by;
                }
                new_fences.insert((x, y, fence));
            }
            price += new_fences.len() * area.len();
        }
        price.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Fence {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum SingleSidedFence {
    Top,
    Left,
    Bottom,
    Right
}

impl SingleSidedFence {
    fn delta(&self) -> (usize, usize) {
        match self {
            SingleSidedFence::Top | SingleSidedFence::Bottom => (1, 0),
            SingleSidedFence::Left | SingleSidedFence::Right => (0, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#)
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 1930.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 1206.into());
    }
}