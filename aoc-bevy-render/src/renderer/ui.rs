use aoc_core::Answer;
use bevy::{prelude::*, tasks::Task};

use super::SolutionDisplayHandler;

mod day_buttons;
mod part_buttons;
mod style;

#[derive(Default)]
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveDay>()
            .add_event::<SelectDayEvent>()
            .add_systems(Startup, day_buttons::setup)
            .add_systems(Update, (day_buttons::update, day_buttons::event_listener))
            .add_systems(
                Update,
                (
                    part_buttons::update,
                    part_buttons::event_listener,
                    part_buttons::process_answer_task_result,
                ),
            );
    }
}

#[derive(Resource, Default)]
struct ActiveDay {
    day: Option<u32>,
    part1_task: Option<Task<AnswerStatus>>,
    part1: Option<AnswerStatus>,
    part2_task: Option<Task<AnswerStatus>>,
    part2: Option<AnswerStatus>,
}

impl ActiveDay {
    fn set_day(&mut self, day: u32) {
        self.day = Some(day);
        self.part1_task = None;
        self.part1 = None;
        self.part2_task = None;
        self.part2 = None;
    }
    fn clear(&mut self) {
        self.day = None;
        self.part1_task = None;
        self.part1 = None;
        self.part2_task = None;
        self.part2 = None;
    }
}

struct AnswerStatus {
    answer: Answer,
    duration: std::time::Duration,
}

impl From<(Answer, std::time::Duration)> for AnswerStatus {
    fn from((answer, duration): (Answer, std::time::Duration)) -> Self {
        Self { answer, duration }
    }
}

#[derive(Event)]
struct SelectDayEvent {
    day: Option<u32>,
}
