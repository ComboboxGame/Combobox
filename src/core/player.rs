use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::dynamics::CoefficientCombineRule;
use bevy_rapier2d::prelude::{
    Collider, ExternalImpulse, Friction, LockedAxes, QueryFilter, RapierContext,
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
    max_speed: f32,
    max_acceleration: f32,
    jump_impulse: f32,
}

impl Player {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, pos: Vec2) {
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
            .spawn_bundle(TransformBundle {
                local: Transform::from_xyz(pos.x, pos.y, 0.),
                global: GlobalTransform::from_xyz(pos.x, pos.y, 0.),
            })
            .insert(Player {
                max_speed: 200.,
                max_acceleration: 850.0,
                jump_impulse: 2000000.,
            })
            .insert(Collider::cuboid(50., 30.))
            .insert(RigidBody::Dynamic)
            .insert(Velocity::default())
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ExternalImpulse::default())
            .insert(ReadMassProperties::default())
            .insert(Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert_bundle(VisibilityBundle::default())
            .add_child(player_sprite);
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
        mut players: Query<(Entity, &mut ExternalImpulse, &Player, &Transform, &Collider)>,
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
                    let start_point = transform.translation.xy()
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
        mut players: Query<&Transform, With<Player>>,
        mut cameras: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    ) {
        for player in players.iter_mut() {
            for mut camera in cameras.iter_mut() {
                camera.translation = player.translation * 0.08 + camera.translation * 0.92;
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
