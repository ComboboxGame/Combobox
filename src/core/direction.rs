use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

#[derive(Clone, Debug, Copy)]
pub enum Direction {
    Down,
    Right,
    Up,
    Left,
}

impl Direction {
    pub fn get_vec(&self) -> Vec2 {
        match *self {
            Direction::Down => Vec2::NEG_Y,
            Direction::Right => Vec2::X,
            Direction::Up => Vec2::Y,
            Direction::Left => Vec2::NEG_X,
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
            Direction::Down => 0,
            Direction::Right => 1,
            Direction::Up => 2,
            Direction::Left => 3,
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

    pub fn gravity_direction(config: &RapierConfiguration) -> Self {
        let dir = config.gravity.normalize();
        if dir.y > 0.1 {
            return Direction::Up;
        }
        if dir.x < -0.1 {
            return Direction::Left;
        }
        if dir.x > 0.1 {
            return Direction::Right;
        }
        return Direction::Down;
    }

    pub fn
}
