use std::{collections::BTreeMap, fmt::Display, hash::Hash, ops::RangeInclusive};

pub struct HashGrid<K, D> {
    grid: BTreeMap<K, BTreeMap<K, D>>,
}

impl<T> From<String> for HashGrid<usize, T>
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

impl<K, D> HashGrid<K, D> {
    pub fn new() -> Self {
        Self {
            grid: Default::default(),
        }
    }
}

/// HashMap alike functions
impl<K, D> HashGrid<K, D>
where
    K: Eq + Hash + Ord,
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
        self.grid
            .into_iter()
            .flat_map(|(_, row)| row.into_iter().map(|(_, value)| value))
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

impl<K, D> HashGrid<K, D>
where
    K: Eq + Hash + Clone,
{
    pub fn into_keys(self) -> impl Iterator<Item = (K, K)> {
        self.grid
            .into_iter()
            .flat_map(|(y, row)| row.into_iter().map(move |(x, _)| (x, y.clone())))
    }
    pub fn into_iter(self) -> impl Iterator<Item = ((K, K), D)> {
        self.grid.into_iter().flat_map(|(y, row)| {
            row.into_iter()
                .map(move |(x, value)| ((x, y.clone()), value))
        })
    }
}

impl<K, D> HashGrid<K, D>
where
    K: Eq + Hash + Clone + Ord,
{
    pub fn transpose(&mut self) -> &Self {
        let old_grid = std::mem::replace(&mut self.grid, BTreeMap::new());
        old_grid.into_iter().for_each(|(y, row)| {
            row.into_iter().for_each(|(x, value)| {
                self.grid.entry(x).or_default().insert(y.clone(), value);
            });
        });
        self
    }
}

impl<K, D> Clone for HashGrid<K, D>
where
    K: Eq + Hash + Clone,
    D: Clone,
{
    fn clone(&self) -> Self {
        Self {
            grid: self.grid.clone(),
        }
    }
}

/// Additional functions
impl<K, D> HashGrid<K, D>
where
    K: Eq + Hash + Ord,
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
/// Additional functions when key is Ord and Step
impl<K, D> HashGrid<K, D>
where
    K: Eq + Hash + Clone + Ord,
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
        row.sort_by(|a, b| a.0.cmp(b.0));
        row.into_iter()
    }
    pub fn column_sorted(&self, x: K) -> impl Iterator<Item = (&K, &D)> {
        let mut column = self.column(x).collect::<Vec<_>>();
        column.sort_by(|a, b| a.0.cmp(b.0));
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

impl<K, D> HashGrid<K, D>
where
    K: Eq + Hash + Into<usize> + From<usize> + Clone + Ord,
    D: Display,
{
    // If we need "step" ranges, convert them to usize
    fn ranges_as_usize(&self) -> Option<(RangeInclusive<usize>, RangeInclusive<usize>)> {
        match (self.x_range(), self.y_range()) {
            (Some(x), Some(y)) => Some((
                RangeInclusive::new(x.start().clone().into(), x.end().clone().into()),
                RangeInclusive::new(y.start().clone().into(), y.end().clone().into()),
            )),
            _ => return None,
        }
    }

    // Rotate the grid 45 degrees clockwise, every call to this function will increase the grid size to X+Y
    pub fn to_diagonal(&mut self) -> &Self {
        let (x_range, y_range): (RangeInclusive<usize>, RangeInclusive<usize>) =
            match self.ranges_as_usize() {
                Some(ranges) => ranges,
                _ => return self,
            };
        let mut old_grid = std::mem::replace(&mut self.grid, BTreeMap::new());
        let diagonals = RangeInclusive::new(
            x_range.start().clone() + y_range.start().clone(),
            x_range.end().clone() + y_range.end().clone(),
        );
        for diagonal in diagonals.clone() {
            for y in y_range.clone().rev() {
                let x = match diagonal.checked_sub(y) {
                    Some(x) => x,
                    _ => continue,
                };
                if !x_range.contains(&x) {
                    continue;
                }
                if let Some(value) = old_grid
                    .get_mut(&y.into())
                    .and_then(|row| row.remove(&x.into()))
                {
                    let new_x = (y_range.end() - y) + x;
                    self.grid
                        .entry(diagonal.into())
                        .or_default()
                        .insert(new_x.into(), value);
                }
            }
        }
        self
    }
}

impl<K, D> Display for HashGrid<K, D>
where
    K: Eq + Hash + Into<usize> + From<usize> + Clone + Ord,
    D: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_range, y_range): (RangeInclusive<usize>, RangeInclusive<usize>) =
            match self.ranges_as_usize() {
                Some(ranges) => ranges,
                _ => return Ok(()),
            };
        let last_y = y_range.end().clone();
        for y in y_range {
            for x in x_range.clone() {
                write!(
                    f,
                    "{}",
                    self.get(x.into(), y.into())
                        .map_or(" ".into(), |v| v.to_string())
                )?;
            }
            if last_y != y {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_grid() {
        let grid: HashGrid<usize, char> = HashGrid::from("123\n456\n789".to_string());
        assert_eq!(grid.to_string(), "123\n456\n789".to_string());
    }

    #[test]
    fn should_transpose_grid() {
        let mut grid: HashGrid<usize, char> = HashGrid::from("123\n456\n789".to_string());
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
            let mut grid: HashGrid<usize, char> = HashGrid::from("abc\ndef\nghi".to_string());
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
            let mut grid: HashGrid<usize, char> =
                HashGrid::from("ABCDEF\nGHIJKL\nMNOPQR".to_string());
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
            let mut grid: HashGrid<usize, char> =
                HashGrid::from("ABC\nDEF\nGHI\nJKL\nMNO\nPQR".to_string());
            grid.to_diagonal();
            assert_eq!(
                grid.to_string(),
                "     A  \n    D B \n   G E C\n  J H F \n M K I  \nP N L   \n Q O    \n  R     "
                    .to_string()
            );
        }
    }
}
