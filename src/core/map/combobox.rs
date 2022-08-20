use bevy::ecs::system::EntityCommands;
use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::{Collider, LockedAxes, RigidBody};

use crate::core::{Combobox, MapBuilder};
use crate::game::Material;

#[derive(Bundle)]
pub struct ComboboxBundle {
    pub combobox: Combobox,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    #[bundle]
    pub mesh_bundle: MaterialMesh2dBundle<Material>,
    pub locked_axes: LockedAxes,
}

impl ComboboxBundle {
    pub fn new(
        combobox: Combobox,
        position: Vec2,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<Material>,
    ) -> Self {
        ComboboxBundle {
            combobox: combobox.clone(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(combobox.size * 0.5, combobox.size * 0.5),
            mesh_bundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Quad::new(Vec2::ONE * combobox.size).into())),
                material: materials.add(Color::rgb_u8(255, 140, 90).into()),
                transform: Transform::from_xyz(position.x, position.y, 0.0),
                ..MaterialMesh2dBundle::default()
            },
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }
}

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
        self.builder.spawn_bundle(ComboboxBundle::new(
            combobox,
            position,
            self.meshes,
            self.materials,
        ))
    }
}