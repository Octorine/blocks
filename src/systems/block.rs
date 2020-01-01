use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, ReadStorage, System, SystemData, World, WriteStorage};

use crate::config::GameConfig;
use crate::gameplay::{Ball, Block, Collider, Collision, Paddle};
#[derive(SystemDesc)]
pub struct BlockSystem;

impl<'a> System<'a> for BlockSystem {
    type SystemData = (ReadStorage<'a, Block>, Entities<'a>);
    fn run(&mut self, (blocks, entities): Self::SystemData) {
        for (entity, block) in (&entities, &blocks).join() {
            if block.health < 1.0 {
                entities.delete(entity);
            }
        }
    }
}
