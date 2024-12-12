use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 11)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        get_stones_after(puzzle.get_input(), 25).into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        get_stones_after(puzzle.get_input(), 75).into()
    }
}

fn get_stones_after(input: String, count: u32) -> u64 {
    use std::collections::HashMap;
    let mut stones: HashMap<u64, u64> = input.split_ascii_whitespace().map(|s| s.parse().unwrap()).fold(HashMap::new(), |mut map, value| {
        map.entry(value).and_modify(|c| *c += 1).or_insert(1);
        map
    });

    for _ in 0..count {
        let mut new_stones = HashMap::new();
        let mut val_as_string = String::new();
        for (stone, count) in stones.into_iter() {
            match stone {
                0 => {
                    new_stones.entry(1).and_modify(|c| *c += count).or_insert(count); 
                },
                s if is_value_str_mod_2(s, &mut val_as_string) => {
                    let left_stone: u64 = val_as_string[..(val_as_string.len() / 2)].parse().unwrap();
                    let right_stone:u64 = val_as_string[(val_as_string.len() / 2)..].parse().unwrap();
                    new_stones.entry(left_stone).and_modify(|c| *c += count).or_insert(count);
                    new_stones.entry(right_stone).and_modify(|c| *c += count).or_insert(count);
                },
                s => {
                    let val = s * 2024;
                    new_stones.entry(val).and_modify(|c| *c += count).or_insert(count); 
                }
            }
        };
        stones = new_stones;
    }
    stones.values().sum()
}

/// Check if the str value length is even, use a mutable string to avoid
/// double casting to string
fn is_value_str_mod_2(input: u64, string_value: &mut String) -> bool {
    *string_value = input.to_string();
    string_value.len() % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(r#"125 17"#)
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 55312.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 65601038650482_u64.into()); // Based on my own calculations
    }
}