use std::collections::{HashMap, HashSet, VecDeque};

use aoc_core::{aoc_puzzle, tools::Grid, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 16)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let (dist_map, end) = create_distance_map_with_dijkstra(puzzle.get_input());
        get_distance(&dist_map, end).unwrap().into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let (dist_map, end) = create_distance_map_with_dijkstra(puzzle.get_input());   

        // Reverse dijkstra's results
        let mut path_cells = HashSet::from([(end.0, end.1)]);
        let mut queue =
            VecDeque::from([(end.0, end.1, get_distance(&dist_map, end).unwrap())]);
        while let Some((x, y, max_distance)) = queue.pop_front() {
            let next_coords: [(Option<usize>, Option<usize>, Direction); 4] = [
                (x.checked_sub(1), Some(y), Direction::Right),
                (Some(x + 1), Some(y), Direction::Left),
                (Some(x), y.checked_sub(1), Direction::Down),
                (Some(x), Some(y + 1), Direction::Up),
            ];
            next_coords
                .iter()
                .filter_map(|c| Some((c.0?, c.1?, c.2)))
                .filter_map(|(dx, dy, d_dir)| {
                    // Did we already visit this cell?
                    if path_cells.contains(&(dx, dy)) {
                        return None;
                    }
                    // Verify the distance if we went from the coordinate to the current
                    let distance_to_current = get_distance(&dist_map, (x, y, d_dir))?;
                    if distance_to_current > max_distance {
                        return None;
                    }
                    // Find from which origins we could have reached the current coordinate by
                    // calculating the cost of the turn combined with the move
                    let distances_to_next = dist_map.get(&(dx, dy))?;
                    for (origin_dir, distance_from_origin) in distances_to_next {
                        if let Some(turn_cost) = origin_dir.turn_cost(&d_dir) {
                            let expected_distance_to_current = distance_from_origin + (turn_cost + 1);
                            if expected_distance_to_current == distance_to_current {
                                queue.push_back((dx, dy, expected_distance_to_current));
                                path_cells.insert((dx, dy));
                            }
                        }
                    }
                    Some(())
                })
                .for_each(|_| ()); // We just want to "run" the iterator
        }
        path_cells.len().into()
    }
}

type DistanceMap = HashMap<(usize, usize), HashMap<Direction, usize>>;
fn get_distance(map: &DistanceMap, c: (usize, usize, Direction)) -> Option<usize> {
    map.get(&(c.0, c.1)).and_then(|m| m.get(&c.2).copied())
}
fn get_best_distance(map: &DistanceMap, c: (usize, usize)) -> Option<(Direction, usize)> {
    map.get(&(c.0, c.1))
        .and_then(|m| m.iter().min_by_key(|a| a.1))
        .map(|v| (*v.0, *v.1))
}

fn create_distance_map_with_dijkstra(
    input: String,
) -> (
    DistanceMap,
    (usize, usize, Direction),
) {
    let mut grid: Grid<usize, char> = input.into();
    let start = grid.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let start = (*start.0, *start.1, Direction::Right);
    let end = grid.iter().find(|(_, &c)| c == 'E').unwrap().0;
    let end = (*end.0, *end.1);
    grid.insert(start.0, start.1, '.');
    grid.insert(end.0, end.1, '.');

    let mut dist_map = HashMap::from([((start.0, start.1), HashMap::from([(start.2, 0)]))]);
    let mut queue = VecDeque::from([(start.0, start.1, start.2, 0)]);
    while let Some((x, y, dir, d)) = queue.pop_front() {
        if (x, y) == (end.0, end.1) {
            continue;
        }
        let next_coords = [
            (x.checked_sub(1), Some(y), Direction::Left),
            (Some(x + 1), Some(y), Direction::Right),
            (Some(x), y.checked_sub(1), Direction::Up),
            (Some(x), Some(y + 1), Direction::Down),
        ];
        next_coords
            .iter()
            .filter_map(|c| Some((c.0?, c.1?, c.2, dir.turn_cost(&c.2)?)))
            .filter(|(dx, dy, _, _)| grid.get(*dx, *dy) == Some(&'.'))
            .for_each(|(dx, dy, new_dir, turn_cost)| {
                let move_cost = turn_cost + d + 1;
                let current_cost = dist_map.get_mut(&(dx, dy));
                if let Some(current_cost) = current_cost {
                    if let Some(current_dir_cost) = current_cost.get_mut(&new_dir) {
                        if *current_dir_cost > move_cost {
                            *current_dir_cost = move_cost;
                        } else {
                            return;
                        }
                    } else {
                        current_cost.insert(new_dir, move_cost);
                    }
                } else {
                    dist_map.insert((dx, dy), HashMap::from([(new_dir, move_cost)]));
                }
                queue.push_back((dx, dy, new_dir, move_cost));
            });
    }
    let best_direction = get_best_distance(&dist_map, end).expect("Best distance").0;
    (dist_map, (end.0, end.1, best_direction))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

impl Direction {
    fn turn_cost(&self, other: &Self) -> Option<usize> {
        if self == other {
            return Some(0);
        }
        if (*self as i32).abs_diff(*other as i32) == 2 {
            return None;
        }
        Some(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 7036.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 45.into());
    }
}
