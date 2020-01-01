use amethyst::ecs::{Component, DenseVecStorage};

pub struct Paddle;

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
