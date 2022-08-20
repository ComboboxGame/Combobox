use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::{Collider, ExternalImpulse, LockedAxes, RigidBody, Velocity};

#[derive(Component)]
pub struct Player {
    moving_speed: f32,
    moving_decay: f32,
    jump_impulse: f32,
}

impl Player {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, pos: Vec2) {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("img/cat.png"),
                transform: Transform {
                    translation: Vec3::new(pos.x, pos.y, 0.),
                    scale: Vec3::new(0.25, 0.25, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player {
                moving_speed: 80.,
                moving_decay: 0.5,
                jump_impulse: 500000.,
            })
            .insert(Collider::cuboid(200., 123.))
            .insert(RigidBody::Dynamic)
            .insert(Velocity::default())
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ExternalImpulse::default());
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
        mut players: Query<(&mut ExternalImpulse, &Player)>,
        keys: Res<Input<KeyCode>>,
    ) {
        for (mut ext_impulse, player) in players.iter_mut() {
            if keys.just_pressed(KeyCode::Space) {
                ext_impulse.impulse = Vec2::new(0., player.jump_impulse);
            }
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Player::move_player);
        app.add_system(Player::jump_player);
    }
}
