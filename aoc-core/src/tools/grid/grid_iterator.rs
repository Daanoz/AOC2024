use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct GridIterator<K> {
    x_range: RangeInclusive<K>,
    y_range: RangeInclusive<K>,
    one: K,
    head: Option<(K, K)>,
    tail: Option<(K, K)>,
}
impl<K> GridIterator<K>
where
    K: Copy + Ord + std::ops::Add<Output = K> + TryFrom<u8>,
{
    pub(super) fn new(x_range: RangeInclusive<K>, y_range: RangeInclusive<K>) -> Self {
        // Ugh, no better interface AFAIK to step through ranges based on generics (Step trait is unstable)
        let one: K = match 1_u8.try_into() {
            Ok(one) => one,
            _ => unreachable!("All 1_u8 values should be convertible to K"),
        };
        Self {
            head: Some((*x_range.start(), *y_range.start())),
            tail: Some((*x_range.end(), *y_range.end())),
            x_range,
            y_range,
            one,
        }
    }

    pub fn get_one(&self) -> K {
        self.one
    }

    pub fn x_iter(&self) -> Self {
        Self::new(self.x_range.clone(), self.one..=self.one)
    }

    pub fn y_iter(&self) -> Self {
        Self::new(self.one..=self.one, self.y_range.clone())
    }
}

impl<K> Iterator for GridIterator<K>
where
    K: Copy + Ord + std::ops::Add<Output = K> + TryFrom<u8>,
{
    type Item = (K, K);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (mut x, mut y) = self.head?;
        if (x, y) == self.tail? {
            return std::mem::take(&mut self.head);
        }
        x = x.add(self.one);
        if x > *self.x_range.end() {
            x = *self.x_range.start();
            y = y.add(self.one);
        }
        std::mem::replace(&mut self.head, Some((x, y)))
    }
}

impl<K> DoubleEndedIterator for GridIterator<K>
where
    K: Copy + Ord + std::ops::Add<Output = K> + std::ops::Sub<Output = K> + TryFrom<u8>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (mut x, mut y) = self.tail?;
        if (x, y) == self.head? {
            return std::mem::take(&mut self.tail);
        }
        if x < self.one {
            x = *self.x_range.end();
            y = y.sub(self.one);
        } else {
            x = x.sub(self.one);
        }
        std::mem::replace(&mut self.tail, Some((x, y)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid_iterator() {
        let grid = GridIterator::<u32>::new(0..=2, 0..=2);
        assert_eq!(
            grid.map(|(x, y)| (x, y)).collect::<Vec<_>>(),
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (0, 1),
                (1, 1),
                (2, 1),
                (0, 2),
                (1, 2),
                (2, 2),
            ]
        );
    }

    #[test]
    fn test_grid_iterator_rev() {
        let grid = GridIterator::<u32>::new(0..=2, 0..=2);
        assert_eq!(
            grid.rev().map(|(x, y)| (x, y)).collect::<Vec<_>>(),
            vec![
                (2, 2),
                (1, 2),
                (0, 2),
                (2, 1),
                (1, 1),
                (0, 1),
                (2, 0),
                (1, 0),
                (0, 0),
            ]
        );
    }
}
