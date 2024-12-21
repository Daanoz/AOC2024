use std::collections::HashSet;

use crate::tools::{BfsBuilder, DijkstraBuilder};

use super::Grid;

pub struct PathFinder<'g, K, D> {
    grid: &'g Grid<K, D>,
    start: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
    obstacles: HashSet<(usize, usize)>,
}

impl<'g, K, D> PathFinder<'g, K, D>
where
    K: Ord + Copy + Into<usize>,
    D: Eq,
{
    pub(super) fn new(grid: &'g Grid<K, D>) -> Self {
        Self {
            grid,
            start: None,
            end: None,
            obstacles: HashSet::new(),
        }
    }
    pub fn with_start_coord(mut self, start: (usize, usize)) -> Self {
        self.start = Some(start);
        self
    }
    pub fn with_start(mut self, value: D) -> Self {
        self.start = self
            .grid
            .find_coord(value)
            .map(|(x, y)| (x.into(), y.into()));
        self
    }
    pub fn with_end_coord(mut self, end: (usize, usize)) -> Self {
        self.end = Some(end);
        self
    }
    pub fn with_end(mut self, value: D) -> Self {
        self.end = self
            .grid
            .find_coord(value)
            .map(|(x, y)| (x.into(), y.into()));
        self
    }
    pub fn with_obstacle_coords<T: IntoIterator<Item = (usize, usize)>>(
        mut self,
        coords: T,
    ) -> Self {
        self.obstacles.extend(coords);
        self
    }
    pub fn with_obstacles(mut self, value: D) -> Self {
        self.obstacles.extend(
            self.grid
                .collect_cells_iter(value)
                .map(|(x, y)| (x.into(), y.into())),
        );
        self
    }
    pub fn bfs(self) -> BfsBuilder {
        let (start, end) = match (self.start, self.end) {
            (Some(start), Some(end)) => (start, end),
            _ => panic!("Start and end coordinates must be set"),
        };
        BfsBuilder::new(start, end).with_obstacles(self.obstacles)
    }
    pub fn dijkstra(self) -> DijkstraBuilder {
        let (start, end) = match (self.start, self.end) {
            (Some(start), Some(end)) => (start, end),
            _ => panic!("Start and end coordinates must be set"),
        };
        DijkstraBuilder::new(start, end).with_obstacles(self.obstacles)
    }
}
