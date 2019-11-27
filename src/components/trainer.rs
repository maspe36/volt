use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};

pub const TRAINER_WIDTH: f32 = 34.0;
pub const TRAINER_HEIGHT: f32 = 52.0;

pub struct Trainer {
    pub width: f32,
    pub height: f32,
}

impl Trainer {
    pub fn new() -> Trainer {
        Trainer {
            width: TRAINER_WIDTH,
            height: TRAINER_HEIGHT
        }
    }
}

impl Component for Trainer {
    type Storage = DenseVecStorage<Self>;
}
