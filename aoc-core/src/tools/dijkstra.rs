use std::collections::{HashMap, HashSet, VecDeque};

type Coord = (usize, usize);

/// A builder for the Dijkstra algorithm
pub struct DijkstraBuilder {
    obstacles: HashSet<Coord>,
    start: Coord,
    end: Coord,
    bounds: Option<(Coord, Coord)>,
    calculate_cost: Box<dyn Fn(CostInput) -> usize>,
}

impl DijkstraBuilder {
    pub fn new(start: Coord, end: Coord) -> Self {
        Self {
            start,
            end,
            obstacles: HashSet::new(),
            bounds: None,
            calculate_cost: Box::new(|input| input.cost + 1),
        }
    }

    /// Add obstacles to the grid, this can be called multiple times
    pub fn with_obstacles(mut self, obstacles: HashSet<Coord>) -> Self {
        self.obstacles.extend(obstacles);
        self
    }

    /// Enforce search bounds, if not provided the bounds will be calculated
    /// based on the start, end and obstacles
    pub fn with_bounds(mut self, top_left: Coord, bottom_right: Coord) -> Self {
        self.bounds = Some((top_left, bottom_right));
        self
    }

    /// Enforce search bounds, if not provided the bounds will be calculated
    /// based on the start, end and obstacles
    pub fn with_cost_func<F>(mut self, func: F) -> Self
    where
        F: Fn(CostInput) -> usize + 'static,
    {
        self.calculate_cost = Box::new(func);
        self
    }

    /// Run the Dijkstra algorithm
    pub fn run(mut self) -> DijkstraResult {
        let mut queue = VecDeque::from([(self.start, 0)]);
        let mut dist_map = HashMap::from([(
            self.start,
            DijkstraEntry {
                cost: 0,
                parent: None,
            },
        )]);
        self.determine_bounds();
        let cost_func = &self.calculate_cost;
        let end = self.end;
        while let Some(((x, y), cost)) = queue.pop_front() {
            for (nx, ny) in self.visitable_neighbors(x, y) {
                let next_cost = cost_func(CostInput {
                    origin: (x, y),
                    next: (nx, ny),
                    cost,
                });
                if let Some(existing_cost) = dist_map.get(&(nx, ny)) {
                    if existing_cost.cost <= next_cost {
                        continue;
                    }
                }
                dist_map.insert(
                    (nx, ny),
                    DijkstraEntry {
                        cost: next_cost,
                        parent: Some((x, y)),
                    },
                );
                if (nx, ny) != end {
                    queue.push_back(((nx, ny), next_cost));
                }
            }
        }
        DijkstraResult {
            map: dist_map,
            end: self.end,
        }
    }

    // Retrieve the neighbors of a cell that are not obstacles and are within the bounds of the grid
    fn visitable_neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = Coord> + '_ {
        let (top_left, bottom_right) = self.bounds.unwrap();
        [
            (x.checked_sub(1), Some(y)),
            (Some(x + 1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (Some(x), Some(y + 1)),
        ]
        .into_iter()
        .filter_map(|(nx, ny)| Some((nx?, ny?)))
        .filter(move |(nx, ny)| {
            *nx >= top_left.0 && *nx <= bottom_right.0 && *ny >= top_left.1 && *ny <= bottom_right.1
        })
        .filter(|(nx, ny)| !self.obstacles.contains(&(*nx, *ny)))
    }

    // Determine grid bounds if not defined
    fn determine_bounds(&mut self) {
        if self.bounds.is_some() {
            return;
        }
        let mut top_left = min_coord(self.start, self.end);
        let mut bottom_right = max_coord(self.start, self.end);
        for c in self.obstacles.iter() {
            top_left = min_coord(top_left, *c);
            bottom_right = max_coord(bottom_right, *c);
        }
        self.bounds = Some((top_left, bottom_right));
    }
}

fn min_coord(a: Coord, b: Coord) -> Coord {
    (a.0.min(b.0), a.1.min(b.1))
}
fn max_coord(a: Coord, b: Coord) -> Coord {
    (a.0.max(b.0), a.1.max(b.1))
}

pub struct CostInput {
    pub origin: Coord,
    pub next: Coord,
    pub cost: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DijkstraEntry {
    cost: usize,
    parent: Option<Coord>,
}

/// Result of the Dijkstra algorithm
pub struct DijkstraResult {
    map: HashMap<(usize, usize), DijkstraEntry>,
    end: (usize, usize),
}

impl DijkstraResult {
    pub fn found_path(&self) -> bool {
        self.map.contains_key(&self.end)
    }

    pub fn path(&self) -> Option<Vec<(usize, usize)>> {
        self.map.get(&self.end).map(|v| {
            let mut path = vec![self.end];
            let mut next = v.parent;
            while let Some(cur) = next {
                path.push(cur);
                next = self.map[&cur].parent;
            }
            path.reverse();
            path
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod single_path {
        use super::*;

        /* Test case:
        S..###
        .#....
        .#.###
        .#.#.E
        .#...#
        */
        fn create_with_testcase() -> DijkstraBuilder {
            DijkstraBuilder::new((0, 0), (5, 3)).with_obstacles(HashSet::from([
                (3, 0),
                (4, 0),
                (5, 0),
                (1, 1),
                (1, 2),
                (3, 2),
                (4, 2),
                (5, 2),
                (1, 3),
                (3, 3),
                (1, 4),
                (5, 4),
            ]))
        }

        #[test]
        fn test_path_finding() {
            let result = create_with_testcase().run().path().expect("Path found");
            assert_eq!(
                result,
                vec![
                    (0, 0),
                    (1, 0),
                    (2, 0),
                    (2, 1),
                    (2, 2),
                    (2, 3),
                    (2, 4),
                    (3, 4),
                    (4, 4),
                    (4, 3),
                    (5, 3)
                ]
            );
        }
    }

    mod multiple_path {
        use super::*;

        /* Test case:
        S..###
        .#....
        .#.##.
        .#.#.E
        .....#
        */
        fn create_with_testcase() -> DijkstraBuilder {
            DijkstraBuilder::new((0, 0), (5, 3)).with_obstacles(HashSet::from([
                (3, 0),
                (4, 0),
                (5, 0),
                (1, 1),
                (1, 2),
                (3, 2),
                (4, 2),
                (1, 3),
                (3, 3),
                (5, 4),
            ]))
        }

        #[test]
        fn test_path_finding() {
            let result = create_with_testcase().run();
            assert_eq!(
                result.path().expect("Path found"),
                vec![
                    (0, 0),
                    (1, 0),
                    (2, 0),
                    (2, 1),
                    (3, 1),
                    (4, 1),
                    (5, 1),
                    (5, 2),
                    (5, 3)
                ]
            );
        }
    }

    mod multiple_path_with_cost {
        use super::*;

        /* Test case, moving up is very expensive:
        S......
        .#####.
        .#.....
        .#.####
        .#.....
        .#####E
        .......
        */
        fn create_with_testcase() -> DijkstraBuilder {
            DijkstraBuilder::new((0, 0), (6, 5))
                .with_bounds((0, 0), (6, 6))
                .with_obstacles(HashSet::from([
                    (1, 1),
                    (2, 1),
                    (3, 1),
                    (4, 1),
                    (5, 1),
                    (1, 2),
                    (1, 3),
                    (3, 3),
                    (4, 3),
                    (5, 3),
                    (6, 3),
                    (1, 4),
                    (1, 5),
                    (2, 5),
                    (3, 5),
                    (4, 5),
                    (5, 5),
                ]))
        }

        #[test]
        fn without_cost() {
            let result = create_with_testcase().run();
            assert_eq!(
                result.path().expect("Path found"),
                vec![
                    (0, 0),
                    (0, 1),
                    (0, 2),
                    (0, 3),
                    (0, 4),
                    (0, 5),
                    (0, 6),
                    (1, 6),
                    (2, 6),
                    (3, 6),
                    (4, 6),
                    (5, 6),
                    (6, 6),
                    (6, 5)
                ]
            );
        }

        #[test]
        fn with_cost_moving_up_expensive() {
            let result = create_with_testcase()
                .with_cost_func(|input| {
                    if input.origin.1 > input.next.1 {
                        input.cost + 10
                    } else {
                        input.cost + 1
                    }
                })
                .run();
            println!("{:#?}", result.map);
            assert_eq!(
                result.path().expect("Path found"),
                vec![
                    (0, 0),
                    (1, 0),
                    (2, 0),
                    (3, 0),
                    (4, 0),
                    (5, 0),
                    (6, 0),
                    (6, 1),
                    (6, 2),
                    (5, 2),
                    (4, 2),
                    (3, 2),
                    (2, 2),
                    (2, 3),
                    (2, 4),
                    (3, 4),
                    (4, 4),
                    (5, 4),
                    (6, 4),
                    (6, 5)
                ]
            );
        }
    }
}
