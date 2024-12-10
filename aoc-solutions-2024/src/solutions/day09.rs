use std::{collections::VecDeque, rc::Rc, str::FromStr};

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 9)]
#[derive(Default)]
pub struct Day;

// to high: 23679480864478

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let disk: Disk = puzzle.get_input().parse().unwrap();
        let compressed_disk = disk.compress_blocks();
        compressed_disk.checksum().into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let disk: Disk = puzzle.get_input().parse().unwrap();
        let compressed_disk = disk.compress_files();
        compressed_disk.checksum().into()
    }
}

#[derive(Debug)]
struct Disk {
    files: Vec<Rc<File>>,
    blocks: Vec<Block>,
}

impl Disk {
    fn compress_blocks(self) -> Self {
        let mut blocks = vec![];
        let mut old_blocks = VecDeque::from(self.blocks.clone());
        while !old_blocks.is_empty() {
            match old_blocks.pop_front().unwrap() {
                Block::File(f) => {
                    blocks.push(Block::File(f));
                    continue;
                },
                Block::Free => loop {
                    match old_blocks.pop_back() {
                        Some(Block::File(f)) => {
                            blocks.push(Block::File(f));
                            break;
                        },
                        Some(Block::Free) => continue,
                        None => break,
                    }
                }
            };
        }
        Self { files: self.files.clone(), blocks }
    }

    fn compress_files(mut self) -> Self {
        for file in self.files.iter().rev() {
            let target_size = file.size;
            let mut start_index = 0;
            let mut current_block_length = 0;
            for (index, block) in self.blocks.iter().enumerate() {
                match block {
                    Block::File(f) => {
                        if f.id == file.id {
                            break; // We found ourselves, abort
                        } else {
                            current_block_length = 0;
                            start_index = index + 1;
                        }
                    },
                    Block::Free => {
                        current_block_length += 1;
                        if current_block_length == target_size {
                            // Remove block from old position
                            for block in self.blocks.iter_mut() {
                                match block {
                                    Block::File(f) => {
                                        if f.id == file.id {
                                            *block = Block::Free;
                                        }
                                    },
                                    Block::Free => {},
                                }
                            }
                            // Add block to new position
                            for i in 0..target_size {
                                self.blocks[start_index + i] = Block::File(file.clone());
                            }
                            break;
                        }
                    },
                }
            }
        }
        self
    }

    fn checksum(&self) -> usize {
        self.blocks.iter().enumerate().map(|(i, b)| match b {
            Block::File(f) => f.id as usize * i,
            Block::Free => 0,
        }).sum()
    }
}

impl FromStr for Disk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut files = Vec::new();
        let mut is_file = true;
        let mut block_id = 0;
        let blocks = s
            .trim()
            .chars()
            .flat_map(|c| {
                let size = c.to_digit(10).unwrap_or_else(|| panic!("Unexpected: {c}")) as usize;
                let blocks = if is_file {
                    let file = Rc::new(File { id: block_id, size });
                    block_id += 1;
                    files.push(file.clone());
                    vec![Block::File(file); size]
                } else {
                    vec![Block::Free; size]
                };
                is_file = !is_file;
                blocks
            })
            .collect::<Vec<_>>();

        Ok(Self { files, blocks })
    }
}

#[derive(Debug, Clone)]
enum Block {
    File(Rc<File>),
    Free,
}

#[derive(Debug)]
struct File {
    id: u32,
    size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle_1() -> Puzzle {
        Puzzle::from(r#"12345"#)
    }
    fn get_puzzle_2() -> Puzzle {
        Puzzle::from(r#"2333133121414131402"#)
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle_1());
        assert_eq!(result, 60.into());
        let result = Day::default().part1(&get_puzzle_2());
        assert_eq!(result, 1928.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle_2());
        assert_eq!(result, 2858.into());
    }
}
