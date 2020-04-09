use crate::{
    audio::{play_sound, Sounds},
    components::{
        ball::{Ball, BALL_RADIUS},
        player::{Player, Side},
    },
    volleyball::ARENA_WIDTH,
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
};
use rand::Rng;
use std::ops::Deref;

const LEFT_EDGE: f32 = BALL_RADIUS;
const RIGHT_EDGE: f32 = ARENA_WIDTH - BALL_RADIUS;

#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (mut balls, players, transforms, storage, sounds, audio_output): Self::SystemData,
    ) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            // Workaround for intellisense
            let ball: &mut Ball = ball;
            let transform: &Transform = transform;

            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            if ball.heads_left() && ball_x <= LEFT_EDGE {
                ball.reverse_x();
                play_sound(
                    &sounds.bounce,
                    &storage,
                    audio_output.as_ref().map(|o| o.deref()),
                );
            } else if ball.heads_right() && ball_x >= RIGHT_EDGE {
                ball.reverse_x();
                play_sound(
                    &sounds.bounce,
                    &storage,
                    audio_output.as_ref().map(|o| o.deref()),
                );
            }

            for (player, transform) in (&players, &transforms).join() {
                // Workaround for intellisense
                let player: &Player = player;
                let transform: &Transform = transform;

                let player_x = transform.translation().x - (player.width * 0.5);
                let player_y = transform.translation().y - (player.height * 0.5);

                if collides_with_player(ball_x, ball_y, player_x, player_y) {
                    if ball.velocity[1] < 0.0 {
                        ball.reverse_y();
                        play_sound(
                            &sounds.bounce,
                            &storage,
                            audio_output.as_ref().map(|o| o.deref()),
                        );

                        let mut rng = rand::thread_rng();
                        ball.velocity[0] = match player.side {
                            Side::Left => ball.velocity[0].abs() * rng.gen_range(0.6, 1.4),
                            Side::Right => -ball.velocity[0].abs() * rng.gen_range(0.6, 1.4),
                        };
                    }
                }
            }
        }
    }
}

fn collides_with_player(ball_x: f32, ball_y: f32, player_x: f32, player_y: f32) -> bool {
    use crate::components::player::{PLAYER_HEIGHT, PLAYER_WIDTH};

    let left = player_x - BALL_RADIUS;
    let bottom = player_y - BALL_RADIUS;
    let right = player_x + PLAYER_WIDTH + BALL_RADIUS;
    let top = player_y + PLAYER_HEIGHT + BALL_RADIUS;

    (ball_x >= left) && (ball_y >= bottom) && (ball_x <= right) && (ball_y <= top)
}
