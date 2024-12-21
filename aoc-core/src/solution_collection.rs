use crate::{Answer, RenderPipeline, RunnableSolution};
use std::{collections::HashMap, time::Duration};

#[derive(Default)]
pub struct SolutionCollection {
    solutions: HashMap<u32, Box<dyn RunnableSolution>>,
    render_pipeline: Option<RenderPipeline>,
}

impl SolutionCollection {
    pub fn register_solution(&mut self, solution: Box<dyn RunnableSolution>) {
        self.solutions.insert(solution.get_day(), solution);
    }

    pub fn set_render_pipeline(&mut self, pipeline: RenderPipeline) {
        self.render_pipeline = Some(pipeline);
    }

    pub fn run(&self, day: Option<u32>) {
        if let Some(day) = day {
            self.run_day(&day);
        } else {
            let mut days = self.solutions.keys().collect::<Vec<_>>();
            days.sort();
            let mut total_time = Duration::default();
            for day in days {
                total_time += self.run_day(day);
            }
            println!("total_time: {:.2?}", total_time);
        }
    }

    fn run_day(&self, day: &u32) -> Duration {
        if !self.solutions.contains_key(day) {
            panic!("Day {} was not yet created", day);
        }
        let solution = &self.solutions.get(day).unwrap();
        let puzzle = solution.get_puzzle(); // Preload puzzle
        println!("Day {}", day);
        let (part1, time1) = crate::timed!(solution.part1(&puzzle));
        let (part2, time2) = crate::timed!(solution.part2(&puzzle));
        println!("Part 1: {}", display_answer(part1));
        println!("Part 2: {}", display_answer(part2));
        println!(
            "time: {:.2?} (1: {:.2?}, 2: {:.2?})",
            time1 + time2,
            time1,
            time2
        );
        time1 + time2
    }

    pub fn run_and_render_part1(&self, day: &u32) -> (Answer, std::time::Duration) {
        let solution = &self.solutions.get(day).unwrap();
        let mut puzzle = solution.get_puzzle(); // Preload puzzle
        puzzle.set_render_pipeline(self.render_pipeline.clone());
        crate::timed!(solution.part1(&puzzle))
    }
    pub fn run_and_render_part2(&self, day: &u32) -> (Answer, std::time::Duration) {
        let solution = &self.solutions.get(day).unwrap();
        let mut puzzle = solution.get_puzzle(); // Preload puzzle
        puzzle.set_render_pipeline(self.render_pipeline.clone());
        crate::timed!(solution.part2(&puzzle))
    }

    pub fn prepare_bench(
        &self,
        day: &u32,
    ) -> (impl Fn() -> Answer + use<'_>, impl Fn() -> Answer + use<'_>) {
        let solution = self.solutions.get(day).unwrap();
        let puzzle1 = solution.get_puzzle(); // Preload puzzle
        let puzzle2 = puzzle1.clone();
        (
            move || solution.part1(&puzzle1),
            move || solution.part2(&puzzle2),
        )
    }

    pub fn get_days(&self) -> Vec<u32> {
        self.solutions.keys().copied().collect()
	}

    pub fn get_days_with_render(&self) -> Vec<(u32, bool)> {
        let mut list = self
            .solutions
            .iter()
            .map(|d| (*d.0, d.1.can_render()))
            .collect::<Vec<_>>();
        list.sort_by(|a, b| a.0.cmp(&b.0));
        list
    }
}

fn display_answer(answer: Answer) -> String {
    match answer.get_result() {
        Ok(result) => result,
        Err(e) => e,
    }
}

/// Imports all the puzzle modules and returns a run function which can be called to run puzzles.
#[macro_export]
macro_rules! setup_solutions {
    ($($x:ident),+) => {
        aoc_core::include_solution_mod!($($x),+);

        pub fn get_collection() -> aoc_core::SolutionCollection {
            let mut puzzles = aoc_core::SolutionCollection::default();
            aoc_core::register_solution!(puzzles, $($x),+);
            puzzles
        }

        pub fn run(day: Option<u32>) {
            let puzzles = get_collection();
            puzzles.run(day);
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! include_solution_mod {
    ($x:ident) => {
        mod $x;
    };
    ($x:ident, $($y:ident),+) => (
        aoc_core::include_solution_mod!($x);
        aoc_core::include_solution_mod!($($y),+);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! register_solution {
    ($collection: ident, $x:ident) => {
        $x::register_solution(&mut $collection);
    };
    ($collection: ident, $x:ident, $($y:ident),+) => (
        aoc_core::register_solution!($collection, $x);
        aoc_core::register_solution!($collection, $($y),+);
    )
}
