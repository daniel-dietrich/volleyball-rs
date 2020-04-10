use crate::{
    audio::{play_sound, Sounds},
    components::ball::{Ball, BALL_RADIUS},
    volleyball::{ScoreBoard, ScoreText, WINDOW_HEIGHT, WINDOW_WIDTH},
};
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, System, SystemData, Write, WriteStorage},
    ui::UiText,
};

const BOTTOM_EDGE: f32 = BALL_RADIUS;
#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (
            mut balls,
            mut transforms,
            mut ui_text,
            mut score,
            score_text,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            // Workaround for intellisense
            let transform: &mut Transform = transform;
            let ball: &mut Ball = ball;

            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            // Check if and on which side the ball hits the floor
            if ball_y <= BOTTOM_EDGE {
                play_sound(&sounds.score, &storage, audio_output.as_deref());

                if ball_x < (WINDOW_WIDTH / 2.0) {
                    score.player_2 = (score.player_2 + 1).min(99);

                    if let Some(text) = ui_text.get_mut(score_text.player_2) {
                        text.text = score.player_2.to_string();
                    }
                } else {
                    score.player_1 = (score.player_1 + 1).min(99);

                    if let Some(text) = ui_text.get_mut(score_text.player_1) {
                        text.text = score.player_1.to_string();
                    }
                }

                transform.set_translation_xyz(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 0.0);
                ball.reset();
                ball.reverse_x();
            }
        }
    }
}
