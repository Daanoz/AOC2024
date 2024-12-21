use std::{collections::BTreeMap, fmt::Display, ops::RangeInclusive, str::FromStr};

mod grid_iterator;
mod grid_printer;
mod path_finder;
pub use grid_iterator::*;
use grid_printer::*;

pub struct Grid<K, D> {
    grid: BTreeMap<K, BTreeMap<K, D>>,
}

impl<T> From<String> for Grid<usize, T>
where
    T: From<char>,
{
    fn from(input: String) -> Self {
        let mut grid: BTreeMap<usize, BTreeMap<usize, T>> = Default::default();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.entry(y).or_default().insert(x, c.into());
            }
        }
        Self { grid }
    }
}

impl FromStr for Grid<usize, u32> {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, String> {
        let mut grid: BTreeMap<usize, BTreeMap<usize, u32>> = Default::default();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.entry(y).or_default().insert(
                    x,
                    c.to_digit(10).ok_or_else(|| "Invalid digit".to_string())?,
                );
            }
        }

        Ok(Self { grid })
    }
}

impl<K, D> Default for Grid<K, D> {
    fn default() -> Self {
        Self {
            grid: Default::default(),
        }
    }
}

/// HashMap alike functions
impl<K, D> Grid<K, D>
where
    K: Ord,
{
    pub fn clear(&mut self) {
        self.grid.clear();
    }
    pub fn contains_key(&self, x: K, y: K) -> bool {
        self.grid.get(&y).map_or(false, |row| row.contains_key(&x))
    }
    pub fn entry(&mut self, x: K, y: K) -> std::collections::btree_map::Entry<K, D> {
        self.grid.entry(y).or_default().entry(x)
    }
    pub fn get(&self, x: K, y: K) -> Option<&D> {
        self.grid.get(&y).and_then(|row| row.get(&x))
    }
    pub fn get_mut(&mut self, x: K, y: K) -> Option<&mut D> {
        self.grid.get_mut(&y).and_then(|row| row.get_mut(&x))
    }
    pub fn insert(&mut self, x: K, y: K, value: D) -> Option<D> {
        self.grid.entry(y).or_default().insert(x, value)
    }
    pub fn into_values(self) -> impl Iterator<Item = D> {
        self.grid.into_iter().flat_map(|(_, row)| row.into_values())
    }
    pub fn is_empty(&self) -> bool {
        self.grid.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = ((&K, &K), &D)> {
        self.grid
            .iter()
            .flat_map(|(y, row)| row.iter().map(move |(x, value)| ((x, y), value)))
    }
    pub fn keys(&self) -> impl Iterator<Item = (&K, &K)> {
        self.grid
            .iter()
            .flat_map(|(y, row)| row.iter().map(move |(x, _)| (x, y)))
    }
    pub fn len(&self) -> usize {
        self.grid.iter().map(|row| row.1.len()).sum()
    }
    pub fn remove(&mut self, x: K, y: K) -> Option<D> {
        self.grid.get_mut(&y).and_then(|row| row.remove(&x))
    }
    pub fn remove_entry(&mut self, x: K, y: K) -> Option<((K, K), D)> {
        self.grid
            .get_mut(&y)
            .and_then(|row| row.remove_entry(&x).map(|(_, value)| ((x, y), value)))
    }
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &K, &mut D) -> bool,
    {
        self.grid.retain(|y, row| {
            row.retain(|x, value| f(x, y, value));
            !row.is_empty()
        });
    }
    pub fn values(&self) -> impl Iterator<Item = &D> {
        self.grid
            .iter()
            .flat_map(|(_, row)| row.iter().map(|(_, value)| value))
    }
}

impl<K, D> Grid<K, D>
where
    K: Clone,
{
    pub fn into_keys(self) -> impl Iterator<Item = (K, K)> {
        self.grid
            .into_iter()
            .flat_map(|(y, row)| row.into_keys().map(move |x| (x, y.clone())))
    }
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> impl Iterator<Item = ((K, K), D)> {
        self.grid.into_iter().flat_map(|(y, row)| {
            row.into_iter()
                .map(move |(x, value)| ((x, y.clone()), value))
        })
    }
}

impl<K, D> Grid<K, D>
where
    K: Ord + Clone,
{
    pub fn transpose(&mut self) -> &Self {
        let old_grid = std::mem::take(&mut self.grid);
        old_grid.into_iter().for_each(|(y, row)| {
            row.into_iter().for_each(|(x, value)| {
                self.grid.entry(x).or_default().insert(y.clone(), value);
            });
        });
        self
    }
}

impl<K, D> Clone for Grid<K, D>
where
    K: Clone,
    D: Clone,
{
    fn clone(&self) -> Self {
        Self {
            grid: self.grid.clone(),
        }
    }
}

/// Additional functions
impl<K, D> Grid<K, D>
where
    K: Ord,
{
    pub fn row(&self, y: K) -> impl Iterator<Item = (&K, &D)> {
        self.grid.get(&y).into_iter().flat_map(|row| row.iter())
    }
    pub fn column(&self, x: K) -> impl Iterator<Item = (&K, &D)> {
        self.grid
            .iter()
            .flat_map(move |(y, row)| row.get(&x).map(|value| (y, value)))
    }
    pub fn width(&self) -> usize {
        self.grid.iter().map(|row| row.1.len()).max().unwrap_or(0)
    }
    pub fn height(&self) -> usize {
        self.grid.len()
    }
    pub fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
}

impl<K, D> Grid<K, D>
where
    K: Ord + Copy,
    D: Eq,
{
    pub fn collect_cells_iter(&self, value: D) -> impl Iterator<Item = (K, K)> + '_ {
        self.iter()
            .filter(move |(_, c)| **c == value)
            .map(|(p, _)| (*p.0, *p.1))
    }
    pub fn collect_cells<T: std::iter::FromIterator<(K, K)>>(&self, value: D) -> T {
        self.collect_cells_iter(value).collect::<T>()
    }
    pub fn find_coord(&self, value: D) -> Option<(K, K)> {
        self.iter()
            .find(|(_, c)| **c == value)
            .map(|(p, _)| (*p.0, *p.1))
    }
}

impl<K, D> Grid<K, D>
where
    K: Clone + Ord,
{
    pub fn x_range(&self) -> Option<RangeInclusive<K>> {
        self.grid
            .values()
            .fold(None, |r, row| match (r, Self::get_range(row.keys())) {
                (Some(r), Some(row_range)) => Some(RangeInclusive::new(
                    r.start().min(row_range.start()).clone(),
                    r.end().max(row_range.end()).clone(),
                )),
                (None, Some(row_range)) => Some(row_range),
                (r, None) => r,
            })
    }
    pub fn y_range(&self) -> Option<RangeInclusive<K>> {
        Self::get_range(self.grid.keys())
    }
    pub fn row_sorted(&self, y: K) -> impl Iterator<Item = (&K, &D)> {
        let mut row = self.row(y).collect::<Vec<_>>();
        row.sort_unstable_by(|a, b| a.0.cmp(b.0));
        row.into_iter()
    }
    pub fn column_sorted(&self, x: K) -> impl Iterator<Item = (&K, &D)> {
        let mut column = self.column(x).collect::<Vec<_>>();
        column.sort_unstable_by(|a, b| a.0.cmp(b.0));
        column.into_iter()
    }
    fn get_range<T>(
        mut keys: std::collections::btree_map::Keys<'_, K, T>,
    ) -> Option<RangeInclusive<K>> {
        let range = if let Some(first) = keys.next() {
            RangeInclusive::new(first.clone(), first.clone())
        } else {
            return None;
        };
        Some(keys.fold(range, |r, k| {
            RangeInclusive::new(r.start().min(k).clone(), r.end().max(k).clone())
        }))
    }
}

impl<D> Grid<usize, D> {
    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if let Some(nx) = x.checked_sub(1) {
            neighbors.push((nx, y));
        }
        if let Some(ny) = y.checked_sub(1) {
            neighbors.push((x, ny));
        }
        neighbors.push((x + 1, y));
        neighbors.push((x, y + 1));
        neighbors
    }
}

impl<K, D> Grid<K, D>
where
    K: Copy + Ord + std::ops::Sub<Output = K> + std::ops::Add<Output = K> + TryFrom<u8>,
    <K as TryFrom<u8>>::Error: std::fmt::Debug,
{
    // Rotate the grid 45 degrees clockwise, every call to this function will increase the grid size to X+Y
    pub fn to_diagonal(&mut self) -> &Self {
        let (x_range, y_range) = match (self.x_range(), self.y_range()) {
            (Some(x), Some(y)) => (x, y),
            _ => return self,
        };
        let mut old_grid = std::mem::take(&mut self.grid);
        let diagonals = RangeInclusive::new(
            *x_range.start() + *y_range.start(),
            *x_range.end() + *y_range.end(),
        );
        // Reuse the grid iterator to iterate over the diagonals
        let diagonal_iterator = GridIterator::new(diagonals, y_range.clone());
        for (diagonal, _) in diagonal_iterator.x_iter() {
            for (_, y) in diagonal_iterator.y_iter().rev() {
                if y > diagonal {
                    continue;
                }
                let x = diagonal.sub(y);
                if !x_range.contains(&x) {
                    continue;
                }
                if let Some(value) = old_grid.get_mut(&y).and_then(|row| row.remove(&x)) {
                    let new_x = y_range.end().sub(y) + x;
                    self.grid.entry(diagonal).or_default().insert(new_x, value);
                }
            }
        }
        self
    }
}

impl<K, D> Grid<K, D>
where
    K: Copy + Ord + std::ops::Add<Output = K> + TryFrom<u8>,
    <K as TryFrom<u8>>::Error: std::fmt::Debug,
{
    /// Returns an iterator over the grid range
    pub fn grid_iter(&self) -> GridIterator<K> {
        match (self.x_range(), self.y_range()) {
            (Some(x_range), Some(y_range)) => GridIterator::new(x_range, y_range),
            _ => {
                let nil = 0.try_into().expect("Should accept 0");
                let noop_range = nil..=nil;
                GridIterator::new(noop_range.clone(), noop_range)
            }
        }
    }

    /// Iter the entire range, even when the cells are empty
    pub fn iter_range(&self) -> impl Iterator<Item = ((K, K), Option<&D>)> {
        self.grid_iter().map(|(x, y)| ((x, y), self.get(x, y)))
    }

    /// Mutate the entire range, even when the cells are empty
    pub fn for_each_entry_range<F>(&mut self, mut f: F)
    where
        F: FnMut((K, K), std::collections::btree_map::Entry<K, D>),
    {
        self.grid_iter().for_each(|c| {
            f((c.0, c.1), self.entry(c.0, c.1));
        });
    }
}

impl<K, D> Grid<K, D>
where
    K: Copy + Ord + std::ops::Add<Output = K> + TryFrom<u8>,
    <K as TryFrom<u8>>::Error: std::fmt::Debug,
    D: Clone,
{
    /// Fill empty cells with a default value
    pub fn fill_empty(&mut self, d: D) {
        self.for_each_entry_range(|_, entry| {
            entry.or_insert(d.clone());
        });
    }
}

impl<K, D> Grid<K, D>
where
    K: Ord + Copy + Into<usize>,
    D: Eq,
{
    pub fn apply_path_finder(&self) -> path_finder::PathFinder<K, D> {
        path_finder::PathFinder::new(self)
    }
}

impl<K, D> Grid<K, D>
where
    K: Copy + Ord + std::ops::Add<Output = K> + TryFrom<u8> + Display,
    <K as TryFrom<u8>>::Error: std::fmt::Debug,
    D: Eq + ToString,
{
    pub fn printer(&self) -> GridPrinter<K, D> {
        GridPrinter::new(self)
    }
}

impl<K, D> Display for Grid<K, D>
where
    K: Copy + Ord + std::ops::Add<Output = K> + TryFrom<u8> + Display,
    <K as TryFrom<u8>>::Error: std::fmt::Debug,
    D: Eq + ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.printer().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_grid() {
        let grid: Grid<usize, char> = Grid::from("123\n456\n789".to_string());
        assert_eq!(grid.to_string(), "123\n456\n789".to_string());
    }

    #[test]
    fn should_transpose_grid() {
        let mut grid: Grid<usize, char> = Grid::from("123\n456\n789".to_string());
        grid.transpose();
        assert_eq!(grid.to_string(), "147\n258\n369".to_string());
    }

    mod to_diagonal {
        use super::*;

        /*
        abc
        def
        ghi

          a
         d b
        g e c
         h f
          i
                */
        #[test]
        fn should_convert_simple_to_diagonal() {
            let mut grid: Grid<usize, char> = Grid::from("abc\ndef\nghi".to_string());
            grid.to_diagonal();
            assert_eq!(
                grid.to_string(),
                "  a  \n d b \ng e c\n h f \n  i  ".to_string()
            );
        }
        /*
        ABCDEF
        GHIJKL
        MNOPQR

          A
         G B
        M H C
         N I D
          O J E
           P K F
            Q L
             R
                */
        #[test]
        fn should_convert_wide_to_diagonal() {
            let mut grid: Grid<usize, char> = Grid::from("ABCDEF\nGHIJKL\nMNOPQR".to_string());
            grid.to_diagonal();
            assert_eq!(
                grid.to_string(),
                "  A     \n G B    \nM H C   \n N I D  \n  O J E \n   P K F\n    Q L \n     R  "
                    .to_string()
            );
        }
        /*
        ABC
        DEF
        GHI
        JKL
        MNO
        PQR

             A
            D B
           G E C
          J H F
         M K I
        P N L
         Q O
          R
                */
        #[test]
        fn should_convert_tall_to_diagonal() {
            let mut grid: Grid<usize, char> =
                Grid::from("ABC\nDEF\nGHI\nJKL\nMNO\nPQR".to_string());
            grid.to_diagonal();
            assert_eq!(
                grid.to_string(),
                "     A  \n    D B \n   G E C\n  J H F \n M K I  \nP N L   \n Q O    \n  R     "
                    .to_string()
            );
        }
    }
}
