use crate::config::GameConfig;
use crate::gameplay::{Ball, Block, Collider, Collision, Paddle, UiModel, UiViews};
use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData, World, Write, WriteStorage,
};
use amethyst::ui::UiText;

#[derive(SystemDesc)]
pub struct LifeSystem;

impl<'a> System<'a> for LifeSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Transform>,
        Write<'a, UiModel>,
        ReadExpect<'a, UiViews>,
        WriteStorage<'a, UiText>,
        Entities<'a>,
        Read<'a, GameConfig>,
    );
    fn run(
        &mut self,
        (mut balls, mut xforms, mut model, views, mut ui_text, entities, cfg): Self::SystemData,
    ) {
        let total_balls = balls.join().count();
        let mut ball_count = total_balls;
        let min_y = -cfg.ball_radius;
        let mut new_ball_needed = false;
        for (mut ball, mut xform, entity) in (&mut balls, &mut xforms, &entities).join() {
            if xform.translation().y < min_y {
                ball_count -= 1;
                if ball_count > 1 {
                    entities.delete(entity);
                } else {
                    model.lives -= 1;
                    if let Some(txt) = ui_text.get_mut(views.lives) {
                        txt.text = format!("{} LIVES", model.lives);
                    }
                    xform.set_translation_xyz(cfg.level_width * 0.5, cfg.level_height * 0.5, 0.0);
                }
            }
        }
        if ball_count == 0 {
            if model.lives > 0 {
                new_ball_needed = true;
            }
        }
        if new_ball_needed {
            //do something here.
        }
    }
}
