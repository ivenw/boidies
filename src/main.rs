use bevy::{prelude::*, time::FixedTimestep};
use bevy_mouse_tracking_plugin::{MousePos, MousePosPlugin, MousePosWorld};
use rand::Rng;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

const NUM_BOIDS: u32 = 100;
const BOID_SIZE: Vec3 = Vec3::new(10., 10., 0.);
const BOID_COLOR: Color = Color::rgb(1., 1., 1.);
const BOID_SPEED: f32 = 500.0;
const BOID_ACCELERATION: f32 = 10.0;
const MIN_BOID_DIST: f32 = 30.0;
const BOID_SENSING_RADIUS: f32 = 100.0;

#[derive(Component, PartialEq)]
struct Boid;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MousePosPlugin::SingleCamera)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup_camera)
                .with_system(setup_boids),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(boid_system),
        )
        // .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .add_system(cursor_grab_system)
        .run();
}

// Add the game's entities to our world
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_boids(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_BOIDS {
        commands
            .spawn()
            .insert(Boid)
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    scale: BOID_SIZE,
                    translation: Vec3::new(
                        rng.gen_range(-700.0..700.0),
                        rng.gen_range(-300.0..300.0),
                        1.,
                    ),
                    ..default()
                },
                sprite: Sprite {
                    color: BOID_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(Velocity(
                Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize()
                    * BOID_SPEED,
            ));
    }
}

fn boid_system(mut query: Query<(Entity, &mut Transform, &mut Velocity), With<Boid>>) {}

// fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity)>) {
//     for (mut transform, mut velocity) in &mut query {
//         if velocity.length() > BOID_SPEED {
//             velocity.0 = velocity.0.normalize() * BOID_SPEED;
//         }
//         transform.translation.x += velocity.x * TIME_STEP;
//         transform.translation.y += velocity.y * TIME_STEP;
//     }
// }

// fn move_boid_towards_perceived_scenter_of_mass(
//     mut query: Query<(Entity, &Transform, &mut Velocity), With<Boid>>,
// ) {
//     let boids: Vec<(Entity, Vec3)> = query.iter().map(|x| (x.0, x.1.translation)).collect();
//     for (entity, transform, mut velocity) in &mut query {
//         let mut arr = Vec3::new(0.0, 0.0, 0.0);
//         let mut len: u32 = 0;
//         for (e, translation) in boids.to_owned() {
//             if e != entity && transform.translation.distance(translation) < BOID_SENSING_RADIUS {
//                 arr += translation;
//                 len += 1;
//             }
//         }
//         let perceived_center = arr / len as f32;
//         let direction = Vec2::new(
//             perceived_center.x - transform.translation.x,
//             perceived_center.y - transform.translation.y,
//         )
//         .normalize();
//         velocity.x += direction.x * BOID_ACCELERATION * 0.5;
//         velocity.y += direction.y * BOID_ACCELERATION * 0.5;
//     }
// }

// fn keep_distance_from_boids(mut query: Query<(Entity, &Transform, &mut Velocity), With<Boid>>) {
//     let boids: Vec<(Entity, Vec3)> = query.iter().map(|x| (x.0, x.1.translation)).collect();
//     for (entity, transform, mut velocity) in &mut query {
//         for (e, translation) in boids.to_owned() {
//             if e != entity {
//                 if transform.translation.distance(translation) < MIN_BOID_DIST {
//                     let direction =
//                         (1.0 * (transform.translation - translation)).normalize() * BOID_SPEED;
//                     velocity.x += direction.x * BOID_ACCELERATION * 0.1;
//                     velocity.y += direction.y * BOID_ACCELERATION * 0.1;
//                 }
//             }
//         }
//     }
// }

// // fn match_other_boids_velocity(
// //     mut query: Query<(Entity, &mut Velocity), With<Boid>>,
// //     query2: Query<(Entity, &Velocity), With<Boid>>,
// // ) {
// //     // let boids: Vec<(Entity, &Velocity)> = query.iter().map(|x| (x.0, x.1)).collect();
// //     for (entity, mut velocity) in &mut query {
// //         let mut arr = Vec2::new(0.0, 0.0);
// //         let mut len: u32 = 0;
// //         for (e, vel) in &query2 {
// //             if e != entity {
// //                 arr.x += vel.x;
// //                 arr.y += vel.y;
// //                 len += 1;
// //             }
// //         }
// //         let perceived_velocity = arr / len as f32;
// //         velocity.x += perceived_velocity.x / 8.0;
// //         velocity.y += perceived_velocity.y / 8.0;
// //     }
// // }

// fn move_boids_to_mouse(
//     mut query: Query<(&Transform, &mut Velocity), With<Boid>>,
//     mouse: Res<MousePosWorld>,
// ) {
//     let mouse_pos = Vec2::new(mouse[0], mouse[1]);
//     // let pos = Vec2::new(0.0, 0.0);
//     for (transform, mut velocity) in &mut query {
//         let boid_pos = Vec2::new(transform.translation.x, transform.translation.y);
//         velocity.0 += (mouse_pos - boid_pos).normalize() * BOID_ACCELERATION;
//     }
// }

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Q) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}
