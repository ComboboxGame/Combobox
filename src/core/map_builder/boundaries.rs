use crate::core::MapBuilder;
use bevy::prelude::*;
use bevy::sprite::Rect;

#[derive(Default, Debug, Clone)]
pub struct MapBoundaries {
    pub rect: Option<Rect>,
}

impl<'w, 's, 'a, 'b> MapBuilder<'w, 's, 'a, 'b> {
    pub fn set_boundaries(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        assert!(left <= right);
        assert!(bottom <= top);
        self.boundaries.rect = Some(Rect {
            min: Vec2::new(left, bottom),
            max: Vec2::new(right, top),
        });
    }
}
