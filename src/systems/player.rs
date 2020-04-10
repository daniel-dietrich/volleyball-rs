use crate::{
    components::player::{Player, Side, PLAYER_WIDTH},
    volleyball::WINDOW_WIDTH,
};
use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

const PLAYER_SPEED: f32 = 120.0;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, players, time, input): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            // Workaround for intellisense
            let player: &Player = player;
            let transform: &mut Transform = transform;

            let movement = match player.side {
                Side::Left => input.axis_value("left_player"),
                Side::Right => input.axis_value("right_player"),
            };

            if let Some(mv_amount) = movement {
                let player_x = transform.translation().x;

                let left_boundary = match player.side {
                    Side::Left => 0.0,
                    Side::Right => WINDOW_WIDTH / 2.0,
                };

                if mv_amount != 0.0 {
                    let scaled_amount = PLAYER_SPEED * mv_amount * time.delta_seconds();
                    transform.set_translation_x(
                        (player_x + scaled_amount)
                            .max(left_boundary + PLAYER_WIDTH / 2.0)
                            .min(left_boundary + WINDOW_WIDTH / 2.0 - PLAYER_WIDTH / 2.0),
                    );
                }
            }
        }
    }
}
