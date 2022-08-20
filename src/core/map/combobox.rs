use bevy::ecs::system::EntityCommands;
use bevy::prelude::shape::Quad;
use bevy::prelude::{Transform, Vec2};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::core::{Combobox, MapBuilder};

impl<'w, 's, 'a, 'b> MapBuilder<'w, 's, 'a, 'b> {
    pub fn spawn_box_xy(
        &mut self,
        combobox: Combobox,
        x: f32,
        y: f32,
    ) -> EntityCommands<'w, 's, '_> {
        self.spawn_box(combobox, Vec2::new(x, y))
    }

    pub fn spawn_box(&mut self, combobox: Combobox, position: Vec2) -> EntityCommands<'w, 's, '_> {
        let mut ec = self.builder.spawn();

        ec.insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(combobox.size * 0.5, combobox.size * 0.5))
            .insert_bundle(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(self.meshes.add(Quad::new(Vec2::ONE * combobox.size).into())),
                material: self.box_material.clone(),
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..MaterialMesh2dBundle::default()
            })
            .insert(combobox);

        ec
    }
}
