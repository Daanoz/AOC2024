#![allow(clippy::type_complexity)]

use bevy::prelude::*;

use super::{style::*, ActiveDay, SelectDayEvent, SolutionDisplayHandler};

#[derive(Component, Debug)]
pub struct DayButton(pub u32, pub bool);
impl DayButton {
    fn get_default_background_color(&self) -> BackgroundColor {
        if self.1 {
            BACKGROUND_COLOR
        } else {
            BACKGROUND_COLOR_NO_RENDER
        }
        .into()
    }
}

const DAY_BUTTON_WIDTH: f32 = 70.0;

pub fn setup(mut commands: Commands, state: Res<SolutionDisplayHandler>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(DAY_BUTTON_WIDTH),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for (day, can_render) in state.get_days() {
                let day_button = DayButton(day, can_render);
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(DAY_BUTTON_WIDTH),
                                ..get_default_button_style()
                            },
                            background_color: day_button.get_default_background_color(),
                            ..get_default_button_bundle()
                        },
                        day_button,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("Day {}", day),
                            get_default_text_style(),
                        ));
                    });
            }
        });
}

pub fn update(
    state: Res<ActiveDay>,
    mut events: EventWriter<SelectDayEvent>,
    mut interaction_query: Query<
        (
            &Interaction,
            &DayButton,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, day_button, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        let is_active_day = Some(day_button.0) == state.day;
        match (*interaction, is_active_day) {
            (Interaction::Pressed, _) | (_, true) => {
                *color = BACKGROUND_COLOR_ACTIVE.into();
                border_color.0 = BORDER_COLOR_ACTIVE;
                text.sections[0].style.color = TEXT_COLOR_ACTIVE;
            }
            (Interaction::Hovered, false) => {
                *color = day_button.get_default_background_color();
                border_color.0 = BORDER_COLOR_ACTIVE;
                text.sections[0].style.color = TEXT_COLOR;
            }
            (Interaction::None, false) => {
                *color = day_button.get_default_background_color();
                border_color.0 = BORDER_COLOR;
                text.sections[0].style.color = TEXT_COLOR;
            }
        }
        if *interaction == Interaction::Pressed {
            if is_active_day {
                events.send(SelectDayEvent { day: None });
            } else {
                events.send(SelectDayEvent {
                    day: Some(day_button.0),
                });
            }
        }
    }
}

pub fn event_listener(
    mut events: EventReader<SelectDayEvent>,
    mut all_button_query: Query<
        (
            &DayButton,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        With<Button>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for event in events.read() {
        if let Some(day) = event.day {
            for (day_button, mut color, mut border_color, children) in &mut all_button_query {
                if day_button.0 != day {
                    let mut text = text_query.get_mut(children[0]).unwrap();
                    *color = day_button.get_default_background_color();
                    border_color.0 = BORDER_COLOR;
                    text.sections[0].style.color = TEXT_COLOR;
                }
            }
        }
    }
}
