use bevy::ecs::system::EntityCommands;
use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;
use post_processing::PointLight2d;

use crate::core::{Combobox, ComboboxState, ComboboxType, Material, SceneBuilder};

#[derive(Bundle)]
pub struct ComboboxBundle {
    pub combobox: Combobox,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    #[bundle]
    pub mesh_bundle: MaterialMesh2dBundle<Material>,
    pub locked_axes: LockedAxes,
    pub combobox_state: ComboboxState,
    pub external_impulse: ExternalImpulse,
    pub read_mass: ReadMassProperties,
    pub collision_groups: CollisionGroups,
    pub velocity: Velocity,
    pub point_light: PointLight2d,
}

impl ComboboxBundle {
    pub fn new(
        combobox: Combobox,
        position: Vec2,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<Material>,
        assets: &mut AssetServer,
    ) -> Self {
        let color = match combobox.box_type {
            ComboboxType::Standard { group } => {
                if group == 1 {
                    Color::rgb_u8(87, 255, 162)
                } else {
                    Color::rgb_u8(255, 87, 162)
                }
            }
            ComboboxType::Buff(_) => Color::CYAN,
            ComboboxType::Undo => Color::rgb_u8(152, 88, 255),
            ComboboxType::Gravity => Color::rgb_u8(211, 42, 42),
            ComboboxType::Direction { .. } => Color::rgb_u8(255, 182, 193),
            ComboboxType::Lamp { color } => color * 2.5,
        };

        let image = match combobox.box_type {
            ComboboxType::Buff(_) => assets.load("images/box-default-3.png"),
            _ => assets.load("images/box-default-2.png"),
        };

        let mut material = Material::from(color);
        material.texture = Some(image);

        let point_light = match combobox.box_type {
            ComboboxType::Lamp { color } => PointLight2d {
                radius: combobox.world_size() * 3.5,
                color,
            },
            _ => PointLight2d::default(),
        };

        ComboboxBundle {
            combobox: combobox.clone(),
            rigid_body: RigidBody::KinematicPositionBased,
            collider: Collider::cuboid(combobox.world_size() * 0.5, combobox.world_size() * 0.5),
            mesh_bundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Quad::new(Vec2::ONE * combobox.world_size()).into())),
                material: materials.add(material),
                transform: Transform::from_xyz(position.x, position.y, SceneBuilder::BOX_DEPTH)
                    .with_scale(Vec3::ONE * 0.01),
                ..MaterialMesh2dBundle::default()
            },
            locked_axes: LockedAxes::ROTATION_LOCKED,
            combobox_state: ComboboxState::SpawningAnimation(0.0),
            external_impulse: ExternalImpulse::default(),
            read_mass: ReadMassProperties::default(),
            collision_groups: CollisionGroups::new(0, 0),
            velocity: Velocity::default(),
            point_light,
        }
    }
}

impl<'w, 's, 'a, 'b> SceneBuilder<'w, 's, 'a, 'b> {
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
            position * Self::CELL_SIZE,
            self.meshes,
            self.materials,
            self.assets,
        ))
    }
}
