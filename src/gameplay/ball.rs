use amethyst::ecs::{Component, DenseVecStorage};

pub struct Ball {
    pub velocity_x: f32,
    pub velocity_y: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
