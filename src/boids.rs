use bevy::prelude::*;
use rand::Rng;
// use std::f32::consts::PI;

const NUM_BOIDS: u32 = 10;
const BOID_SIZE: Vec3 = Vec3::new(10., 10., 1.);
const BOID_COLOR: Color = Color::rgb(1., 1., 1.);
// max boid speed in pixels.sec
const MAX_BOID_SPEED: f32 = 10.0;
// const BOID_ACCELERATION: f32 = 10.0;
// const MIN_BOID_DIST: f32 = 30.0;
// const BOID_SENSING_RADIUS: f32 = 100.0;

#[derive(Component)]
pub struct Boid {
    position: Vec2,
    velocity: Vec2,
}

pub fn setup_boids(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_BOIDS {
        let position = Vec2::new(rng.gen_range(-700.0..700.0), rng.gen_range(-300.0..300.0));
        let velocity = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize()
            * MAX_BOID_SPEED;
        commands
            .spawn()
            .insert(Boid { position, velocity })
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    scale: BOID_SIZE,
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: BOID_COLOR,
                    ..default()
                },
                ..default()
            });
        // start boids in random direction at max boid speed
    }
}

pub fn boid_system(mut query: Query<(Entity, &mut Boid, &mut Transform)>, windows: Res<Windows>) {
    // let boids: Vec<(Entity, Boid)> = query.iter().map(|x| (x.0, x.1.translation)).collect();
    let window = windows.get_primary().unwrap();
    for (boid_id, mut boid, mut transform) in query.iter_mut() {
        // move_boid_towards_perceived_scenter_of_mass(&boid, &boids, &mut velocity, &transform);
        teleport_boid(&mut boid, window);
        limit_boid_speed(&mut boid);
        apply_velocity_to_boid(&mut boid, &mut transform);
    }
}

// fn move_boid_towards_perceived_scenter_of_mass(
//     boid: &Entity,
//     boids: &Vec<(Entity, Vec3)>,
//     velocity: &mut Mut<Velocity>,
//     transform: &Mut<Transform>,
// ) {
//     let mut arr = Vec3::new(0.0, 0.0, 0.0);
//     let mut len: u32 = 0;
//     for (e, translation) in boids.to_owned() {
//         let angle =
//             (transform.translation + velocity.0).angle_between(transform.translation + translation);
//         if e != *boid
//             && transform.translation.distance(translation) < BOID_SENSING_RADIUS
//             && angle < PI / 6.0
//             && angle > -PI / 6.0
//         {
//             arr += translation;
//             len += 1;
//         }
//     }
//     arr.z = 1.0;
//     let perceived_center = arr / len as f32;
//     let direction = Vec2::new(
//         perceived_center.x - transform.translation.x,
//         perceived_center.y - transform.translation.y,
//     )
//     .normalize();
//     velocity.x += direction.x * BOID_ACCELERATION;
//     velocity.y += direction.y * BOID_ACCELERATION;
// }

// fn test<'a>(mut velocity: Mut<'a, Velocity>) -> Mut<'a, Velocity> {
//     velocity
// }

fn limit_boid_speed(boid: &mut Mut<Boid>) {
    if boid.velocity.length() > MAX_BOID_SPEED {
        boid.velocity = boid.velocity.normalize() * MAX_BOID_SPEED * crate::TIME_STEP;
    }
}

fn teleport_boid(boid: &mut Mut<Boid>, window: &Window) {
    if boid.position.x > window.width() / 2.0 {
        boid.position.x = window.width() / 2.0 * -1.0
    } else if boid.position.x < window.width() / 2.0 * -1.0 {
        boid.position.x = window.width() / 2.0
    } else if boid.position.y > window.height() / 2.0 {
        boid.position.y = window.height() / 2.0 * -1.0
    } else if boid.position.y < window.height() / 2.0 * -1.0 {
        boid.position.y = window.height() / 2.0
    }
}

fn apply_velocity_to_boid(boid: &mut Mut<Boid>, transform: &mut Transform) {
    let velocity = boid.velocity;
    boid.position += velocity;
    transform.translation.x = boid.position.x;
    transform.translation.y = boid.position.y;
}
