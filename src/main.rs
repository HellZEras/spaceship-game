use std::time::Duration;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    sprite::TextureAtlas,
    window::{PrimaryWindow, WindowResized},
};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (execute_animations, update_stars, update_background),
        )
        .run();
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            frame_timer: Timer::new(
                Duration::from_secs_f32(1.0 / (fps as f32)),
                TimerMode::Repeating,
            ),
        }
    }
}

fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut config, mut atlas) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if atlas.index == config.last_sprite_index {
                atlas.index = config.first_sprite_index;
            } else {
                atlas.index += 1;
            }
        }
    }
}

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct Background;
#[derive(Component)]
pub struct Player;

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());

    if let Ok(win) = query.get_single() {
        spawn_background(&mut commands, win);
        spawn_stars(&mut commands, win, &assets);
        spawn_spaceship(&mut commands, &assets);
    }
}

fn update_stars(
    mut query: Query<&mut Transform, With<Star>>,
    events: EventReader<WindowResized>,
    win_query: Query<&Window, With<PrimaryWindow>>,
) {
    if !events.is_empty() {
        if let Ok(win) = win_query.get_single() {
            let mut rng = rand::thread_rng();
            for mut star in query.iter_mut() {
                let x = rng.gen_range((-1. * (win.width() / 2.))..win.width() / 2.);
                let y = rng.gen_range((-1. * (win.height() / 2.))..win.height() / 2.);
                star.translation.x = x;
                star.translation.y = y;
            }
        }
    }
}

fn update_background(
    mut query: Query<&mut Sprite, With<Background>>,
    events: EventReader<WindowResized>,
    win_query: Query<&Window, With<PrimaryWindow>>,
) {
    if !events.is_empty() {
        if let Ok(win) = win_query.get_single() {
            if let Ok(mut bg) = query.get_single_mut() {
                bg.custom_size = Some(vec2(win.resolution.width(), win.resolution.height()));
            }
        }
    }
}

fn spawn_background(commands: &mut Commands, win: &Window) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0.0, 0., -2.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::srgb(0., 0., 0.),
                custom_size: Some(vec2(win.resolution.width(), win.resolution.height())),
                ..default()
            },
            ..default()
        },
        Background,
    ));
}

fn spawn_stars(commands: &mut Commands, win: &Window, assets: &Res<AssetServer>) {
    let texture = assets.load("/home/numerouscuts/Coding/game/assets/Spritesheet/star.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 3, 1, None, None);
    let texture_atlas_layout = assets.add(layout);
    let mut rng = rand::thread_rng();
    for _ in 0..8000 {
        let animation_config = AnimationConfig::new(0, 2, 5);

        let x = rng.gen_range((-1. * (win.width() / 2.))..win.width() / 2.);
        let y = rng.gen_range((-1. * (win.height() / 2.))..win.height() / 2.);
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: vec3(x, y, -1.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(vec2(1.0, 1.0)),
                    ..default()
                },
                texture: texture.clone(),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index,
            },
            Star,
            animation_config,
        ));
    }
}

fn spawn_spaceship(commands: &mut Commands, assets: &Res<AssetServer>) {
    let texture = assets.load("/home/numerouscuts/Coding/game/assets/Spritesheet/spaceship.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0., 0., 0.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(vec2(80.0, 80.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Player,
    ));
}
