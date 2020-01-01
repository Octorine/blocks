use amethyst::ecs::{Component, DenseVecStorage};

pub struct Block {
    pub health: f32,
}

impl Block {
    pub fn new() -> Block {
        Block { health: 1.0 }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
