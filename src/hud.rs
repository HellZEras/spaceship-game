use bevy::{
    math::{vec2, vec3},
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::Player;

#[derive(Component)]
pub struct ReLoadingText {
    done: bool,
}
#[derive(Component)]
pub struct AmmoText;

#[derive(Component)]
pub struct AmmoIcon;

#[derive(Component)]
pub struct Heart;

pub fn spawn_ammo_icon(commands: &mut Commands, texture: Handle<Image>, win: &Window) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(
                    win.resolution.width() / 2. - 25.0,
                    -(win.resolution.height() / 2.) - 25.0,
                    0.0,
                ),
                ..default()
            },
            texture,
            sprite: Sprite {
                custom_size: Some(vec2(60.0, 60.0)),
                ..default()
            },
            ..default()
        },
        AmmoIcon,
    ));
}

pub fn update_ammo_icon_pos(
    win_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<AmmoIcon>>,
    events: EventReader<WindowResized>,
) {
    if let Ok(win) = win_query.get_single() {
        if let Ok(mut transform) = query.get_single_mut() {
            if !events.is_empty() {
                transform.translation.x = win.resolution.width() / 2. - 90.0;
                transform.translation.y = -win.resolution.height() / 2. + 30.0;
            }
        }
    }
}

pub fn spawn_ammo_text(commands: &mut Commands) {
    commands.spawn((
        TextBundle::from(TextSection::new(
            "60",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 40.0,
                ..default()
            },
        ))
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        AmmoText,
    ));
}

pub fn update_ammo_text(
    player_query: Query<&Player, With<Player>>,
    mut ammo_text_query: Query<&mut Text, With<AmmoText>>,
) {
    if let Ok(mut ammo_text) = ammo_text_query.get_single_mut() {
        if let Ok(player) = player_query.get_single() {
            let ammo = player.ammunition;
            ammo_text.sections[0].value = {
                if ammo < 10 {
                    format!("0{}", player.ammunition)
                } else {
                    format!("{}", player.ammunition)
                }
            }
        }
    }
}

pub fn spawn_hearts(commands: &mut Commands, texture: Handle<Image>, win: &Window) {
    let height = win.resolution.height();
    let width = win.resolution.width();
    let top_left_x = -width / 2.0 + 50.0;
    let top_left_y = height / 2.0;
    for i in 0..3 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: vec3(top_left_x + (i as f32 * 60.0), top_left_y, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(vec2(50.0, 50.0)),
                    ..default()
                },
                texture: texture.clone(),
                ..default()
            },
            Heart,
        ));
    }
}
pub fn update_hearts_pos(
    win_query: Query<&Window, With<PrimaryWindow>>,
    mut hearts_query: Query<&mut Transform, With<Heart>>,
) {
    if let Ok(win) = win_query.get_single() {
        let width = win.resolution.width();
        let height = win.resolution.height();
        let top_left_x = -width / 2.0 + 30.0;
        let top_left_y = height / 2.0 - 40.;
        for (idx, mut heart) in hearts_query.iter_mut().enumerate() {
            heart.translation.x = top_left_x + (idx as f32 * 60.0);
            heart.translation.y = top_left_y;
        }
    }
}

pub fn spawn_reloading_text(commands: &mut Commands) {
    commands.spawn((
        TextBundle::from(TextSection::new(
            "",
            TextStyle {
                font_size: 40.0,
                ..default()
            },
        ))
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(130.0),
            ..default()
        }),
        ReLoadingText { done: false },
    ));
}

pub fn update_reloading_text(
    player_query: Query<&Player, With<Player>>,
    mut reloading_text_query: Query<(&mut Text, &mut ReLoadingText), With<ReLoadingText>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut reloading_text) = reloading_text_query.get_single_mut() {
            if player.ammunition == 0 {
                if !reloading_text.1.done {
                    reloading_text.0.sections[0].value.push_str("Reloading...");
                    reloading_text.1.done = true;
                }
            } else {
                reloading_text.0.sections[0].value.clear();
                reloading_text.1.done = false;
            }
        }
    }
}
