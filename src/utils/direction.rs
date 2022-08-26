use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

#[derive(Clone, Debug, Copy)]
pub enum SceneDirection {
    Down,
    Right,
    Up,
    Left,
}

impl SceneDirection {
    pub fn get_vec(&self) -> Vec2 {
        match *self {
            SceneDirection::Down => Vec2::NEG_Y,
            SceneDirection::Right => Vec2::X,
            SceneDirection::Up => Vec2::Y,
            SceneDirection::Left => Vec2::NEG_X,
        }
    }

    pub fn get_opposite(&self) -> Self {
        Self::from_index(self.get_index() + 2)
    }

    pub fn get_perp(&self) -> Self {
        Self::from_index(self.get_index() + 1)
    }

    pub fn get_index(&self) -> u32 {
        match *self {
            SceneDirection::Down => 0,
            SceneDirection::Right => 1,
            SceneDirection::Up => 2,
            SceneDirection::Left => 3,
        }
    }

    pub fn from_index(index: u32) -> Self {
        match index % 4 {
            0 => Self::Down,
            1 => Self::Right,
            2 => Self::Up,
            3 => Self::Left,
            _ => Self::Down,
        }
    }

    pub fn from_gravity_direction(config: &RapierConfiguration) -> Self {
        let dir = config.gravity.normalize();
        if dir.y > 0.1 {
            return SceneDirection::Up;
        }
        if dir.x < -0.1 {
            return SceneDirection::Left;
        }
        if dir.x > 0.1 {
            return SceneDirection::Right;
        }
        return SceneDirection::Down;
    }
}
