use amethyst::ecs::{Component, DenseVecStorage};

pub const BALL_RADIUS: f32 = 4.0;
const BALL_VELOCITY_X: f32 = 60.0;
const BALL_VELOCITY_Y: f32 = 0.0;

pub struct Ball {
    /// Rate of change of balls position.
    /// ```
    /// let x = ball.velocity[0];
    /// let y = ball.velocity[1];
    /// ```
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
            radius: BALL_RADIUS,
        }
    }

    pub fn reset_y(&mut self) {
        self.velocity[1] = 0.0;
    }

    pub fn reverse_x(&mut self) {
        self.velocity[0] = -self.velocity[0];
    }

    pub fn reverse_y(&mut self) {
        self.velocity[1] = -self.velocity[1];
    }

    pub fn heads_left(&self) -> bool {
        self.velocity[0] < 0.0
    }

    pub fn heads_right(&self) -> bool {
        self.velocity[0] > 0.0
    }
}
