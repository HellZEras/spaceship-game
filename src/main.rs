use asteroids::replace_asteroids;
use asteroids::spawn_asteroids;
use asteroids::update_asteroids;
use asteroids::Asteroid;
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
                falling_stars.run_if(alive),
                player_inputs.run_if(alive),
                fire_logic.run_if(alive),
                update_ammo_icon_pos,
                update_ammo_text,
                update_hearts_pos,
                update_reloading_text.run_if(alive),
                update_ammunition,
                update_asteroids.run_if(alive),
                replace_asteroids.run_if(alive),
                detect_player_collision.run_if(alive),
                loop_logic,
                update_game_over_button,
                detect_bullet_collision,
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
        spawn_game_over_text(&mut commands);
        spawn_score_text(&mut commands);
        spawn_restart_button(&mut commands);
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

fn alive(hearts_query: Query<&Heart, With<Heart>>) -> bool {
    let hearts = hearts_query.iter().count();
    hearts > 0
}

fn loop_logic(
    mut commands: Commands,
    mut visibility_queries: ParamSet<(
        Query<&mut Visibility, With<Player>>,
        Query<&mut Visibility, With<AmmoText>>,
        Query<&mut Visibility, With<AmmoIcon>>,
        Query<&mut Visibility, With<GameOverText>>,
        Query<&mut Visibility, With<GameOverButton>>,
        Query<(&mut Visibility, &mut Text), With<ScoreText>>,
    )>,
    bullets_query: Query<Entity, With<Fire>>,
    hearts_query: Query<&Heart, With<Heart>>,
    mut asteroids_query: Query<&mut Transform, With<Asteroid>>,
    win_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(win) = win_query.get_single() {
        if let Ok(mut player_visibility) = visibility_queries.p0().get_single_mut() {
            if alive(hearts_query) {
                *player_visibility = Visibility::Visible;
                if let Ok(mut ammo_text_visibility) = visibility_queries.p1().get_single_mut() {
                    *ammo_text_visibility = Visibility::Visible;
                }
                if let Ok(mut ammo_icon_visibility) = visibility_queries.p2().get_single_mut() {
                    *ammo_icon_visibility = Visibility::Visible;
                }
                if let Ok(mut game_over_text_visibility) = visibility_queries.p3().get_single_mut()
                {
                    *game_over_text_visibility = Visibility::Hidden;
                }
                if let Ok(mut game_over_button_visibility) =
                    visibility_queries.p4().get_single_mut()
                {
                    *game_over_button_visibility = Visibility::Hidden;
                }
                if let Ok((mut score_visibility, _)) = visibility_queries.p5().get_single_mut() {
                    *score_visibility = Visibility::Visible;
                }
            } else {
                for bullet in bullets_query.iter() {
                    commands.entity(bullet).despawn();
                }
                *player_visibility = Visibility::Hidden;
                if let Ok(mut ammo_text_visibility) = visibility_queries.p1().get_single_mut() {
                    *ammo_text_visibility = Visibility::Hidden;
                }
                if let Ok(mut ammo_icon_visibility) = visibility_queries.p2().get_single_mut() {
                    *ammo_icon_visibility = Visibility::Hidden;
                }
                if let Ok(mut game_over_text_visibility) = visibility_queries.p3().get_single_mut()
                {
                    *game_over_text_visibility = Visibility::Visible;
                }
                if let Ok(mut game_over_button_visibility) =
                    visibility_queries.p4().get_single_mut()
                {
                    *game_over_button_visibility = Visibility::Visible;
                }
                if let Ok((mut score_visibility, mut text)) =
                    visibility_queries.p5().get_single_mut()
                {
                    *score_visibility = Visibility::Hidden;
                    text.sections[0].value = String::from("0");
                }
                for mut asteroid in asteroids_query.iter_mut() {
                    asteroid.translation.y = win.resolution.height() / 2. + 25.;
                }
            }
        }
    }
}
