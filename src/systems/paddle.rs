use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::config::GameConfig;
use crate::gameplay::{Collider, Paddle};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'a> System<'a> for PaddleSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Paddle>,
        Read<'a, InputHandler<StringBindings>>,
        WriteStorage<'a, Collider>,
        Read<'a, GameConfig>,
    );
    fn run(&mut self, (mut xforms, paddles, input, mut colliders, cfg): Self::SystemData) {
        if let Some(movement) = input.axis_value("paddle") {
            if movement != 0.0 {
                for (_paddle, xform, collider) in (&paddles, &mut xforms, &mut colliders).join() {
                    xform.set_translation_x(
                        (xform.translation().x + movement * cfg.paddle_speed as f32)
                            .max(cfg.paddle_width * 0.5)
                            .min(cfg.level_width - cfg.paddle_width * 0.5),
                    );
                    collider.update(xform);
                }
            }
        }
    }
}
