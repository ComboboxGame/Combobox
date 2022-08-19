use crate::core::MapBuilder;
use crate::game::Material;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
    wall: Wall,
    rigid_body: RigidBody,

    #[bundle]
    mesh: MaterialMesh2dBundle<Material>,
}

impl<'b> MapBuilder<'b> {
    pub fn build_wall_from_to_xy<'w, 's, 'a, 'c>(
        &'c mut self,
        parent: &'c mut ChildBuilder<'w, 's, 'a>,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
    ) -> EntityCommands<'w, 's, '_> {
        self.build_wall_from_to(parent, Vec2::new(left, bottom), Vec2::new(right, top))
    }

    pub fn build_wall_from_to<'w, 's, 'a, 'c>(
        &'c mut self,
        parent: &'c mut ChildBuilder<'w, 's, 'a>,
        from: Vec2,
        to: Vec2,
    ) -> EntityCommands<'w, 's, '_> {
        let size = from.max(to) - from.min(to);
        let translation = (from + to) * 0.5;
        parent.spawn_bundle(WallBundle {
            wall: Wall::default(),
            rigid_body: RigidBody::Fixed,
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(self.meshes.add(Quad::new(size).into())),
                material: self.wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(
                    translation.x,
                    translation.y,
                    0.0,
                )),
                ..default()
            },
        })
    }
}
