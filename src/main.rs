use asteroids::replace_asteroids;
use asteroids::spawn_asteroids;
use asteroids::update_asteroids;
use bevy::{prelude::*, window::PrimaryWindow};

mod asteroids;
mod background;
mod hud;
mod player;
use crate::background::*;
use crate::hud::*;
use crate::player::*;

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
                update_ammo_icon_pos,
                update_ammo_text,
                update_hearts_pos,
                update_reloading_text,
                update_ammunition,
                update_asteroids,
                replace_asteroids,
            ),
        )
        .run();
}

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
    let heart_texture = assets.load("../assets/Spritesheet/heart.png");
    let asteroid_texture = assets.load("../assets/Spritesheet/asteroid.png");
    if let Ok(win) = query.get_single() {
        spawn_background(&mut commands, win);
        spawn_ammo_text(&mut commands);
        spawn_ammo_icon(&mut commands, ammo_icon_texture, win);
        spawn_stars(
            &mut commands,
            win,
            &star_texture,
            &texture_atlas_layout,
            &animation_config,
        );
        spawn_asteroids(&mut commands, asteroid_texture, win);
        spawn_reloading_text(&mut commands);
        spawn_hearts(&mut commands, heart_texture, win);
        spawn_spaceship(&mut commands, space_ship_texture, win);
    }
}

#[derive(Component)]
pub struct Collider;
