use amethyst::core::Transform;
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Collider {
    pub xform: Transform,
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Collision {
    NoCollision,
    CollideLeft,
    CollideRight,
    CollideUp,
    CollideDown,
}

impl Collider {
    pub fn update(&mut self, xform: &Transform) {
        self.left += xform.translation().x - self.xform.translation().x;
        self.right += xform.translation().x - self.xform.translation().x;
        self.top += xform.translation().y - self.xform.translation().y;
        self.bottom += xform.translation().y - self.xform.translation().y;
        self.xform = xform.clone();
    }
    pub fn collide(&self, other: &Collider, dx: f32, dy: f32) -> Collision {
        if self.right < other.left
            || self.left > other.right
            || self.top < other.bottom
            || self.bottom > other.top
        {
            Collision::NoCollision
        } else if self.left - dx > other.right {
            Collision::CollideLeft
        } else if self.right - dx < other.left {
            Collision::CollideRight
        } else if self.top - dy < other.bottom {
            Collision::CollideUp
        } else {
            Collision::CollideDown
        }
    }
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}
