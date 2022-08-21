use crate::core::MapBuilder;
use bevy::prelude::*;


#[derive(Debug, Clone, Component)]
pub struct SpawnPoint {
    pub id: u32,
}

impl<'w, 's, 'a, 'b> MapBuilder<'w, 's, 'a, 'b> {
    pub fn set_spawn_point_xy(&mut self, x: f32, y: f32, id: u32) {
        self.set_spawn_point(Vec2::new(x, y), id);
    }

    pub fn set_spawn_point(&mut self, position: Vec2, id: u32) {
        self.builder
            .spawn()
            .insert(SpawnPoint { id })
            .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
                position.x, position.y, 0.0,
            )))
            .insert_bundle(VisibilityBundle::default());
    }
}
