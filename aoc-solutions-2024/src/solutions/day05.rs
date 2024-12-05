use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 5)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let (rules, jobs) = get_rules_and_jobs(puzzle.to_string());
        jobs.into_iter()
            .filter(|job| rules.is_valid_job(job))
            .map(|job| job.get_job_id())
            .sum::<u32>()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let (rules, jobs) = get_rules_and_jobs(puzzle.to_string());
        jobs.into_iter()
            .filter(|job| !rules.is_valid_job(job))
            .map(|job| rules.fix_job(job))
            .map(|job| job.get_job_id())
            .sum::<u32>()
            .into()
    }
}

fn get_rules_and_jobs(input: String) -> (PrintRules, Vec<PrintJob>) {
    let (rules, jobs) = input.split_once("\n\n").expect("Valid input");
    (
        PrintRules::from_str(rules).expect("Valid rules"),
        jobs.lines()
            .map(|job| PrintJob::from_str(job).expect("Valid job"))
            .collect::<Vec<_>>(),
    )
}

#[derive(Debug)]
struct PrintRules {
    rules: HashMap<u32, (HashSet<u32>, HashSet<u32>)>,
}

impl PrintRules {
    fn is_valid_job(&self, job: &PrintJob) -> bool {
        for (index, page) in job.pages.iter().enumerate() {
            if let Some((before, after)) = self.rules.get(page) {
                for page_before in &job.pages[..index] {
                    if !before.contains(page_before) {
                        return false;
                    }
                }
                for page_after in &job.pages[index + 1..] {
                    if !after.contains(page_after) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn fix_job(&self, mut job: PrintJob) -> PrintJob {
        job.pages.sort_unstable_by(|a, b| {
            let (_, after) = match self.rules.get(a) {
                Some((before, after)) => (before, after),
                None => return std::cmp::Ordering::Equal,
            };
            if after.contains(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        job
    }
}

impl FromStr for PrintRules {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s
            .lines()
            .map(|line| {
                let (left, right) = line.split_once("|").ok_or("Invalid rule")?;
                Ok((
                    left.parse().map_err(|_| "Invalid page number")?,
                    right.parse().map_err(|_| "Invalid page number")?,
                ))
            })
            .collect::<Result<HashSet<(u32, u32)>, Self::Err>>()?
            .into_iter()
            .fold(HashMap::<u32, (HashSet<u32>, HashSet<u32>)>::new(), |mut map, (a, b)| {
                map.entry(b).or_default().0.insert(a);
                map.entry(a).or_default().1.insert(b);                
                map
            });

        Ok(PrintRules { rules })
    }
}

#[derive(Debug)]
struct PrintJob {
    pages: Vec<u32>,
}
impl PrintJob {
    fn get_job_id(&self) -> u32 {
        self.pages[(self.pages.len() - 1) / 2]
    }
}
impl FromStr for PrintJob {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages = s
            .split(',')
            .map(|page| page.parse().map_err(|_| "Invalid page number".to_string()))
            .collect::<Result<Vec<_>, Self::Err>>()?;
        Ok(PrintJob { pages })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 143.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 123.into());
    }
}
