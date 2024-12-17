use std::str::FromStr;

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 17)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let mut program: Program = puzzle.get_input().parse().unwrap();
        program
            .run()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let mut program: Program = puzzle.get_input().parse().unwrap();

        // Find where the number of output elements start to equal the number of instructions
        let mut i = 1;
        loop {
            program.registers.reset();
            program.registers.a = i;
            if program.run().len() == program.instructions.len() {
                break;
            }
            i *= 2;
        }

        let mut start = i / 2;
        // Digits after the first change every n^8
        for digits in 1..=program.instructions.len() {
            // Calculate a nice & fast delta based on n^8
            let delta = 8_usize.pow((program.instructions.len() - (digits + 1)) as u32);
            
            // For this loop, only look at the last n digits
            let slice_range = program.instructions.len() - digits;

            // Take the last known wrong position, where we know the good number is close
            let mut i = start;
            loop {
                program.registers.reset();
                program.registers.a = i;
                let out = program.run();
                if out.len() == program.instructions.len() {
                    if out == program.instructions {
                        return i.into();
                    }
                    if out[slice_range..] == program.instructions[slice_range..] {
                        // The last n digits match, rollback to last wrong number, and start next loop
                        start = i - delta;
                        break;
                    }
                }
                i += delta;
            }
        }
        0.into()
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<usize>,
    registers: Registers,
}

impl Program {
    fn run(&mut self) -> Vec<usize> {
        let mut p = 0;
        let mut outputs = vec![];
        while let Some(instr) = self.instructions.get(p) {
            match *instr {
                0 => {
                    // Adv
                    let operand = self.registers.read_combo(self.instructions[p + 1]);
                    self.registers.a /= 2_usize.pow(operand as u32);
                }
                1 => {
                    // Bxl
                    let operand = self.instructions[p + 1];
                    self.registers.b ^= operand;
                }
                2 => {
                    // Bst
                    let operand = self.registers.read_combo(self.instructions[p + 1]);
                    self.registers.b = operand % 8;
                }
                3 => {
                    // Jnz
                    if self.registers.a != 0 {
                        p = self.registers.read_combo(self.instructions[p + 1]);
                        continue;
                    }
                }
                4 => {
                    // Bxc
                    self.registers.b ^= self.registers.c;
                }
                5 => {
                    // Out
                    let operand = self.registers.read_combo(self.instructions[p + 1]);
                    outputs.push(operand % 8);
                }
                6 => {
                    // Bdv
                    let operand = self.registers.read_combo(self.instructions[p + 1]);
                    self.registers.b = self.registers.a / 2_usize.pow(operand as u32);
                }
                7 => {
                    // Cdv
                    let operand = self.registers.read_combo(self.instructions[p + 1]);
                    self.registers.c = self.registers.a / 2_usize.pow(operand as u32);
                }
                _ => unimplemented!("Instruction not implemented"),
            }
            p += 2;
        }
        outputs
    }
}

impl FromStr for Program {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (registers, instructions) = s.split_once("\n\n").ok_or("Could not split")?;
        let registers: Registers = registers.parse()?;
        let program = instructions
            .strip_prefix("Program: ")
            .ok_or("Missing prefix")?
            .trim()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self {
            instructions: program,
            registers,
        })
    }
}

#[derive(Debug)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
    initial_a: usize,
    initial_b: usize,
    initial_c: usize,
}

impl Registers {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            a,
            b,
            c,
            initial_a: a,
            initial_b: b,
            initial_c: c,
        }
    }

    fn read_combo(&self, combo: usize) -> usize {
        match combo {
            0..=3 => combo,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unimplemented!("Invalid combo"),
        }
    }

    fn reset(&mut self) {
        self.a = self.initial_a;
        self.b = self.initial_b;
        self.c = self.initial_c;
    }
}

impl FromStr for Registers {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        for line in s.lines() {
            let (reg, val) = line
                .strip_prefix("Register ")
                .ok_or("Missing prefix")?
                .split_once(": ")
                .ok_or("Missing delimiter")?;
            match reg {
                "A" => a = val.parse()?,
                "B" => b = val.parse()?,
                "C" => c = val.parse()?,
                _ => unreachable!(),
            }
        }
        Ok(Self::new(a, b, c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#,
        )
    }

    fn get_puzzle_b() -> Puzzle {
        Puzzle::from(
            r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#,
        )
    }

    
    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0".into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle_b());
        assert_eq!(result, 117440.into());
    }

    mod operations {
        use super::*;

        #[test]
        fn case1() {
            let mut program = Program {
                instructions: vec![2, 6],
                registers: Registers::new(0, 0, 9),
            };
            assert!(program.run().is_empty());
            assert_eq!(program.registers.b, 1);
        }

        #[test]
        fn case2() {
            let mut program = Program {
                instructions: vec![5, 0, 5, 1, 5, 4],
                registers: Registers::new(10, 0, 0),
            };
            assert_eq!(program.run(), vec![0, 1, 2]);
        }

        #[test]
        fn case3() {
            let mut program = Program {
                instructions: vec![0, 1, 5, 4, 3, 0],
                registers: Registers::new(2024, 0, 0),
            };
            assert_eq!(program.run(), vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
            assert_eq!(program.registers.a, 0);
        }

        #[test]
        fn case4() {
            let mut program = Program {
                instructions: vec![1, 7],
                registers: Registers::new(0, 29, 0),
            };
            let _ = program.run();
            assert_eq!(program.registers.b, 26);
        }

        #[test]
        fn case5() {
            let mut program = Program {
                instructions: vec![4, 0],
                registers: Registers::new(0, 2024, 43690),
            };
            assert!(program.run().is_empty());
            assert_eq!(program.registers.b, 44354);
        }
    }
}
