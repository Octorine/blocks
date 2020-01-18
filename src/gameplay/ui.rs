use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::ecs::{Builder, World, WorldExt};
use amethyst::ecs::{DenseVecStorage, Entity};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

#[derive(Debug)]
pub struct UiModel {
    pub lives: u32,
    pub score: u32,
}
impl Default for UiModel {
    fn default() -> UiModel {
        UiModel { lives: 3, score: 0 }
    }
}

pub struct UiViews {
    pub lives: Entity,
    pub score: Entity,
}

pub fn initialize_ui(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let lives_transform = UiTransform::new(
        "lives".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -500.,
        -10.,
        1.,
        200.,
        50.,
    );
    let score_transform = UiTransform::new(
        "score".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        500.,
        -10.,
        1.,
        200.,
        50.,
    );
    let lives = world
        .create_entity()
        .with(lives_transform)
        .with(UiText::new(
            font.clone(),
            "3 LIVES".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();
    let score = world
        .create_entity()
        .with(score_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();
    world.insert(UiViews { lives, score });
    world.insert(UiModel { lives: 3, score: 0 });
}
