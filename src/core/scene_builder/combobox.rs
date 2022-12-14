use bevy::ecs::system::EntityCommands;
use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;
use post_processing::PointLight2d;

use crate::core::{
    material_from_texture_and_emissive, Combobox, ComboboxState, ComboboxType, Material,
    SceneBuilder,
};

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
    pub friction: Friction,
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
                    Color::rgb_u8(103, 245, 124)
                } else if group == 2 {
                    Color::rgb_u8(242, 176, 90)
                } else {
                    Color::rgb_u8(90, 176, 242)
                }
            }
            ComboboxType::Buff(_) => Color::rgb_u8(50, 91, 227),
            ComboboxType::Undo => Color::rgb_u8(141, 50, 227),
            ComboboxType::Gravity => Color::rgb_u8(232, 67, 56),
            ComboboxType::Direction { .. } => Color::rgb_u8(29, 196, 91),
            ComboboxType::Lamp { color } => color * 2.5,
        };

        let overlay = match combobox.box_type {
            ComboboxType::Undo => Some(assets.load("images/overlay-undo.png")),
            ComboboxType::Direction { direction } => {
                if direction.y > 0.5 {
                    Some(assets.load("images/overlay-up.png"))
                } else if direction.y < -0.5 {
                    Some(assets.load("images/overlay-down.png"))
                } else if direction.x < -0.5 {
                    Some(assets.load("images/overlay-left.png"))
                } else {
                    Some(assets.load("images/overlay-right.png"))
                }
            }
            ComboboxType::Buff(buff) => {
                if (buff - 3.0).abs() < 0.1 {
                    Some(assets.load("images/overlay-x3.png"))
                } else if (buff - 9.0).abs() < 0.1 {
                    Some(assets.load("images/overlay-x9.png"))
                } else if (buff - 2.0).abs() < 0.1 {
                    Some(assets.load("images/overlay-x2.png"))
                } else {
                    Some(assets.load("images/overlay-x4.png"))
                }
            }
            ComboboxType::Gravity => Some(assets.load("images/overlay-gravity.png")),
            _ => None,
        };

        let image = assets.load("images/box-default-2.png");

        let mut material = material_from_texture_and_emissive(image, None, overlay);
        material.color = color;

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
            friction: Friction {
                coefficient: 0.9,
                ..default()
            },
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
