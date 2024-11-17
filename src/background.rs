use std::time::Duration;
const STAR_FALLING_SPEED: f32 = 50.0;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use rand::Rng;

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct Background;

#[derive(Component, Clone)]
pub struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
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

pub fn execute_animations(
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

pub fn update_stars(
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

pub fn update_background(
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

pub fn spawn_background(commands: &mut Commands, win: &Window) {
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

pub fn spawn_stars(
    commands: &mut Commands,
    win: &Window,
    texture: &Handle<Image>,
    texture_atlas_layout: &Handle<TextureAtlasLayout>,
    animation_config: &AnimationConfig,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..500 {
        let x = rng.gen_range((-1. * (win.width() / 2.))..win.width() / 2.);
        let y = rng.gen_range((-1. * (win.height() / 2.))..win.height() / 2.);
        let animation_config = animation_config.clone();
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: vec3(x, y, -1.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(vec2(3.0, 3.0)),
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

pub fn falling_stars(
    mut query: Query<&mut Transform, With<Star>>,
    win_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    if let Ok(win) = win_query.get_single() {
        for mut transform in query.iter_mut() {
            transform.translation.y -= time.delta_seconds() * STAR_FALLING_SPEED;

            if transform.translation.y < -win.height() / 2. {
                transform.translation.y = win.height() / 2.;
            }
        }
    }
}
