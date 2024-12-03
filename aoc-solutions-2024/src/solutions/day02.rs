use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 2)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        puzzle
            .get_input()
            .lines()
            .map(|line| line.split(' ').map(|d| d.parse::<u32>().unwrap()).collect())
            .filter(|r: &Vec<u32>| is_safe_report(r, false))
            .count()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        puzzle
            .get_input()
            .lines()
            .map(|line| line.split(' ').map(|d| d.parse::<u32>().unwrap()).collect())
            .filter(|r: &Vec<u32>| is_safe_report(r, true))
            .count()
            .into()
    }
}

fn is_safe_report(report: &[u32], problem_dampener: bool) -> bool {
    if report.len() < 2 {
        return true;
    }
    let is_increasing = report[0] < report[1];
    let mut prev = report[0];
    for i in 1..report.len() {
        let curr = report[i];
        if !is_valid_pair(prev, curr, is_increasing) {
            if problem_dampener {
                let (left, right) = report.split_at(i);
                return is_safe_report(&[left, &right[1..]].concat(), false) ||
                       is_safe_report(&[&left[..(i - 1)], right].concat(), false) ||
                       (
                         // drop the first element when we are comparing index 1 & 2
                         i <= 2 && is_safe_report(&report[1..], false)
                       );
            } else {
                return false;
            }
        } else {
            prev = curr;
        }
    }
    true
}

fn is_valid_pair(a: u32, b: u32, expect_increase: bool) -> bool {
    (expect_increase == (a < b)) && (1..=3).contains(&a.abs_diff(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#)
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 2.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 4.into());
    }
}
