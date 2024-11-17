use bevy::{
    math::{vec2, vec3},
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::Player;

#[derive(Component)]
pub struct ReLoadingText;
#[derive(Component)]
pub struct AmmoText;

#[derive(Component)]
pub struct AmmoIcon;

#[derive(Component)]
pub struct Heart;

#[derive(Component)]
pub struct GameOverButton;

#[derive(Component)]
pub struct GameOverText;

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
    let mut text_bundle = TextBundle::from(TextSection::new(
        "Reloading",
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
    });
    text_bundle.visibility = Visibility::Hidden;
    commands.spawn((text_bundle, ReLoadingText));
}

pub fn update_reloading_text(
    player_query: Query<&Player, With<Player>>,
    mut reloading_text_query: Query<&mut Visibility, With<ReLoadingText>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut reloading_text_visibility) = reloading_text_query.get_single_mut() {
            if player.ammunition == 0 {
                *reloading_text_visibility = Visibility::Visible;
            } else {
                *reloading_text_visibility = Visibility::Hidden;
            }
        }
    }
}

pub fn spawn_game_over_text(commands: &mut Commands) {
    let mut text_bundle = TextBundle::from(TextSection::new(
        "Game Over",
        TextStyle {
            font_size: 40.0,
            ..default()
        },
    ))
    .with_style(Style {
        position_type: PositionType::Absolute,
        align_self: AlignSelf::Center,
        top: Val::Percent(40.0),
        margin: UiRect {
            left: Val::Auto,
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Auto,
        },
        ..default()
    });
    text_bundle.visibility = Visibility::Hidden;
    commands.spawn((text_bundle, GameOverText));
}

pub fn spawn_restart_button(commands: &mut Commands) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                position_type: PositionType::Absolute,
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            visibility: Visibility::Hidden, // Set visibility directly here
            ..default()
        })
        .insert(GameOverButton) // Insert the `GameOverButton` component
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Restart",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..default()
            });
        });
}

pub fn update_game_over_button(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut visibility_params_query: ParamSet<(
        Query<&mut Visibility, With<GameOverText>>,
        Query<&mut Visibility, With<GameOverButton>>,
    )>,
    win_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut text_query: Query<&mut Text>,
    assets: Res<AssetServer>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if let Ok(mut game_over_text_visibility) =
                    visibility_params_query.p0().get_single_mut()
                {
                    *game_over_text_visibility = Visibility::Hidden;
                }
                if let Ok(mut game_over_button_visibility) =
                    visibility_params_query.p1().get_single_mut()
                {
                    *game_over_button_visibility = Visibility::Hidden;
                }
                let texture: Handle<Image> = assets.load("../assets/Spritesheet/heart.png");
                if let Ok(win) = win_query.get_single() {
                    spawn_hearts(&mut commands, texture, win);
                    if let Ok(mut player) = player_query.get_single_mut() {
                        player.translation.x = 0.;
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::WHITE);
                text.sections[0].style.color = Color::srgb(0., 0., 0.)
            }
            _ => {
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
                text.sections[0].style.color = Color::WHITE
            }
        }
    }
}
