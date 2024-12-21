use crate::{Answer, Puzzle};

pub trait RunnableSolution: Send + Sync {
    fn get_puzzle(&self) -> Puzzle;
    fn part1(&self, puzzle: &Puzzle) -> Answer;
    fn part2(&self, puzzle: &Puzzle) -> Answer;
    fn get_day(&self) -> u32;
}

pub trait PuzzleSolution: Send + Sync {
    fn part1(&self, puzzle: &Puzzle) -> Answer;
    fn part2(&self, puzzle: &Puzzle) -> Answer;
}

pub struct SolutionWrapper<S>
where
    S: PuzzleSolution,
{
    solution: S,
    props: SolutionProps,
}

pub struct SolutionProps {
    pub year: u32,
    pub day: u32,
}

impl<S> SolutionWrapper<S>
where
    S: PuzzleSolution,
{
    pub fn new(solution: S, props: SolutionProps) -> Self {
        Self { solution, props }
    }
}

impl<P> RunnableSolution for SolutionWrapper<P>
where
    P: PuzzleSolution,
{
    fn get_puzzle(&self) -> Puzzle {
        Puzzle::new(self.props.day, self.props.year)
    }

    fn part1(&self, puzzle: &Puzzle) -> Answer {
        self.solution.part1(puzzle)
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        self.solution.part2(puzzle)
    }

    fn get_day(&self) -> u32 {
        self.props.day
    }
}
