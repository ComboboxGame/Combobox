use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::sprite::Mesh2dHandle;
use bevy_rapier2d::dynamics::{CoefficientCombineRule, Velocity};
use bevy_rapier2d::prelude::*;
use post_processing::PointLight2d;

use crate::core::{
    collision_groups, material_from_texture_and_emissive, Material, Player, PlayerIndex,
    PlayerType, SceneBuilder,
};

fn create_quad(half_size: Vec2, state: u32, num_states: u32, rotation: u32) -> Mesh {
    let extent_x = half_size.x;
    let extent_y = half_size.y;

    let vertices = [
        ([-extent_x, -extent_y, 0.0], [0.0, 0.0, 1.0], [0.0f32, 1.0]),
        ([-extent_x, extent_y, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
        ([extent_x, extent_y, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
        ([extent_x, -extent_y, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
    ];

    let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

    let positions: Vec<_> = vertices
        .iter()
        .map(|(p, _, _)| *p)
        .map(|[mut x, mut y, z]| {
            for _ in 0..rotation {
                [x, y] = [-y, x]
            }
            [x, y, z]
        })
        .collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices
        .iter()
        .map(|(_, _, uv)| *uv)
        .map(|[u, v]| {
            [
                (u.clamp(0.01, 0.99) + state as f32) / (num_states as f32),
                v,
            ]
        })
        .collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

#[derive(Component, Default)]
pub struct PlayerRectState {
    pub current_state: u32,
    pub current_rotation: u32,
    pub states: Vec<[(Collider, Mesh2dHandle); 4]>,
}

impl PlayerRectState {
    pub fn get_current_bundle(&self) -> (Collider, Mesh2dHandle) {
        self.states[self.current_state as usize][self.current_rotation as usize].clone()
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    collider: Collider,
    mesh: Mesh2dHandle,
    player_states: PlayerRectState,
    rigid_body: RigidBody,
    velocity: Velocity,
    axes: LockedAxes,
    impulse: ExternalImpulse,
    mass: ReadMassProperties,
    collision_groups: CollisionGroups,
    friction: Friction,
    material: Handle<Material>,
    #[bundle]
    visibility: VisibilityBundle,
    #[bundle]
    transform: TransformBundle,
    point_light: PointLight2d,
    density: ColliderMassProperties,
}

impl PlayerBundle {
    pub fn new(
        index: PlayerIndex,
        player_type: PlayerType,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<Material>,
        asset_server: &AssetServer,
    ) -> Self {
        let mut player_states = PlayerRectState::default();
        let material = materials.add(material_from_texture_and_emissive(
            asset_server.load(player_type.get_states_image().as_str()),
            Some(asset_server.load("images/robot-states-emissive.png")),
            None,
        ));

        let player = Player { index, ..default() };

        for state in 0..5 {
            player_states.states.push([0, 1, 2, 3].map(|rot| {
                let mesh = create_quad(Vec2::new(player.width, player.height) * 0.5, state, 5, rot);
                let collider = if rot % 2 == 0 {
                    Collider::cuboid(player.width * 0.5 * 0.9, player.height * 0.5)
                } else {
                    Collider::cuboid(player.height * 0.5, player.width * 0.5 * 0.9)
                };
                (collider, Mesh2dHandle(meshes.add(mesh)))
            }))
        }

        player_states.current_state = 2;
        PlayerBundle {
            player,
            collider: player_states.get_current_bundle().0,
            mesh: player_states.get_current_bundle().1,
            player_states,
            rigid_body: RigidBody::Dynamic,
            axes: LockedAxes::ROTATION_LOCKED,
            collision_groups: collision_groups::PLAYER,
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            point_light: PointLight2d {
                radius: 150.0,
                color: Color::WHITE,
            },
            transform: TransformBundle::from_transform(Transform::from_xyz(
                0.0,
                0.0,
                SceneBuilder::PLAYER_DEPTH,
            )),
            material,
            density: ColliderMassProperties::Density(2.0),
            ..default()
        }
    }
}
