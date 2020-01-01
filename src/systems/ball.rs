use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};

use crate::config::GameConfig;
use crate::gameplay::{Ball, Block, Collider, Collision, Paddle};
#[derive(SystemDesc)]
pub struct MoveBallSystem;

impl<'a> System<'a> for MoveBallSystem {
    type SystemData = (
        ReadStorage<'a, Ball>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Collider>,
        Read<'a, GameConfig>,
    );
    fn run(&mut self, (balls, mut xforms, mut colliders, cfg): Self::SystemData) {
        for (ball, xform, collider) in (&balls, &mut xforms, &mut colliders).join() {
            let mut current_translation = xform.translation().clone();
            current_translation.x += ball.velocity_x * cfg.ball_velocity;
            current_translation.y += ball.velocity_y * cfg.ball_velocity;
            xform.set_translation(current_translation);
            collider.update(xform)
        }
    }
}

#[derive(SystemDesc)]
pub struct BounceBallSystem;

impl<'a> System<'a> for BounceBallSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Collider>,
        WriteStorage<'a, Block>,
        Read<'a, GameConfig>,
    );
    fn run(&mut self, (mut balls, xforms, paddles, colliders, mut blocks, cfg): Self::SystemData) {
        for (mut ball, ball_xform, ball_collider) in (&mut balls, &xforms, &colliders).join() {
            let x = ball_xform.translation().x;
            let y = ball_xform.translation().y;
            let max_x = cfg.level_width - cfg.ball_radius;
            let min_x = cfg.ball_radius;
            let max_y = cfg.level_height - cfg.ball_radius;
            if x < min_x || x > max_x {
                ball.velocity_x *= -1.0;
                // xform.set_translation_x(x.min(max_x).max(min_x));
            }
            if y > max_y {
                // xform.set_translation_y(max_y);
                ball.velocity_y *= -1.0;
            }
            for (_paddle, paddle_xform, paddle_collider) in (&paddles, &xforms, &colliders).join() {
                match ball_collider.collide(&paddle_collider) {
                    Collision::NoCollision => (),
                    Collision::CollideLeft => {
                        ball.velocity_x = -ball.velocity_x.abs();
                    }
                    Collision::CollideRight => {
                        ball.velocity_x = ball.velocity_x.abs();
                    }
                    Collision::CollideUp => {
                        ball.velocity_y = -ball.velocity_y.abs();
                    }
                    Collision::CollideDown => {
                        ball.velocity_y = ball.velocity_y.abs();
                    }
                };
                for (mut block, block_xform, block_collider) in
                    (&mut blocks, &xforms, &colliders).join()
                {
                    match ball_collider.collide(&block_collider) {
                        Collision::NoCollision => (),
                        Collision::CollideLeft => {
                            ball.velocity_x = -ball.velocity_x.abs();
                            block.health -= 1.0;
                        }
                        Collision::CollideRight => {
                            ball.velocity_x = ball.velocity_x.abs();
                            block.health -= 1.0;
                        }
                        Collision::CollideUp => {
                            ball.velocity_y = -ball.velocity_y.abs();
                            block.health -= 1.0;
                        }
                        Collision::CollideDown => {
                            ball.velocity_y = ball.velocity_y.abs();
                            block.health -= 1.0;
                        }
                    };
                }
            }
        }
    }
}
