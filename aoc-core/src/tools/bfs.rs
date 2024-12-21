use std::collections::{HashMap, HashSet, VecDeque};

type Coord = (usize, usize);

/// A builder for the Breadth First Search algorithm
///
/// Rules to use BFS:
/// - Only one path from start to end
/// - The path is the shortest path
/// - All edges have the same weight
pub struct BfsBuilder {
    obstacles: HashSet<Coord>,
    start: Coord,
    end: Coord,
    use_dfs: bool,
    bounds: Option<(Coord, Coord)>,
}

impl BfsBuilder {
    pub fn new(start: Coord, end: Coord) -> Self {
        Self {
            start,
            end,
            obstacles: HashSet::new(),
            use_dfs: false,
            bounds: None,
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

    /// Use Depth First Search instead of Breadth First Search
    pub fn use_dfs(mut self) -> Self {
        self.use_dfs = true;
        self
    }

    /// Run the Breadth First Search algorithm
    ///
    /// This is not a pure implementation of BFS, as we also store the parent
    /// of each cell to be able to reconstruct the path
    pub fn run(mut self) -> Option<BfsResult> {
        let mut queue = VecDeque::from([self.start]);
        let mut visited = HashMap::from([(self.start, None)]);

        while let Some((x, y)) = if self.use_dfs {
            queue.pop_back()
        } else {
            queue.pop_front()
        } {
            if x == self.end.0 && y == self.end.1 {
                let mut path = vec![(x, y)];
                let mut cur = (x, y);
                while let Some(Some(prev)) = visited.get(&cur) {
                    path.push(*prev);
                    cur = *prev;
                }
                path.reverse();
                return Some(BfsResult { path });
            }
            for (nx, ny) in self.visitable_neighbors(x, y) {
                if visited.contains_key(&(nx, ny)) {
                    continue;
                }
                visited.insert((nx, ny), Some((x, y)));
                queue.push_back((nx, ny));
            }
        }
        None
    }

    // Retrieve the neighbors of a cell that are not obstacles and are within the bounds of the grid
    fn visitable_neighbors(&mut self, x: usize, y: usize) -> impl Iterator<Item = Coord> + '_ {
        let (top_left, bottom_right) = self.get_bounds();
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

    // Retrieve the bounds of the grid, resolve and cache bounds if they were not provided
    fn get_bounds(&mut self) -> (Coord, Coord) {
        match self.bounds {
            Some(bounds) => bounds,
            None => {
                let mut top_left = min_coord(self.start, self.end);
                let mut bottom_right = max_coord(self.start, self.end);
                for c in self.obstacles.iter() {
                    top_left = min_coord(top_left, *c);
                    bottom_right = max_coord(bottom_right, *c);
                }
                self.bounds = Some((top_left, bottom_right));
                (top_left, bottom_right)
            }
        }
    }
}

fn min_coord(a: Coord, b: Coord) -> Coord {
    (a.0.min(b.0), a.1.min(b.1))
}
fn max_coord(a: Coord, b: Coord) -> Coord {
    (a.0.max(b.0), a.1.max(b.1))
}

/// Result of the Breadth First Search algorithm
pub struct BfsResult {
    path: Vec<Coord>,
}
impl BfsResult {
    pub fn path(&self) -> &Vec<Coord> {
        &self.path
    }
    pub fn len(&self) -> usize {
        self.path.len()
    }
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Test case:
    S..###
    .#....
    .#.###
    .#.#.E
    .#...#
     */
    fn create_with_testcase() -> BfsBuilder {
        BfsBuilder::new((0, 0), (5, 3)).with_obstacles(HashSet::from([
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
    fn test_bfs() {
        let result = create_with_testcase().run().expect("Path not found");
        assert_eq!(
            result.path(),
            &vec![
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

    #[test]
    fn test_dfs() {
        let result = create_with_testcase()
            .use_dfs()
            .run()
            .expect("Path not found");
        assert_eq!(
            result.path(),
            &vec![
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
