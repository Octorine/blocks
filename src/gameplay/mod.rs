mod ball;
mod block;
mod collider;
mod paddle;
mod spritesheet;
mod ui;

pub use self::ball::Ball;
pub use self::block::Block;
pub use self::collider::{Collider, Collision};
pub use self::paddle::Paddle;
use self::spritesheet::load_sprite_sheet;
pub use self::ui::{initialize_ui, UiModel, UiViews};
use crate::config::GameConfig;
use amethyst::assets::Handle;
use amethyst::config::Config;
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, SpriteRender, SpriteSheet};
use amethyst::utils::application_root_dir;

pub struct BlocksGame;

impl SimpleState for BlocksGame {
    fn on_start(&mut self, game_data: StateData<'_, GameData<'_, '_>>) {
        let world = game_data.world;
        let cfg = GameConfig::load(
            application_root_dir()
                .unwrap()
                .join("config")
                .join("game_config.ron"),
        );
        world.register::<Block>();
        world.insert(cfg.clone());
        let ss = load_sprite_sheet(world);
        initalize_camera(world, &cfg);
        initialize_ball(world, ss.clone(), &cfg);
        initialize_paddle(world, ss.clone(), &cfg);
        initialize_blocks(world, ss, &cfg);
        initialize_ui(world);
    }
}

fn initalize_camera(world: &mut World, cfg: &GameConfig) {
    let mut xform = Transform::default();
    xform.set_translation_xyz(cfg.level_width * 0.5, cfg.level_height * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(cfg.level_width, cfg.level_height))
        .with(xform)
        .build();
}

fn initialize_ball(world: &mut World, sprite_sheet: Handle<SpriteSheet>, cfg: &GameConfig) {
    let mut xform = Transform::default();
    let x = cfg.level_width * 0.5;
    let y = cfg.level_height * 0.5;

    xform.set_translation_xyz(x, y, 0.0);
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };
    let collider = Collider {
        left: x - cfg.ball_radius,
        right: x + cfg.ball_radius,
        top: y + cfg.ball_radius,
        bottom: y - cfg.ball_radius,
        xform: xform.clone(),
    };
    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            velocity_x: cfg.ball_velocity * 3.0,
            velocity_y: cfg.ball_velocity * 2.0,
        })
        .with(xform)
        .with(collider)
        .build();
}

fn initialize_paddle(world: &mut World, sprite_sheet: Handle<SpriteSheet>, cfg: &GameConfig) {
    let mut xform = Transform::default();
    let x = cfg.level_width * 0.5;
    let y = cfg.paddle_height * 0.5;
    xform.set_translation_xyz(x, y, 0.0);
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 1,
    };
    let collider = Collider {
        left: x - cfg.paddle_width * 0.5,
        right: x + cfg.paddle_width * 0.5,
        top: y + cfg.paddle_height * 0.5,
        bottom: y - cfg.paddle_height * 0.5,
        xform: xform.clone(),
    };
    world
        .create_entity()
        .with(sprite_render)
        .with(Paddle)
        .with(xform)
        .with(collider)
        .build();
}
fn initialize_blocks(world: &mut World, sprite_sheet: Handle<SpriteSheet>, cfg: &GameConfig) {
    let mut block_type = 0;
    let number_of_columns = 13;
    let number_of_rows = 5;
    let top_margin = cfg.block_height * 0.5 + 50.;
    let left_margin = cfg.block_width * 0.5
        + (cfg.level_width
            - (number_of_columns as f32 * cfg.block_width
                + (number_of_columns as f32 - 1.0) * cfg.block_margin))
            * 0.5;
    for row in 0..number_of_rows {
        for column in 0..number_of_columns {
            let mut xform = Transform::default();
            let x = left_margin + column as f32 * (cfg.block_margin + cfg.block_width);
            let y = cfg.level_height
                - top_margin
                - (cfg.block_margin + row as f32 * (cfg.block_margin + cfg.block_height));

            xform.set_translation_xyz(x, y, 0.0);
            let sprite = SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: block_type + 2,
            };
            let collider = Collider {
                left: x - cfg.block_width * 0.5,
                right: x + cfg.block_width * 0.5,
                top: y + cfg.block_height * 0.5,
                bottom: y - cfg.block_height * 0.5,
                xform: xform.clone(),
            };
            world
                .create_entity()
                .with(xform)
                .with(sprite)
                .with(Block::new())
                .with(collider)
                .build();
            block_type = (block_type + 1) % 4;
        }
    }
}
