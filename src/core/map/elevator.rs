use bevy::ecs::system::EntityCommands;
use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::{Collider, CollisionGroups, RigidBody};

use crate::core::{Elevator, ElevatorType, MapBuilder, ELEVATOR_BIT, ELEVATOR_FILTER};
use crate::game::Material;

#[derive(Bundle)]
pub struct ElevatorBundle {
    pub elevator: Elevator,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    #[bundle]
    pub mesh_bundle: MaterialMesh2dBundle<Material>,
    pub collision_groups: CollisionGroups,
}

impl ElevatorBundle {
    pub fn new(
        elevator: Elevator,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<Material>,
    ) -> Self {
        let start = elevator.start.clone();
        ElevatorBundle {
            elevator,
            rigid_body: RigidBody::KinematicPositionBased,
            collider: Collider::cuboid(30.0, 3.0),
            mesh_bundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Quad::new(Vec2::new(60.0, 6.0)).into())),
                material: materials.add(Color::BLUE.into()),
                transform: Transform::from_xyz(start.x, start.y, -1.0),
                ..MaterialMesh2dBundle::default()
            },
            collision_groups: CollisionGroups::new(ELEVATOR_BIT, ELEVATOR_FILTER),
        }
    }
}

impl<'w, 's, 'a, 'b> MapBuilder<'w, 's, 'a, 'b> {
    pub fn spawn_elevator_xy(
        &mut self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        elevator_type: ElevatorType,
    ) -> EntityCommands<'w, 's, '_> {
        self.spawn_elevator(
            Vec2::new(start_x, start_y),
            Vec2::new(end_x, end_y),
            elevator_type,
        )
    }

    pub fn spawn_elevator(
        &mut self,
        start: Vec2,
        end: Vec2,
        elevator_type: ElevatorType,
    ) -> EntityCommands<'w, 's, '_> {
        self.builder.spawn_bundle(ElevatorBundle::new(
            Elevator {
                start,
                end,
                elevator_type,
            },
            self.meshes,
            self.materials,
        ))
    }
}
