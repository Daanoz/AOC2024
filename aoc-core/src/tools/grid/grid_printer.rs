use super::Grid;
use std::fmt::Display;

type OverrideFn<K> = dyn Fn((K, K)) -> Option<String>;

pub struct GridPrinter<'g, K, D> {
    grid: &'g Grid<K, D>,
    legend: bool,
    cell_width: usize,
    cell_fill: Vec<D>,
    cell_override_fn: Option<Box<OverrideFn<K>>>,
}
impl<'g, K, D> GridPrinter<'g, K, D>
where
    K: Copy + Ord + std::ops::Add<Output = K> + TryFrom<u8> + Display,
    <K as TryFrom<u8>>::Error: std::fmt::Debug,
    D: Eq + ToString,
{
    pub(super) fn new(grid: &'g Grid<K, D>) -> Self {
        Self {
            grid,
            legend: false,
            cell_width: 1,
            cell_fill: Vec::new(),
            cell_override_fn: None,
        }
    }
    /// Print an X/Y legend on the side of the grid
    pub fn with_legend(mut self) -> Self {
        self.legend = true;
        self
    }
    /// Print all cells with a specific width
    pub fn with_cell_width(mut self, width: usize) -> Self {
        self.cell_width = width;
        self
    }
    /// Print cells with given content to fill the cell instead of using empty space
    pub fn with_cell_fill(mut self, content: D) -> Self {
        self.cell_fill.push(content);
        self
    }
    /// Override cell contents with custom content
    pub fn with_cell_override_fn<F>(mut self, cell_override_fn: F) -> Self
    where
        F: Fn((K, K)) -> Option<String> + 'static,
    {
        self.cell_override_fn = Some(Box::new(cell_override_fn));
        self
    }
    /// Print grid
    pub fn print(self) {
        println!("{}", self);
    }

    fn format_value<T: Display>(&self, value: T) -> String {
        let mut value = value.to_string();
        if value.len() > self.cell_width {
            value = value.split_at(value.len() - self.cell_width).0.to_string();
        }
        format!("{:^width$}", value, width = self.cell_width)
    }
}
impl<'g, K, D> Display for GridPrinter<'g, K, D>
where
    K: Copy + Ord + std::ops::Add<Output = K> + TryFrom<u8> + Display,
    <K as TryFrom<u8>>::Error: std::fmt::Debug,
    D: Eq + ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid_iter = self.grid.grid_iter();
        if self.legend {
            write!(f, "{}", self.format_value(' '))?;
            for (x, _) in grid_iter.x_iter() {
                write!(f, "{}", self.format_value(x))?;
            }
            writeln!(f)?;
        }

        let mut last_y = None;
        for (x, y) in grid_iter {
            if last_y.is_none() {
                if self.legend {
                    write!(f, "{}", self.format_value(y))?;
                }
            } else if last_y != Some(y) {
                writeln!(f)?;
                if self.legend {
                    write!(f, "{}", self.format_value(y))?;
                }
            }
            let cell = if let Some(cell_override_fn) = &self.cell_override_fn {
                cell_override_fn((x, y))
            } else {
                None
            };
            let cell = cell.unwrap_or_else(|| {
                self.grid.get(x, y).map_or(" ".to_string(), |v| {
                    if self.cell_fill.contains(v) {
                        let mut content = v.to_string().repeat(self.cell_width);
                        content.truncate(self.cell_width);
                        content
                    } else {
                        v.to_string()
                    }
                })
            });
            write!(f, "{}", self.format_value(cell))?;
            last_y = Some(y);
        }
        Ok(())
    }
}
