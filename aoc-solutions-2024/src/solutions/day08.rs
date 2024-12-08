use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use aoc_core::{aoc_puzzle, tools::Grid, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 8)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let grid: Grid<usize, char> = puzzle.get_input().into();
        get_anti_node_count(grid, true).into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let grid: Grid<usize, char> = puzzle.get_input().into();
        get_anti_node_count(grid, false).into()
    }
}

fn get_anti_node_count(grid: Grid<usize, char>, single: bool) -> usize {
    let bounds: (usize, usize) = grid.size();
    let antennas: HashMap<char, Vec<(usize, usize)>> =
        grid.iter().fold(HashMap::new(), |mut map, (coord, char)| {
            if char != &'.' {
                map.entry(*char).or_default().push((*coord.0, *coord.1));
            }
            map
        });
    antennas
        .into_values()
        .par_bridge()
        .flat_map(|freq_antennas| {
            freq_antennas
                .iter()
                .flat_map(|a| {
                    freq_antennas.iter().filter_map(
                        move |b| {
                            if a == b {
                                None
                            } else {
                                Some((*a, *b))
                            }
                        },
                    )
                })
                .flat_map(|(a, b)| {
                    let dx = b.0 as isize - a.0 as isize;
                    let dy = b.1 as isize - a.1 as isize;
                    let mut anti_nodes = vec![];
                    if !single {
                        anti_nodes.push(a);
                        anti_nodes.push(b);
                    }

                    // Extrapolate in the positive direction
                    let mut x = b.0 as isize + dx;
                    let mut y = b.1 as isize + dy;
                    while x >= 0 && y >= 0 && (x as usize) < bounds.0 && (y as usize) < bounds.1 {
                        anti_nodes.push((x as usize, y as usize));
                        if single {
                            break;
                        }
                        x += dx;
                        y += dy;
                    }

                    // Extrapolate in the negative direction
                    let mut x = a.0 as isize - dx;
                    let mut y = a.1 as isize - dy;
                    while x >= 0 && y >= 0 && (x as usize) < bounds.0 && (y as usize) < bounds.1 {
                        anti_nodes.push((x as usize, y as usize));
                        if single {
                            break;
                        }
                        x += dx;
                        y += dy;
                    }
                    anti_nodes
                })
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#)
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 14.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 34.into());
    }
}
