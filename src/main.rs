use bevy::{
    math::{vec2, vec3},
    prelude::*,
    sprite::TextureAtlas,
    window::{PrimaryWindow, WindowResized},
};
use rand::Rng;
use std::time::{Duration, Instant};

const PLAYER_MOVEMENT_SPEED: f32 = 600.0;
const STAR_FALLING_SPEED: f32 = 50.0;
const SPACESHIP_SIZE: f32 = 80.0;
const SHOT_SPEED: f32 = 400.0;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // .set(WindowPlugin {
                                                                //     primary_window: Some(Window {
                                                                //         resizable: true,
                                                                //         mode: bevy::window::WindowMode::BorderlessFullscreen,
                                                                //         ..default()
                                                                //     }),
                                                                //     ..default()
                                                                // }),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                execute_animations,
                update_stars,
                update_background,
                falling_stars,
                player_inputs,
                fire_logic,
            ),
        )
        .run();
}

#[derive(Component, Clone)]
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
pub struct Fire;

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct Background;
#[derive(Component)]
pub struct Player {
    click_instant: Instant,
    ammunition: usize,
}

#[derive(Component)]
pub struct AmmoIcon;

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());
    let star_texture: Handle<Image> = assets.load("../assets/Spritesheet/star.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 3, 1, None, None);
    let texture_atlas_layout = assets.add(layout);
    let animation_config = AnimationConfig::new(0, 2, 5);
    let space_ship_texture = assets.load("../assets/Spritesheet/spaceship1.png");
    let ammo_icon_texture = assets.load("../assets/Spritesheet/ammo_icon.png");
    if let Ok(win) = query.get_single() {
        spawn_background(&mut commands, win);
        spawn_ammo_icon(&mut commands, ammo_icon_texture, win);
        spawn_stars(
            &mut commands,
            win,
            &star_texture,
            &texture_atlas_layout,
            &animation_config,
        );
        spawn_spaceship(&mut commands, space_ship_texture, win);
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

fn spawn_stars(
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

fn spawn_spaceship(commands: &mut Commands, texture: Handle<Image>, win: &Window) {
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
            ammunition: 60,
        },
    ));
}

fn falling_stars(
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

fn spawn_ammo_icon(commands: &mut Commands, texture: Handle<Image>, win: &Window) {
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
                custom_size: Some(vec2(80.0, 80.0)),
                ..default()
            },
            ..default()
        },
        AmmoIcon,
    ));
}

fn player_inputs(
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

fn fire_logic(
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
