use crate::core::SceneBuilder;
use bevy::prelude::*;
use bevy::sprite::Rect;

#[derive(Default, Debug, Clone)]
pub struct SceneBoundaries {
    pub rect: Option<Rect>,
    pub view_range: Option<f32>,
}

impl<'w, 's, 'a, 'b> SceneBuilder<'w, 's, 'a, 'b> {
    pub fn set_boundaries(&mut self, left: f32, right: f32, bottom: f32, top: f32) {
        assert!(left <= right);
        assert!(bottom <= top);
        self.boundaries.rect = Some(Rect {
            min: Vec2::new(left, bottom),
            max: Vec2::new(right, top),
        });
    }

    pub fn set_min_view_range(&mut self, range: f32) {
        self.boundaries.view_range = Some(range);
    }
}
