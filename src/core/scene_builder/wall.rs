use crate::core::{collision_groups, Material, SceneBuilder};
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
    collider: Collider,

    #[bundle]
    mesh: MaterialMesh2dBundle<Material>,
    collision_groups: CollisionGroups,
}

impl<'w, 's, 'a, 'b> SceneBuilder<'w, 's, 'a, 'b> {
    pub fn spawn_wall_from_to_xy(
        &mut self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
    ) -> EntityCommands<'w, 's, '_> {
        self.spawn_wall_from_to(Vec2::new(left, bottom), Vec2::new(right, top))
    }

    pub fn spawn_wall_from_to(
        &mut self,
        mut from: Vec2,
        mut to: Vec2,
    ) -> EntityCommands<'w, 's, '_> {
        from *= Self::CELL_SIZE;
        to *= Self::CELL_SIZE;
        let size = from.max(to) - from.min(to);
        let translation = (from + to) * 0.5;
        self.builder.spawn_bundle(WallBundle {
            wall: Wall::default(),
            collider: Collider::cuboid(size.x * 0.5, size.y * 0.5),
            rigid_body: RigidBody::Fixed,
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(self.meshes.add(Quad::new(size).into())),
                material: self.wall_material.clone(),
                transform: Transform::from_translation(Vec3::new(
                    translation.x,
                    translation.y,
                    Self::WALL_DEPTH,
                )),
                ..default()
            },
            collision_groups: collision_groups::WALL,
        })
    }
}
