use crate::{Answer, Puzzle, Renderer};

pub trait RunnableSolution: Send + Sync {
    fn get_puzzle(&self) -> Puzzle;
    fn part1(&self, puzzle: &Puzzle) -> Answer;
    fn part2(&self, puzzle: &Puzzle) -> Answer;
    fn get_day(&self) -> u32;
    fn can_render(&self) -> bool;
}

pub trait PuzzleSolution: Send + Sync {
    fn part1(&self, puzzle: &Puzzle) -> Answer;
    fn part2(&self, puzzle: &Puzzle) -> Answer;
    fn render_part1(&self, _puzzle: &Puzzle, _renderer: Renderer) -> Option<Answer> {
        None
    }
    fn render_part2(&self, _puzzle: &Puzzle, _renderer: Renderer) -> Option<Answer> {
        None
    }
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
    pub can_render: bool,
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
        #[cfg(feature = "render")]
        {
            if self.can_render() && puzzle.is_ready_to_render() {
                if let Some(answer) = self.solution.render_part1(puzzle, puzzle.renderer()) {
                    return answer;
                }
            }
        }
        self.solution.part1(puzzle)
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        #[cfg(feature = "render")]
        {
            if self.can_render() && puzzle.is_ready_to_render() {
                if let Some(answer) = self.solution.render_part2(puzzle, puzzle.renderer()) {
                    return answer;
                }
            }
        }
        self.solution.part2(puzzle)
    }

    fn get_day(&self) -> u32 {
        self.props.day
    }

    fn can_render(&self) -> bool {
        self.props.can_render
    }
}
