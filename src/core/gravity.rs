use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

pub const GRAVITY: f32 = 9.8 * 100.;

#[derive(Clone, Debug, Copy)]
pub enum GravityDirection {
    Down,
    Right,
    Up,
    Left,
}

impl GravityDirection {
    pub fn get_vec(&self) -> Vec2 {
        match *self {
            GravityDirection::Down => Vec2::NEG_Y,
            GravityDirection::Right => Vec2::X,
            GravityDirection::Up => Vec2::Y,
            GravityDirection::Left => Vec2::NEG_X,
        }
    }

    pub fn get_index(&self) -> u32 {
        match *self {
            GravityDirection::Down => 0,
            GravityDirection::Right => 1,
            GravityDirection::Up => 2,
            GravityDirection::Left => 3,
        }
    }

    pub fn get_from_config(config: &RapierConfiguration) -> Self {
        let dir = config.gravity.normalize();
        if dir.y > 0.1 {
            return GravityDirection::Up;
        }
        if dir.x < -0.1 {
            return GravityDirection::Left;
        }
        if dir.x > 0.1 {
            return GravityDirection::Right;
        }
        return GravityDirection::Down;
    }
}
