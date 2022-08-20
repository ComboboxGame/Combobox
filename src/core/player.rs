use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::{
    Collider, ExternalImpulse, LockedAxes, QueryFilter, RapierContext, RigidBody, Velocity,
};

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
    moving_speed: f32,
    moving_decay: f32,
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
                moving_speed: 150.,
                moving_decay: 0.5,
                jump_impulse: 2000000.,
            })
            .insert(Collider::cuboid(50., 30.))
            .insert(RigidBody::Dynamic)
            .insert(Velocity::default())
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ExternalImpulse::default())
            .insert_bundle(VisibilityBundle::default())
            .add_child(player_sprite);
    }

    pub fn move_player(mut players: Query<(&mut Velocity, &Player)>, keys: Res<Input<KeyCode>>) {
        for (mut velocity, player) in players.iter_mut() {
            let vel = Vec3::X.xy() * player.moving_speed;
            if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
                velocity.linvel -= vel;
            } else if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
                velocity.linvel += vel;
            }

            velocity.linvel.x *= player.moving_decay;
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
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Player::move_player);
        app.add_system(Player::jump_player);
        app.add_system(PlayerSprite::turn_player);
    }
}
