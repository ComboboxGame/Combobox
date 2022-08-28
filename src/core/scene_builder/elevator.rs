use bevy::ecs::system::EntityCommands;
use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;

use crate::core::{collision_groups, Elevator, ElevatorType, Material, SceneBuilder};

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
        let dir = (elevator.end - elevator.start).normalize();

        let (w, h) = if dir.dot(Vec2::Y).abs() > 0.8 {
            (Elevator::WIDTH, Elevator::HEIGHT)
        } else {
            (Elevator::HEIGHT, Elevator::WIDTH)
        };

        ElevatorBundle {
            elevator,
            rigid_body: RigidBody::KinematicPositionBased,
            collider: Collider::cuboid(w * 0.5, h * 0.5),
            mesh_bundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Quad::new(Vec2::new(w, h)).into())),
                material: materials.add(Color::rgb_u8(33, 41, 36).into()),
                transform: Transform::from_xyz(start.x, start.y, SceneBuilder::ELEVATOR_DEPTH),
                ..MaterialMesh2dBundle::default()
            },
            collision_groups: collision_groups::ELEVATOR,
        }
    }
}

impl<'w, 's, 'a, 'b> SceneBuilder<'w, 's, 'a, 'b> {
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
        mut start: Vec2,
        mut end: Vec2,
        elevator_type: ElevatorType,
    ) -> EntityCommands<'w, 's, '_> {
        start *= SceneBuilder::CELL_SIZE;
        end *= SceneBuilder::CELL_SIZE;
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
