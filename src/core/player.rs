use crate::core::{MapBoundaries, PLAYER_BIT, PLAYER_FILTER};

use bevy::render::camera::RenderTarget;
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::dynamics::CoefficientCombineRule;
use bevy_rapier2d::prelude::{
    Collider, CollisionGroups, ExternalImpulse, Friction, LockedAxes, QueryFilter, RapierContext,
    ReadMassProperties, RigidBody, Velocity,
};

use crate::game::GameState;

#[derive(Component)]
pub struct PlayerSprite {
    is_turned_left: bool,
}

impl Default for PlayerSprite {
    fn default() -> Self {
        Self {
            is_turned_left: true,
        }
    }
}

impl PlayerSprite {
    fn turn_player(
        keys: Res<Input<KeyCode>>,
        mut query: Query<(&mut Transform, &mut PlayerSprite)>,
    ) {
        for (mut transform, mut sprite) in query.iter_mut() {
            if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
                if !sprite.is_turned_left {
                    transform.scale *= Vec3::new(-1., 1., 1.);
                    sprite.is_turned_left = true;
                }
            } else if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
                if sprite.is_turned_left {
                    transform.scale *= Vec3::new(-1., 1., 1.);
                    sprite.is_turned_left = false;
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub max_speed: f32,
    pub max_acceleration: f32,
    pub jump_impulse: f32,
    pub id: u32,
}

impl Player {
    pub fn spawn(commands: &mut Commands, asset_server: &AssetServer, id: u32) -> Entity {
        let player_sprite = commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("img/cat.png"),
                transform: Transform {
                    scale: Vec3::new(0.25, 0.25, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(PlayerSprite::default())
            .id();

        commands
            .spawn_bundle(TransformBundle::default())
            .insert(Player {
                max_speed: 200.,
                max_acceleration: 850.0,
                jump_impulse: 2000000.,
                id,
            })
            .insert(Collider::cuboid(50., 30.))
            .insert(RigidBody::Dynamic)
            .insert(Velocity::default())
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ExternalImpulse::default())
            .insert(ReadMassProperties::default())
            .insert(CollisionGroups::new(PLAYER_BIT, PLAYER_FILTER))
            .insert(Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert_bundle(VisibilityBundle::default())
            .add_child(player_sprite)
            .id()
    }

    pub fn move_player(
        mut players: Query<(
            &mut ExternalImpulse,
            &Velocity,
            &ReadMassProperties,
            &Player,
        )>,
        keys: Res<Input<KeyCode>>,
        time: Res<Time>,
    ) {
        for (mut impulse, velocity, mass, player) in players.iter_mut() {
            let mut target_velocity = 0.0;

            if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
                target_velocity -= player.max_speed;
            }
            if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
                target_velocity += player.max_speed;
            }

            let delta_velocity = target_velocity - velocity.linvel.x;
            let k = ((delta_velocity.abs() - player.max_speed * 1.0).max(0.0) / player.max_speed)
                .clamp(0.0, 2.0);
            let dv = delta_velocity
                .abs()
                .min(player.max_acceleration * time.delta_seconds() * (1.0 + k));
            impulse.impulse += Vec2::X * delta_velocity.signum() * dv * mass.0.mass;
        }
    }

    pub fn jump_player(
        mut players: Query<(
            Entity,
            &mut ExternalImpulse,
            &Player,
            &GlobalTransform,
            &Collider,
        )>,
        rapier_context: Res<RapierContext>,
        keys: Res<Input<KeyCode>>,
    ) {
        let hits_floor = |entity: Entity, pos: Vec2| -> bool {
            let dir = Vec2::new(0., -1.);
            rapier_context
                .cast_ray(
                    pos,
                    dir,
                    4.,
                    true,
                    QueryFilter::new().exclude_collider(entity),
                )
                .is_some()
        };

        for (entity, mut ext_impulse, player, transform, collider) in players.iter_mut() {
            if keys.just_pressed(KeyCode::Space) {
                let collider = collider.as_cuboid().unwrap();

                let mut can_jump = false;
                for i in 0..4 {
                    let start_point = transform.translation().xy()
                        - Vec2::new(
                            collider.half_extents().x * (1. - 2. / (i as f32)),
                            collider.half_extents().y,
                        );

                    if hits_floor(entity, start_point) {
                        can_jump = true;
                        break;
                    }
                }

                if can_jump {
                    ext_impulse.impulse = Vec2::new(0., player.jump_impulse);
                }
            }
        }
    }

    pub fn camera_follow(
        mut players: Query<&GlobalTransform, With<Player>>,
        mut cameras: Query<(&mut Transform, &Camera), (With<Camera2d>, Without<Player>)>,
        boundaries: Res<MapBoundaries>,
        windows: Res<Windows>,
        images: Res<Assets<Image>>,
    ) {
        for player in players.iter_mut() {
            for (mut transform, camera) in cameras.iter_mut() {
                let mut pos: Vec3 = player.translation() * 0.08 + transform.translation * 0.92;

                if let Some(rect) = boundaries.rect {
                    let _viewport = match &camera.target {
                        RenderTarget::Window(window_id) => {
                            windows.get(*window_id).and_then(|window| {
                                Some(UVec2::new(
                                    window.physical_width(),
                                    window.physical_height(),
                                ))
                            })
                        }
                        RenderTarget::Image(image_handle) => {
                            images.get(&image_handle).map(|image| {
                                UVec2::new(
                                    image.texture_descriptor.size.width,
                                    image.texture_descriptor.size.height,
                                )
                            })
                        }
                    }
                    .unwrap();

                    // matrix for undoing the projection and camera transform
                    let ndc_to_world =
                        transform.compute_matrix() * camera.projection_matrix().inverse();

                    // use it to convert ndc to world-space coordinates
                    let world_pos = ndc_to_world.project_point3(Vec2::ONE.extend(-1.0));

                    // reduce it to a 2D value
                    let world_pos: Vec2 = (world_pos - transform.translation).truncate();

                    pos.x = pos
                        .x
                        .clamp(rect.min.x + world_pos.x, rect.max.x - world_pos.x);
                    pos.y = pos
                        .y
                        .clamp(rect.min.y + world_pos.y, rect.max.y - world_pos.y);
                }

                transform.translation = pos;
            }
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(Player::move_player));
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(Player::jump_player));
        app.add_system_set(
            SystemSet::on_update(GameState::Game).with_system(Player::camera_follow),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Game).with_system(PlayerSprite::turn_player),
        );
    }
}
