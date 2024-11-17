use std::time::{Duration, Instant};

use bevy::{
    audio::Volume,
    math::{vec2, vec3},
    prelude::*,
    window::PrimaryWindow,
};
use rand::Rng;

use crate::{asteroids::Asteroid, Heart, ScoreText};

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
    let spawn_point = -1.0 * (win.resolution.height() / 2.) + 80.;
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
            let laser = assets.load("../assets/Laser.ogg");
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
                commands.spawn(AudioBundle {
                    source: laser,
                    settings: PlaybackSettings {
                        volume: Volume::new(0.05),
                        ..default()
                    },
                });
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
            if transform.translation.y > win.resolution.height() / 2. - 10. {
                commands.entity(shot).despawn()
            } else {
                transform.translation.y += time.delta_seconds() * SHOT_SPEED
            }
        }
    }
}

pub fn update_ammunition(
    mut commands: Commands,
    mut player_query: Query<&mut Player, With<Player>>,
    assets: Res<AssetServer>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if player.ammunition == 0 && player.reload_instant.is_none() {
            player.reload_instant = Some(Instant::now());
            let reload = assets.load("../assets/reload.ogg");
            commands.spawn(AudioBundle {
                source: reload,
                settings: PlaybackSettings {
                    speed: 0.3,
                    ..default()
                },
            });
        }
        if let Some(inst) = player.reload_instant {
            if inst.elapsed() > Duration::from_secs(2) {
                player.reload_instant = None;
                player.ammunition = 60;
            }
        }
    }
}

pub fn detect_player_collision(
    mut commands: Commands,
    mut query: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Asteroid>>,
    )>,
    mut hearts_query: Query<Entity, With<Heart>>,
    win_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(win) = win_query.get_single() {
        if let Ok(player) = query.p0().get_single() {
            let player_coords = player.translation.truncate();
            let player_radius = SPACESHIP_SIZE / 2.;

            for mut asteroid in query.p1().iter_mut() {
                let asteroid_coords = asteroid.translation.truncate();
                let asteroid_radius = asteroid.scale.x / 2.;
                let distance = player_coords.distance(asteroid_coords);

                if distance < player_radius + asteroid_radius {
                    asteroid.translation.y = win.height() / 2.;

                    let mut hearts = hearts_query.iter_mut().collect::<Vec<_>>();
                    if let Some(heart) = hearts.last_mut() {
                        commands.entity(*heart).despawn();
                    }
                }
            }
        }
    }
}

pub fn detect_bullet_collision(
    mut commands: Commands,
    bullets_query: Query<(&Transform, Entity), (With<Fire>, Without<Asteroid>)>,
    mut asteroids_query: Query<&mut Transform, (With<Asteroid>, Without<Fire>)>,
    mut score_query: Query<&mut Text, With<ScoreText>>,
    win_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    if let Ok(win) = win_query.get_single() {
        let explosion = assets.load("../assets/Explosion.ogg");
        for bullet in bullets_query.iter() {
            let bullet_coords = bullet.0.translation.truncate();
            let bullet_radius = 10.;

            for mut asteroid in asteroids_query.iter_mut() {
                let asteroid_coords = asteroid.translation.truncate();
                let asteroid_radius = 25.;

                let distance = bullet_coords.distance(asteroid_coords);

                if distance < bullet_radius + asteroid_radius {
                    commands.spawn(AudioBundle {
                        source: explosion.clone(),
                        settings: PlaybackSettings {
                            volume: Volume::new(0.1),
                            ..default()
                        },
                    });
                    commands.entity(bullet.1).despawn();
                    asteroid.translation.y = win.height() / 2. + 50.;
                    let mut rng = rand::thread_rng();
                    let x = rng.gen_range(
                        -win.resolution.width() / 2.0 + 50. ..win.resolution.width() / 2. - 50.,
                    );
                    asteroid.translation.x = x;
                    if let Ok(mut score) = score_query.get_single_mut() {
                        let value = score.sections[0].value.parse::<i64>().unwrap();
                        score.sections[0].value = format!("{}", value + 100);
                    }
                }
            }
        }
    }
}
