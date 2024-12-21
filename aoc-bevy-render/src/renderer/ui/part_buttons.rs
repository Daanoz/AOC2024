#![allow(clippy::type_complexity)]

use aoc_core::Answer;
use bevy::{
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool},
};

use super::{style::*, ActiveDay, SelectDayEvent, SolutionDisplayHandler};

#[derive(Component)]
pub struct RunPartButtons;

#[derive(Component)]
pub struct RunPartButton(Part);

#[derive(Component)]
pub struct RunPartResult(Part);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Part {
    A,
    B,
}
impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::A => write!(f, "A"),
            Part::B => write!(f, "B"),
        }
    }
}

pub fn update(
    solution_state: Res<SolutionDisplayHandler>,
    mut active_day_state: ResMut<ActiveDay>,
    mut interaction_query: Query<
        (
            &Interaction,
            &RunPartButton,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, part_button, mut color, mut border_color, children) in &mut interaction_query
    {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = BACKGROUND_COLOR_ACTIVE.into();
                border_color.0 = BORDER_COLOR_ACTIVE;
                text.sections[0].style.color = TEXT_COLOR_ACTIVE;
            }
            Interaction::Hovered => {
                *color = BACKGROUND_COLOR.into();
                border_color.0 = BORDER_COLOR_ACTIVE;
                text.sections[0].style.color = TEXT_COLOR;
            }
            Interaction::None => {
                *color = BACKGROUND_COLOR.into();
                border_color.0 = BORDER_COLOR;
                text.sections[0].style.color = TEXT_COLOR;
            }
        }
        if *interaction == Interaction::Pressed {
            if let Some(day) = active_day_state.day {
                let task_pool = AsyncComputeTaskPool::get();
                let solutions = solution_state.clone();
                match part_button.0 {
                    Part::A => {
                        if active_day_state.part1_task.is_none() {
                            active_day_state.part1_task = Some(
                                task_pool.spawn(async move { solutions.run_part_a(day).into() }),
                            );
                        }
                    }
                    Part::B => {
                        if active_day_state.part2_task.is_none() {
                            active_day_state.part2_task = Some(
                                task_pool.spawn(async move { solutions.run_part_b(day).into() }),
                            );
                        }
                    }
                }
            }
        }
    }
}

pub fn process_answer_task_result(
    mut active_day_state: ResMut<ActiveDay>,
    mut text_result_query: Query<(&mut Text, &RunPartResult), With<RunPartResult>>,
) {
    if let Some(task) = active_day_state.part1_task.as_mut() {
        let status = block_on(future::poll_once(task));
        if let Some(result) = status {
            active_day_state.part1_task = None;
            for (mut text, run_part_result) in &mut text_result_query {
                if run_part_result.0 == Part::A {
                    text.sections[0].value = format!(
                        "{:?} ({:.2?})",
                        display_answer(&result.answer),
                        result.duration
                    );
                }
            }
            active_day_state.part1 = Some(result);
        }
    }
    if let Some(task) = active_day_state.part2_task.as_mut() {
        let status = block_on(future::poll_once(task));
        if let Some(result) = status {
            active_day_state.part2_task = None;
            for (mut text, run_part_result) in &mut text_result_query {
                if run_part_result.0 == Part::B {
                    text.sections[0].value = format!(
                        "{:?} ({:.2?})",
                        display_answer(&result.answer),
                        result.duration
                    );
                }
            }
            active_day_state.part2 = Some(result);
        }
    }
}

fn display_answer(answer: &Answer) -> String {
    match answer.get_result() {
        Ok(result) => result.to_string(),
        Err(e) => e.to_string(),
    }
}

pub fn event_listener(
    mut state: ResMut<ActiveDay>,
    mut commands: Commands,
    mut select_day_events: EventReader<SelectDayEvent>,
    button_container_query: Query<Entity, With<RunPartButtons>>,
) {
    for event in select_day_events.read() {
        if let Some(day) = event.day {
            spawn_run_buttons(&mut commands);
            state.set_day(day);
        } else {
            for entity in &button_container_query {
                commands.entity(entity).despawn_recursive();
            }
            state.clear();
        }
    }
}

fn spawn_run_buttons(commands: &mut Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    left: Val::Px(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            RunPartButtons {},
        ))
        .with_children(|parent| {
            for part in [Part::A, Part::B] {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(20.0),
                            margin: UiRect::all(Val::Px(1.0)),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(15.0),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(100.0),
                                        ..get_default_button_style()
                                    },
                                    ..get_default_button_bundle()
                                },
                                RunPartButton(part),
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    format!("Run {}", part),
                                    get_default_text_style(),
                                ));
                            });
                        parent.spawn((
                            TextBundle::from_section("".to_string(), get_default_text_style()),
                            RunPartResult(part),
                        ));
                    });
            }
        });
}
