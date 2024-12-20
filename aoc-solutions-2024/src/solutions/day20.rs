use std::collections::{HashMap, HashSet, VecDeque};

use aoc_core::{aoc_puzzle, tools::Grid, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 20)]
pub struct Day {
    limit_a: usize,
    limit_b: usize,
}

impl Default for Day {
    fn default() -> Self {
        Self {
            limit_a: 100,
            limit_b: 100,
        }
    }
}

type Coord = (usize, usize);

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let grid: Grid<usize, char> = puzzle.get_input().into();
        let walls = grid.collect_cells::<HashSet<_>>('#');
        let start = grid.find_coord('S').unwrap();
        let end = grid.find_coord('E').unwrap();

        let start_to_finish = do_dijkstra(start, end, &walls);
        let finish_to_start = do_dijkstra(end, start, &walls);
        let length = start_to_finish.get_path().unwrap().len() - 1;

        let mut cheat_map: HashMap<usize, HashSet<Coord>> = HashMap::new();
        for x in 1..(*grid.x_range().unwrap().end()) {
            for y in 1..(*grid.y_range().unwrap().end()) {
                if walls.contains(&(x, y)) {
                    for (l, r) in [
                        ((x.checked_sub(1), Some(y)), (Some(x + 1), Some(y))),
                        ((Some(x), y.checked_sub(1)), (Some(x), Some(y + 1))),
                    ]
                    .iter()
                    .filter_map(|(l, r)| Some(((l.0?, l.1?), (r.0?, r.1?))))
                    .filter(|(l, r)| !walls.contains(l) && !walls.contains(r))
                    {
                        if let (Some(var1l), Some(var1r)) =
                            (start_to_finish.map.get(&l), finish_to_start.map.get(&r))
                        {
                            let var2r = start_to_finish.map.get(&r).unwrap();
                            let var2l = finish_to_start.map.get(&l).unwrap();
                            let var1 = (var1l.0 + var1r.0) + 2;
                            let var2 = (var2l.0 + var2r.0) + 2;
                            let new_length = var1.min(var2);

                            if new_length < length {
                                let saved = length - new_length;
                                cheat_map
                                    .entry(saved)
                                    .and_modify(|v| {
                                        v.insert((x, y));
                                    })
                                    .or_insert(HashSet::from([(x, y)]));
                            }
                        }
                    }
                }
            }
        }

        // grid.printer()
        //     .with_cell_fill('#')
        //     .with_cell_width(4)
        //     .with_legend()
        //     .with_cell_override_fn(move |(x, y)| {
        //         start_to_finish
        //             .map
        //             .get(&(x, y))
        //             .map(|v| v.0.to_string())
        //     })
        //     .print();

        cheat_map
            .iter()
            .filter(|(k, _)| **k >= self.limit_a)
            .map(|(_, v)| v.len())
            .sum::<usize>()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let grid: Grid<usize, char> = puzzle.get_input().into();
        let walls = grid.collect_cells::<HashSet<_>>('#');
        let start = grid.find_coord('S').unwrap();
        let end = grid.find_coord('E').unwrap();

        let start_to_finish = do_dijkstra(start, end, &walls);
        let finish_to_start = do_dijkstra(end, start, &walls);
        let length = start_to_finish.get_path().unwrap().len() - 1;
        let (x_range, y_range) = (grid.x_range().unwrap(), grid.y_range().unwrap());

        let mut cheat_map: HashMap<usize, HashSet<(Coord, Coord)>> =
            HashMap::new();
        for (cheat_start, (distance_from_start, _)) in start_to_finish.map.clone() {
            if length - distance_from_start < 50 {
                // We can't cheat enough to save 50 picoseconds
                continue;
            }
            for x in cheat_start.0.saturating_sub(20)..=(cheat_start.0 + 20).max(*x_range.end()) {
                for y in cheat_start.1.saturating_sub(20)..=(cheat_start.1 + 20).max(*y_range.end())
                {
                    let distance = cheat_start.0.abs_diff(x) + cheat_start.1.abs_diff(y);
                    if distance > 20 {
                        continue;
                    }
                    if let Some((distance_from_end, _)) = finish_to_start.map.get(&(x, y)) {
                        let new_length = distance_from_start + distance_from_end + distance;
                        if new_length < length {
                            let saved = length - new_length;
                            cheat_map
                                .entry(saved)
                                .and_modify(|v| {
                                    v.insert((cheat_start, (x, y)));
                                })
                                .or_insert(HashSet::from([(cheat_start, (x, y))]));
                        }
                    }
                }
            }
        }

        cheat_map
            .iter()
            .filter(|(k, _)| **k >= self.limit_b)
            .map(|(_, v)| v.len())
            .sum::<usize>()
            .into()
    }
}

fn do_dijkstra(
    start: Coord,
    end: Coord,
    walls: &HashSet<Coord>,
) -> DijkstraResult {
    let mut queue = VecDeque::<(usize, usize, usize)>::from([(start.0, start.1, 0)]);
    let mut result = DijkstraResult::new(start, end);
    while let Some((x, y, dist)) = queue.pop_front() {
        let next_dist = dist + 1;
        for (nx, ny) in [
            (x.checked_sub(1), Some(y)),
            (Some(x + 1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (Some(x), Some(y + 1)),
        ]
        .iter()
        .filter_map(|c| Some((c.0?, c.1?)))
        .filter(|(nx, ny)| !walls.contains(&(*nx, *ny)))
        {
            if let Some(cur) = result.map.get_mut(&(nx, ny)) {
                if cur.0 <= next_dist {
                    continue;
                }
                *cur = (next_dist, Some((x, y)));
            } else {
                result.map.insert((nx, ny), (next_dist, Some((x, y))));
            }
            if nx == end.0 && ny == end.1 {
                continue;
            }
            queue.push_back((nx, ny, next_dist));
        }
    }
    result
}

type DijkstraCoordRecord = (usize, Option<Coord>);

#[allow(unused)]
struct DijkstraResult {
    map: HashMap<Coord, DijkstraCoordRecord>,
    start: Coord,
    end: Coord,
}

impl DijkstraResult {
    pub fn new(start: Coord, end: Coord) -> Self {
        Self {
            map: HashMap::from([(start, (0, None))]),
            start,
            end,
        }
    }

    #[allow(unused)]
    pub fn found_path(&self) -> bool {
        self.map.contains_key(&self.end)
    }

    pub fn get_path(&self) -> Option<Vec<Coord>> {
        self.map.get(&self.end).map(|v| {
            let mut path = vec![self.end];
            let mut next = v.1;
            while let Some(cur) = next {
                path.push(cur);
                next = self.map[&cur].1;
            }
            path.reverse();
            path
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day {
            limit_a: 0,
            limit_b: 50,
        }
        .part1(&get_puzzle());
        assert_eq!(result, 44.into());
    }

    #[test]
    fn part2() {
        let result = Day {
            limit_a: 0,
            limit_b: 50,
        }
        .part2(&get_puzzle());
        assert_eq!(result, 285.into());
    }
}
