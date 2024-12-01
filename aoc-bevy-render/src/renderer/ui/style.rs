use bevy::prelude::*;

pub const BORDER_COLOR: Color = Color::srgb(148. / 255., 185. / 255., 235. / 255.);
pub const BORDER_COLOR_ACTIVE: Color = Color::srgb(25. / 255., 118. / 255., 210. / 255.);
pub const BACKGROUND_COLOR: Color = Color::srgb(1., 1., 1.);
pub const BACKGROUND_COLOR_NO_RENDER: Color = Color::srgb(0.7, 0.7, 0.7);
pub const BACKGROUND_COLOR_ACTIVE: Color = Color::srgb(25. / 255., 118. / 255., 210. / 255.);
pub const TEXT_COLOR: Color = Color::srgb(25. / 255., 118. / 255., 210. / 255.);
pub const TEXT_COLOR_ACTIVE: Color = Color::srgb(1., 1., 1.);

pub fn get_default_button_style() -> Style {
    Style {
        width: Val::Px(100.0),
        height: Val::Px(20.0),
        margin: UiRect::all(Val::Px(1.0)),
        border: UiRect::all(Val::Px(1.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
pub fn get_default_button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: get_default_button_style(),
        border_color: BORDER_COLOR.into(),
        border_radius: BorderRadius::all(Val::Px(4.0)),
        background_color: BACKGROUND_COLOR.into(),
        ..default()
    }
}
pub fn get_default_text_style() -> TextStyle {
    TextStyle {
        font_size: 14.0,
        color: TEXT_COLOR,
        ..default()
    }
}
