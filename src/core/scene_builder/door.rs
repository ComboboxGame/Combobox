use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;

use crate::core::{collision_groups, Door, DoorButton, Material, SceneBuilder};
use crate::utils::SceneDirection;

#[derive(Bundle)]
pub struct DoorBundle {
    pub door: Door,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    #[bundle]
    pub mesh_bundle: MaterialMesh2dBundle<Material>,
    pub collision_groups: CollisionGroups,
}

impl DoorBundle {
    pub fn new(door: Door, meshes: &mut Assets<Mesh>, materials: &mut Assets<Material>) -> Self {
        let size_y = door.direction.get_vec() * door.height;
        let size_x = door.direction.get_perp().get_vec() * 30.0;
        let size = (size_y + size_x).abs();

        DoorBundle {
            door,
            rigid_body: RigidBody::KinematicPositionBased,
            collider: Collider::cuboid(size.x * 0.5, size.y * 0.5),
            mesh_bundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Quad::new(size).into())),
                material: materials.add(Color::DARK_GREEN.into()),
                ..MaterialMesh2dBundle::default()
            },
            collision_groups: collision_groups::ELEVATOR,
        }
    }
}

impl<'w, 's, 'a, 'b> SceneBuilder<'w, 's, 'a, 'b> {
    pub fn spawn_door_xy(
        &mut self,
        x: f32,
        y: f32,
        height: f32,
        direction: SceneDirection,
        pressed_mask: u32,
        not_pressed_mask: u32,
    ) {
        self.spawn_door(
            Vec2::new(x, y),
            height,
            direction,
            pressed_mask,
            not_pressed_mask,
        );
    }

    pub fn spawn_button_xy(&mut self, x: f32, y: f32, direction: SceneDirection, mask: u32) {
        self.spawn_button(Vec2::new(x, y), direction, mask);
    }

    pub fn spawn_button(&mut self, mut position: Vec2, direction: SceneDirection, mask: u32) {
        let size_y = direction.get_vec() * 55.0 * 3.0 / 20.0;
        let size_x = direction.get_perp().get_vec() * 55.0;
        let size = (size_y + size_x).abs();

        position *= Self::CELL_SIZE;

        position -= direction.get_vec() * (Self::CELL_SIZE - 1.0) * 0.5;

        self.builder
            .spawn()
            .insert_bundle(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(self.meshes.add(Quad::new(size).into())),
                material: self.button_off.clone(),
                transform: Transform::from_xyz(position.x, position.y, Self::DOOR_DEPTH),
                ..MaterialMesh2dBundle::default()
            })
            .insert(DoorButton {
                mask,
                direction,
                button_on: self.button_on.clone(),
                button_off: self.button_off.clone(),
                enabled: false,
            });
    }

    pub fn spawn_door(
        &mut self,
        position: Vec2,
        height: f32,
        direction: SceneDirection,
        pressed_mask: u32,
        not_pressed_mask: u32,
    ) {
        self.builder
            .spawn()
            .insert_bundle(VisibilityBundle::default())
            .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
                position.x * Self::CELL_SIZE,
                position.y * Self::CELL_SIZE,
                Self::DOOR_DEPTH,
            )))
            .with_children(|builder| {
                builder.spawn_bundle(DoorBundle::new(
                    Door {
                        height: height * Self::CELL_SIZE,
                        direction,
                        pressed_mask,
                        not_pressed_mask,
                        progress: 0.0,
                    },
                    self.meshes,
                    self.materials,
                ));
            });
    }
}
