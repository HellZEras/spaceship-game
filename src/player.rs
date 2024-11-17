use std::time::{Duration, Instant};

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    window::PrimaryWindow,
};

use crate::Collider;

const PLAYER_MOVEMENT_SPEED: f32 = 600.0;
const SPACESHIP_SIZE: f32 = 80.0;
const SHOT_SPEED: f32 = 400.0;

#[derive(Component)]
pub struct Fire;

#[derive(Component)]
pub struct Player {
    pub click_instant: Instant,
    pub reload_instant: Option<Instant>,
    pub ammunition: usize,
}

pub fn spawn_spaceship(commands: &mut Commands, texture: Handle<Image>, win: &Window) {
    let spawn_point = -1.0 * (win.resolution.height() / 2.1);
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0., spawn_point, 0.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(vec2(SPACESHIP_SIZE, SPACESHIP_SIZE)),
                ..default()
            },
            texture,
            ..default()
        },
        Player {
            click_instant: Instant::now(),
            reload_instant: None,
            ammunition: 60,
        },
        Collider,
    ));
}

pub fn player_inputs(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    win_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(win) = win_query.get_single() {
        if let Ok(mut player) = player_query.get_single_mut() {
            let texture = assets.load("../assets/Spritesheet/fire.png");
            let dt = time.delta_seconds();
            let half_width = win.resolution.width() / 2.0;

            if input.pressed(KeyCode::KeyD) {
                player.0.translation.x += dt * PLAYER_MOVEMENT_SPEED;
            } else if input.pressed(KeyCode::KeyA) {
                player.0.translation.x -= dt * PLAYER_MOVEMENT_SPEED;
            }
            if input.pressed(KeyCode::KeyJ)
                && player.1.click_instant.elapsed() > Duration::from_millis(100)
                && player.1.ammunition > 0
            {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: vec3(player.0.translation.x, player.0.translation.y, -1.0),
                            ..default()
                        },
                        sprite: Sprite {
                            custom_size: Some(vec2(20., 20.)),
                            ..default()
                        },
                        texture,
                        ..default()
                    },
                    Fire,
                    Collider,
                ));
                player.1.click_instant = Instant::now();
                player.1.ammunition -= 1;
            }
            player.0.translation.x = player.0.translation.x.clamp(
                -half_width + SPACESHIP_SIZE / 2.,
                half_width - SPACESHIP_SIZE / 2.,
            );
        }
    }
}

pub fn fire_logic(
    mut fire_query: Query<(&mut Transform, Entity), With<Fire>>,
    mut commands: Commands,
    win_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    if let Ok(win) = win_query.get_single() {
        for (mut transform, shot) in fire_query.iter_mut() {
            if transform.translation.y > win.resolution.height() / 2. {
                commands.entity(shot).despawn()
            } else {
                transform.translation.y += time.delta_seconds() * SHOT_SPEED
            }
        }
    }
}

pub fn update_ammunition(mut player_query: Query<&mut Player, With<Player>>) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if player.ammunition == 0 && player.reload_instant.is_none() {
            player.reload_instant = Some(Instant::now())
        }
        if let Some(inst) = player.reload_instant {
            if inst.elapsed() > Duration::from_secs(2) {
                player.reload_instant = None;
                player.ammunition = 60;
            }
        }
    }
}
