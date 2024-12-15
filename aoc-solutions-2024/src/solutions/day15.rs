use std::collections::BTreeSet;

use aoc_core::{aoc_puzzle, tools::Grid, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 15)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let (grid, moves) = puzzle.input_as_str().split_once("\n\n").unwrap();
        let grid: Grid<usize, char> = Grid::from(grid.to_string());
        let moves = moves
            .replace("\n", "")
            .chars()
            .map(Move::from_char)
            .collect::<Result<Vec<_>, _>>()
            .expect("Valid moves");
        let mut warehouse = Warehouse::from(grid);
        warehouse.make_moves(moves);
        get_score(&warehouse.boxes).into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let (grid, moves) = puzzle.input_as_str().split_once("\n\n").unwrap();
        let grid: Grid<usize, char> = Grid::from(grid.to_string());
        let moves = moves
            .replace("\n", "")
            .chars()
            .map(Move::from_char)
            .collect::<Result<Vec<_>, _>>()
            .expect("Valid moves");
        let mut warehouse = Warehouse::from(grid).make_wide();
        warehouse.make_moves(moves);
        get_score(&warehouse.boxes).into()
    }
}

struct Warehouse {
    robot: (usize, usize),
    walls: BTreeSet<(usize, usize)>,
    boxes: BTreeSet<(usize, usize)>,
}

impl Warehouse {
    fn make_moves(&mut self, moves: Vec<Move>) {
        for m in moves {
            self.make_move(m);
        }
    } 

    fn make_move(&mut self, m: Move) {
        let next_robot_pos = m.move_coord(&self.robot);
        if self.walls.contains(&next_robot_pos) {
            return;
        }
        if self.boxes.contains(&next_robot_pos) {
            let mut next_box_pos = m.move_coord(&next_robot_pos);
            loop {
                if self.walls.contains(&next_box_pos) {
                    // We cannot move the boxes
                    return;
                }
                if self.boxes.contains(&next_box_pos) {
                    // There is another box in the way
                    next_box_pos = m.move_coord(&next_box_pos);
                    continue;
                }
                // Move the box stack by moving the first to the next position
                self.boxes.remove(&next_robot_pos);
                self.boxes.insert(next_box_pos);
                break;
            }
        }
        self.robot = next_robot_pos;
    }

    fn make_wide(self) -> WideWarehouse {
        WideWarehouse {
            robot: (self.robot.0 * 2, self.robot.1),
            walls: self
                .walls
                .iter()
                .flat_map(|(x, y)| [(x * 2, *y), (x * 2 + 1, *y)])
                .collect(),
            boxes: self.boxes.iter().map(|(x, y)| (x * 2, *y)).collect(),
        }
    }
}

struct WideWarehouse {
    robot: (usize, usize),
    walls: BTreeSet<(usize, usize)>,
    boxes: BTreeSet<(usize, usize)>,
}

impl WideWarehouse {
    fn make_moves(&mut self, moves: Vec<Move>) {
        for m in moves {
            self.make_move(m);
        }
    } 
    
    fn make_move(&mut self, m: Move) {
        let next_robot_pos = m.move_coord(&self.robot);
        if self.walls.contains(&next_robot_pos) {
            // There is a wall, we cannot move
            return;
        }
        let box_collision_pos_left = (next_robot_pos.0 - 1, next_robot_pos.1);
        // Check if the space we are moving into contains a box
        let collided_box = if self.boxes.contains(&next_robot_pos) {
            Some(next_robot_pos)
        } else if self.boxes.contains(&box_collision_pos_left) {
            Some(box_collision_pos_left)
        } else {
            None
        };

        if let Some(collided_box) = collided_box {
            // The stack of boxes that need to be moved if possible
            let mut box_stack: BTreeSet<(usize, usize)> = BTreeSet::new();
            // The list of coordinates that need to be checked for collisions
            let mut box_collision_checks = vec![collided_box];
            while let Some(check) = box_collision_checks.pop() {
                if let Some(collided_boxes) = self.can_move_box(check, m) {
                    box_stack.insert(check);
                    for collided_box in collided_boxes {
                        if box_stack.contains(&collided_box) {
                            // We have already checked this box
                            continue;
                        }
                        box_collision_checks.push(collided_box);
                    }
                } else {
                    // Collided into a wall
                    return;
                }
            }

            // Move all the boxes
            for box_pos in &box_stack {
                self.boxes.remove(box_pos);
            }
            for box_pos in &box_stack {
                let next_box_pos: (usize, usize) = m.move_coord(box_pos);
                self.boxes.insert(next_box_pos);
            }
        }
        self.robot = next_robot_pos;
    }

    fn can_move_box(&self, box_pos: (usize, usize), m: Move) -> Option<Vec<(usize, usize)>> {
        // Next left and right box coordinates after moving
        let nlc = m.move_coord(&box_pos);
        let nrc = m.move_coord(&(box_pos.0 + 1, box_pos.1));

        // Check for wall collisions
        if match m {
            Move::Up | Move::Down => self.walls.contains(&nlc) || self.walls.contains(&nrc),
            Move::Left => self.walls.contains(&nlc),
            Move::Right => self.walls.contains(&nrc),
        } {
            return None;
        }

        // Check for box collisions
        match m {
            Move::Left => {
                let box_collision_coord = (nlc.0 - 1, nlc.1);
                if self.boxes.contains(&box_collision_coord) {
                    return Some(vec![box_collision_coord]);
                }
                Some(vec![])
            }
            Move::Right => {
                let box_collision_coord = (nrc.0, nrc.1);
                if self.boxes.contains(&box_collision_coord) {
                    return Some(vec![box_collision_coord]);
                }
                Some(vec![])
            }
            Move::Up | Move::Down => {
                Some(
                    vec![
                        (nlc.0 - 1, nlc.1), // Left above
                        (nlc.0, nlc.1),     // Directly above
                        (nrc.0, nrc.1),     // Right above
                    ]
                    .into_iter()
                    .filter(|c| self.boxes.contains(c))
                    .collect(),
                )
            }
        }
    }
}

fn get_score(boxes: &BTreeSet<(usize, usize)>) -> usize {
    boxes.iter().map(|(x, y)| x + (100 * y)).sum::<usize>()
}

impl From<Grid<usize, char>> for Warehouse {
    fn from(grid: Grid<usize, char>) -> Self {
        let robot = grid.iter().find(|(_, &c)| c == '@').unwrap().0;
        let walls = grid
            .iter()
            .filter(|(_, c)| **c == '#')
            .map(|(pos, _)| (*pos.0, *pos.1))
            .collect();
        let boxes = grid
            .iter()
            .filter(|(_, c)| **c == 'O')
            .map(|(pos, _)| (*pos.0, *pos.1))
            .collect();
        Self {
            robot: (*robot.0, *robot.1),
            walls,
            boxes,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn from_char(s: char) -> Result<Self, String> {
        match s {
            'v' => Ok(Self::Down),
            '^' => Ok(Self::Up),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            c => Err(format!("Invalid move: {c}")),
        }
    }

    /// Assuming the grid is protected by walls, and we check for walls before moving
    fn move_coord(&self, coord: &(usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (coord.0, coord.1 - 1),
            Self::Down => (coord.0, coord.1 + 1),
            Self::Left => (coord.0 - 1, coord.1),
            Self::Right => (coord.0 + 1, coord.1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 10092.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 9021.into());
    }
}
