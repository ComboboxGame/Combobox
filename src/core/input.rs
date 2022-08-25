use crate::core::MapDirection;
use crate::KeyCode;

pub enum MoveKeyGroups {
    WASD,
    Arrows,
}

impl MoveKeyGroups {
    pub fn get_key(&self, direction: MapDirection) -> KeyCode {
        let keys = match *self {
            MoveKeyGroups::WASD => [KeyCode::S, KeyCode::D, KeyCode::W, KeyCode::A],
            MoveKeyGroups::Arrows => [KeyCode::Down, KeyCode::Right, KeyCode::Up, KeyCode::Left],
        };
        keys[direction.get_index() as usize]
    }
}
