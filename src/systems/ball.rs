use crate::components::ball::Ball;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

const ACCELERATION: f32 = -120.0;
const ROTATION: f32 = 0.05;

#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut balls, mut transforms, time): Self::SystemData) {
        let timestep = time.delta_seconds();

        for (ball, transform) in (&mut balls, &mut transforms).join() {
            // Workaround for intellisense
            let transform: &mut Transform = transform;
            let ball: &mut Ball = ball;

            // Rotation
            transform.rotate_2d(ROTATION);

            // Gravity
            transform.prepend_translation_x(ball.velocity[0] * timestep);
            transform.prepend_translation_y(
                (ball.velocity[1] + timestep * ACCELERATION / 2.0) * timestep,
            );

            ball.velocity[1] += timestep * ACCELERATION;
        }
    }
}
