use crate::core::{MapBuilder, PlayerIndex};
use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Debug, Clone, Component)]
pub struct SpawnPoint {
    pub index: PlayerIndex,
}

#[derive(Debug, Clone, Component)]
pub struct FinishPoint;

#[derive(Debug, Clone, Component)]
pub struct FinishPointArrow;

impl<'w, 's, 'a, 'b> MapBuilder<'w, 's, 'a, 'b> {
    pub fn set_spawn_point_xy(&mut self, x: f32, y: f32, index: PlayerIndex) {
        self.set_spawn_point(Vec2::new(x, y), index);
    }

    pub fn set_spawn_point(&mut self, position: Vec2, index: PlayerIndex) {
        self.builder
            .spawn()
            .insert(SpawnPoint { index })
            .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
                position.x, position.y, 0.0,
            )))
            .insert_bundle(VisibilityBundle::default());
    }

    pub fn set_finish_point_xy(&mut self, x: f32, y: f32) {
        self.set_finish_point(Vec2::new(x, y));
    }

    pub fn set_finish_point(&mut self, position: Vec2) {
        self.builder
            .spawn()
            .insert(FinishPoint)
            .insert_bundle(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(self.meshes.add(Quad::new(Vec2::new(200.0, 200.0)).into())),
                material: self
                    .materials
                    .add(self.assets.load("images/finish.png").into()),
                transform: Transform::from_xyz(position.x, position.y, -1.0),
                ..default()
            }).with_children(|parent| {
            parent.spawn()
                .insert(FinishPointArrow)
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(self.meshes.add(Quad::new(Vec2::new(50.0, 50.0)).into())),
                    material: self
                        .materials
                        .add(self.assets.load("images/finish-arrow.png").into()),
                    ..default()
                });
        });
    }
}
