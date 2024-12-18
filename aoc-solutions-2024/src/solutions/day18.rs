use std::collections::{HashMap, HashSet, VecDeque};

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 18)]
pub struct Day {
    simulation_size: usize,
    size: (u32, u32),
}

impl Default for Day {
    fn default() -> Self {
        Self { simulation_size: 1024, size: (70, 70) }
    }
}

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let dead_cells = puzzle
            .input_as_str()
            .lines()
            .take(self.simulation_size)
            .map(|line| line.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
            .collect::<HashSet<_>>();
        do_dijkstra(self.size, &dead_cells).unwrap_or_default().len().into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let mut dead_cells = HashSet::new();
        let mut falling_bytes = puzzle
            .input_as_str()
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()));
        falling_bytes.by_ref().take(self.simulation_size).for_each(|b| { dead_cells.insert(b); }); // fast forward 1024 bytes
        let mut current_path = do_dijkstra(self.size, &dead_cells).unwrap();
        let final_byte = falling_bytes.find(|b| {
            dead_cells.insert(*b);
            if !current_path.contains(b) {
                // We did not hit the path, so no need to recalculate
                return false;
            }
            let new_path = do_dijkstra(self.size, &dead_cells);
            if let Some(path) = new_path {
                current_path = path;
                return false;
            }
            true
        });
        final_byte.map(|(x, y)| format!("{x},{y}")).into()
    }
}

fn do_dijkstra(
    size: (u32, u32),
    dead_cells: &HashSet<(u32, u32)>,
) -> Option<Vec<(u32, u32)>> {
    let start = (0, 0);
    let end = size;
    let mut queue = VecDeque::<(u32, u32, u32)>::from([(start.0, start.1, 0)]);
    let mut dist_map: HashMap<(u32, u32), (u32, (u32, u32))> = HashMap::new();
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
            .filter(|(nx, ny)| 
                *nx <= size.0 && *ny <= size.1 && !dead_cells.contains(&(*nx, *ny))
            ) {
            if let Some(cur) = dist_map.get_mut(&(nx, ny)) {
                if cur.0 <= next_dist {
                    continue;
                }
                *cur = (next_dist, (x, y));
            } else {
                dist_map.insert((nx, ny), (next_dist, (x, y)));
            }
            queue.push_back((nx, ny, next_dist));
        }
    }
    dist_map.get(&end).map(|v| {
        let mut path = vec![end];
        let mut cur = v.1;
        while cur != start {
            path.push(cur);
            cur = dist_map[&cur].1;
        }
        path.reverse();
        path
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day { simulation_size: 12, size: (6, 6) }.part1(&get_puzzle());
        assert_eq!(result, 22.into());
    }

    #[test]
    fn part2() {
        let result = Day { simulation_size: 12, size: (6, 6) }.part2(&get_puzzle());
        assert_eq!(result, "6,1".into());
    }
}
