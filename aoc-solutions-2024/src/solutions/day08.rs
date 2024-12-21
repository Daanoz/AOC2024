use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use aoc_core::{aoc_puzzle, tools::Grid, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 8, render)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let grid: Grid<usize, char> = puzzle.get_input().into();
        get_anti_node_count(grid, true).into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let grid: Grid<usize, char> = puzzle.get_input().into();
        get_anti_node_count(grid, false).into()
    }

    // Rendering logic, just for for extra
    fn render_part1(&self, puzzle: &Puzzle, renderer: Renderer) -> Option<Answer> {
        Some(do_render(&puzzle.get_input().into(), renderer, true).into())
    }
    fn render_part2(&self, puzzle: &Puzzle, renderer: Renderer) -> Option<Answer> {
        Some(do_render(&puzzle.get_input().into(), renderer, false).into())
    }
}

fn get_anti_node_count(grid: Grid<usize, char>, single: bool) -> usize {
    let bounds: (usize, usize) = grid.size();
    get_antennas(&grid)
        .into_values()
        .par_bridge()
        .flat_map(|freq_antennas| {
            get_all_interpolated_antennas(freq_antennas, bounds, single)
        })
        .collect::<HashSet<_>>()
        .len()
}

fn get_antennas(grid: &HashGrid<usize, char>) -> HashMap<char, Vec<(usize, usize)>> {
    grid.iter().fold(HashMap::new(), |mut map, (coord, char)| {
        if char != &'.' {
            map.entry(*char).or_default().push((*coord.0, *coord.1));
        }
        map
    })
}

fn get_all_interpolated_antennas(freq_antennas: Vec<(usize, usize)>, bounds: (usize, usize), single: bool) -> HashSet<(usize, usize)> {
    freq_antennas
        .iter()
        .flat_map(|a| {
            // Get all pairs of antennas
            freq_antennas
                .iter()
                .filter_map(move |b| if a == b { None } else { Some((*a, *b)) })
        })
        .flat_map(|(a, b)| {
            let dx = b.0 as isize - a.0 as isize;
            let dy = b.1 as isize - a.1 as isize;
            let mut anti_nodes = vec![];
            if !single {
                anti_nodes.push(a);
                anti_nodes.push(b);
            }

            // Extrapolate in the positive direction
            let mut x = b.0 as isize + dx;
            let mut y = b.1 as isize + dy;
            while x >= 0 && y >= 0 && (x as usize) < bounds.0 && (y as usize) < bounds.1 {
                anti_nodes.push((x as usize, y as usize));
                if single {
                    break;
                }
                x += dx;
                y += dy;
            }

            // Extrapolate in the negative direction
            let mut x = a.0 as isize - dx;
            let mut y = a.1 as isize - dy;
            while x >= 0 && y >= 0 && (x as usize) < bounds.0 && (y as usize) < bounds.1 {
                anti_nodes.push((x as usize, y as usize));
                if single {
                    break;
                }
                x += dx;
                y += dy;
            }
            anti_nodes
        })
        .collect::<HashSet<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 14.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 34.into());
    }
}

/// Keep rendering logic below all other stuff
/// Its reusing puzzle solution logic, but feeding it into the renderer
/// Performance is less of a concern
fn do_render(grid: &HashGrid<usize, char>, mut renderer: Renderer, single: bool) -> usize {
    let mut freq_colors: HashMap<char, (Color, usize)> = Default::default();
    renderer.grid_shape(0.0, 0.0);
    renderer.grid_with(&grid, |c, text, _| {
        if c == &'.' {
            return;
        }
        let next_color = (Color::from_palette(freq_colors.len()), freq_colors.len());
        let color = freq_colors.entry(*c).or_insert(next_color);
        text.with_color(color.0);
        text.with_content(c.to_string());
    });
    get_antennas(&grid)
        .into_iter()
        .flat_map(|(antenna, freq_antennas)| {
            let color = freq_colors.get(&antenna).unwrap();
            get_all_interpolated_antennas(freq_antennas, grid.size(), single)
                .into_iter()
                .inspect(|(x, y)| {
                    let indicator_size = 7; // 7x7 grid
                    let x_offset = (color.1 % indicator_size) as f32 / indicator_size as f32;
                    let y_offset = (color.1 / indicator_size) as f32 / indicator_size as f32;
                    let circle_size = 0.5 / indicator_size as f32;
                    renderer.grid_shape_with(*x as f32 + x_offset, *y as f32 + y_offset, |cell| {
                        cell.with_color(color.0);
                        cell.with_shape(GridShapeType::Circle(circle_size));
                    });
                })
                .collect::<HashSet<_>>()
        })        
        .collect::<HashSet<_>>()
        .len()
}