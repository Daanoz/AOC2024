use std::{collections::HashMap, fmt::{Debug, Write}};

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 21)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let pad = create_keypads(2);
        puzzle
            .get_input()
            .lines()
            .map(|code| {
                let moves: usize = get_move_count(pad.clone(), code);
                moves * code_as_uint(code)
            })
            .sum::<usize>()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let pad = create_keypads(25);
        puzzle
            .get_input()
            .lines()
            .map(|code| {
                let moves = get_move_count(pad.clone(), code);
                moves * code_as_uint(code)
            })
            .sum::<usize>()
            .into()
    }
}

fn create_keypads(c: u32) -> Keypad {
    let mut pad = Keypad::new_code_pad();
    for _ in 0..c {
        pad = Keypad::new_arrow_pad(pad);
    }
    pad
}

fn code_as_uint(code: &str) -> usize {
    code.trim_start_matches('0')
        .trim_end_matches('A')
        .parse()
        .unwrap()
}

fn get_move_count(mut keypad: Keypad, code: &str) -> usize {
    let mut count = 0;
    for c in code.chars() {
        for (moves, c) in keypad.get_moves_to(c) {
            count += c * moves.len();
        }
    }
    count
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    Accept,
}
impl Move {
    fn as_char(&self) -> char {
        match self {
            Move::Up => '^',
            Move::Down => 'v',
            Move::Left => '<',
            Move::Right => '>',
            Move::Accept => 'A',
        }
    }
}
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}
impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Clone)]
struct Keypad {
    position: (u32, u32),
    field_map: HashMap<char, (u32, u32)>,
    next: Option<Box<Keypad>>,
}
impl Keypad {
    fn new_code_pad() -> Self {
        Self {
            position: (2, 3),
            field_map: HashMap::from([
                ('7', (0, 0)),
                ('8', (1, 0)),
                ('9', (2, 0)),
                ('4', (0, 1)),
                ('5', (1, 1)),
                ('6', (2, 1)),
                ('1', (0, 2)),
                ('2', (1, 2)),
                ('3', (2, 2)),
                ('0', (1, 3)),
                ('A', (2, 3)),
            ]),
            next: None,
        }
    }
    fn new_arrow_pad(next: Keypad) -> Self {
        Self {
            position: (2, 0),
            field_map: HashMap::from([
                ('^', (1, 0)),
                ('A', (2, 0)),
                ('<', (0, 1)),
                ('v', (1, 1)),
                ('>', (2, 1)),
            ]),
            next: Some(Box::new(next)),
        }
    }

    fn get_own_moves_to(&mut self, target_char: char) -> Vec<Move> {
        let target = self.field_map.get(&target_char).unwrap();
        let mut current = self.position;
        let mut moves = vec![];
        while &current != target {
            let next = if current.1 == 0 && target.0 == 0 {
                // Avoid move left first if it takes us through top left
                vec![Move::Down; (target.1 - current.1) as usize]
            } else if current.0 > target.0 {
                // Moving left first is preferable
                vec![Move::Left; (current.0 - target.0) as usize]
            } else if current.1 > target.1 && current.0 != 0 {
                // Move up second, if we can, ie. not first column
                vec![Move::Up; (current.1 - target.1) as usize]
            } else if current.1 < target.1 {
                // Move down third
                vec![Move::Down; (target.1 - current.1) as usize]
            } else if current.0 < target.0 {
                // Move right fourth
                vec![Move::Right; (target.0 - current.0) as usize]
            } else if current.1 > target.1 {
                // Move up if we skipped due to top left
                vec![Move::Up; (current.1 - target.1) as usize]
            } else {
                unreachable!();
            };
            current = make_moves(current, &next);
            moves.extend(next);
        }
        self.position = current;
        moves
    }

    fn get_moves_to(&mut self, value: char) -> HashMap<Vec<Move>, usize> {
        let mut move_chunks: HashMap<Vec<Move>, usize> = HashMap::new();
        if let Some(next) = self.next.as_mut() {
            let child_move_map = next.get_moves_to(value);
            for (child_moves, count) in child_move_map {
                for child_move in child_moves {
                    let mut moves = self.get_own_moves_to(child_move.as_char());
                    moves.push(Move::Accept);
                    move_chunks
                        .entry(moves)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
            }
        } else {
            let target = self.field_map.get(&value).unwrap();
            let mut current = self.position;
            let mut moves = vec![];
            while &current != target {
                let next = if current.1 == 3 && target.0 == 0 {
                    // Avoid move left first if it takes us through bottom left
                    vec![Move::Up; (current.1 - target.1) as usize]
                } else if current.0 > target.0 {
                    // Moving left first is preferable
                    vec![Move::Left; (current.0 - target.0) as usize]
                } else if current.1 > target.1 {
                    // Move up second
                    vec![Move::Up; (current.1 - target.1) as usize]
                } else if current.1 < target.1 && !(current.0 == 0 && target.1 == 3) {
                    // Move down third, if we can, ie. not first column, and target not last
                    vec![Move::Down; (target.1 - current.1) as usize]
                } else if current.0 < target.0 {
                    // Move right fourth
                    vec![Move::Right; (target.0 - current.0) as usize]
                } else if current.1 < target.1 {
                    // Move down if we skipped due to bottom left
                    vec![Move::Down; (target.1 - current.1) as usize]
                } else {
                    unreachable!();
                };
                current = make_moves(current, &next);
                moves.extend(next);
            }
            self.position = current;
            moves.push(Move::Accept);
            move_chunks.insert(moves, 1);
        }
        move_chunks
    }
}

fn make_move((x, y): (u32, u32), direction: &Move) -> (u32, u32) {
    match direction {
        Move::Up => (x, y - 1),
        Move::Down => (x, y + 1),
        Move::Left => (x - 1, y),
        Move::Right => (x + 1, y),
        Move::Accept => panic!("Cannot move to Accept"),
    }
}

fn make_moves(xy: (u32, u32), directions: &[Move]) -> (u32, u32) {
    let mut next = xy;
    for dir in directions {
        next = make_move(next, dir);
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"029A
980A
179A
456A
379A"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 126384.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 154115708116294_usize.into());
    }
}
