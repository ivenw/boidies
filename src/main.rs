use bevy::{prelude::*, time::FixedTimestep};
// use bevy_mouse_tracking_plugin::{MousePos, MousePosPlugin, MousePosWorld};

mod boids;

// Defines the update frequency of the simulation in Hz
const GAME_FREQUENCY: f32 = 60.0;
pub const TIME_STEP: f32 = 1.0 / GAME_FREQUENCY;
const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(MousePosPlugin::SingleCamera)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup_camera)
                .with_system(boids::setup_boids),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                // .with_system(move_boid_towards_perceived_scenter_of_mass)
                // .with_system(keep_distance_from_boids)
                .with_system(boids::boid_system),
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
    mut window: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = window.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Q) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}
