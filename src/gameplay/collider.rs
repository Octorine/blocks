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
    pub fn collide(&self, other: &Collider) -> Collision {
        if self.right < other.left
            || self.left > other.right
            || self.top < other.bottom
            || self.bottom > other.top
        {
            Collision::NoCollision
        } else {
            let x = (self.left + self.right) / 2.0;
            let y = (self.top + self.bottom) / 2.0;
            let ox = (other.left + other.right) / 2.0;
            let oy = (other.top + other.bottom) / 2.0;
            if ((x - ox) / (other.right - other.left)).abs()
                > ((y - oy) / (other.top - other.bottom)).abs()
            {
                if x < ox {
                    Collision::CollideRight
                } else {
                    Collision::CollideLeft
                }
            } else if y < oy {
                Collision::CollideUp
            } else {
                Collision::CollideDown
            }
        }
    }
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}
