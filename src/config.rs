use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GameConfig {
    pub level_width: f32,
    pub level_height: f32,
    pub ball_radius: f32,
    pub ball_velocity: f32,
    pub paddle_height: f32,
    pub paddle_width: f32,
    pub paddle_speed: f32,
    pub block_width: f32,
    pub block_height: f32,
    pub block_margin: f32,
}
