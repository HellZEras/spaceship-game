use bevy::{
    math::{vec2, vec3},
    prelude::*,
    window::PrimaryWindow,
};
use rand::Rng;
#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct Velocity {
    pub speed: f32,
    pub direction_x: f32,
}

pub fn spawn_asteroids(commands: &mut Commands, texture: Handle<Image>, win: &Window) {
    let mut rng = rand::thread_rng();
    let y = win.height();

    for _ in 0..20 {
        let x = rng.gen_range(-win.width() / 2.0 + 20.0..win.width() / 2.0 - 20.0);
        let falling_speed = rng.gen_range(150.0..400.0);
        let falling_x = rng.gen_range(-40.0..40.0);

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: vec3(x, y, -1.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(vec2(50.0, 50.0)),
                    ..default()
                },
                texture: texture.clone(),
                ..default()
            },
            Asteroid,
            Velocity {
                speed: falling_speed,
                direction_x: falling_x,
            },
        ));
    }
}

pub fn update_asteroids(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Asteroid>>,
) {
    let dt = time.delta_seconds();
    for (mut transform, velocity) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(1.0 * dt);
        transform.translation.y -= velocity.speed * dt;
        transform.translation.x += velocity.direction_x * dt;
    }
}
pub fn replace_asteroids(
    mut query: Query<(&mut Transform, &Velocity), With<Asteroid>>,
    win_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(win) = win_query.get_single() {
        for (mut transform, _) in query.iter_mut() {
            if transform.translation.y < -win.height() / 2. {
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(-win.width() + 5. / 2.0..win.width() / 2. - 5.);
                transform.translation.y = win.height() / 2.;
                transform.translation.x = x;
            }
        }
    }
}
