use amethyst::ecs::{Component, DenseVecStorage};

pub const PLAYER_HEIGHT: f32 = 64.0;
pub const PLAYER_WIDTH: f32 = 44.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Player {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Player {
    pub fn new(side: Side) -> Player {
        Player {
            side,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
        }
    }
}
