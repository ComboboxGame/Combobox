use crate::core::{MapBoundaries, PlayerRectState};
use crate::core::gravity::GravityDirection;
use bevy::render::camera::RenderTarget;
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::{
    Collider, ExternalImpulse, QueryFilter, RapierConfiguration, RapierContext, ReadMassProperties,
    Velocity,
};

use crate::game::GameState;

#[derive(Component)]
pub struct Player {
    pub width: f32,
    pub height: f32,
    pub max_speed: f32,
    pub max_acceleration: f32,
    pub jump_impulse: f32,
    pub is_moving: bool,
    pub id: u32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            width: 60.0,
            height: 100.0,
            max_speed: 200.,
            max_acceleration: 1850.0,
            jump_impulse: 600.,
            is_moving: false,
            id: 0,
        }
    }
}

impl Player {
    pub fn move_player(
        mut players: Query<(
            &mut ExternalImpulse,
            &Velocity,
            &ReadMassProperties,
            &mut Player,
        )>,
        keys: Res<Input<KeyCode>>,
        time: Res<Time>,
        config: ResMut<RapierConfiguration>,
    ) {
        for (mut impulse, velocity, mass, mut player) in players.iter_mut() {
            let mut target_velocity = 0.0;

            let mut moving = false;
            if ((keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left)) && config.gravity.y != 0.)
                || ((keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down))
                    && config.gravity.x != 0.)
            {
                target_velocity -= player.max_speed;
                moving = true;
            }
            if ((keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right))
                && config.gravity.y != 0.)
                || ((keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up))
                    && config.gravity.x != 0.)
            {
                target_velocity += player.max_speed;
                moving = true;
            }

            player.is_moving = moving;

            let right = (Quat::from_rotation_arc_2d(Vec2::NEG_Y, Vec2::X)
                * config.gravity.abs().normalize().extend(0.0))
            .truncate()
                * Vec2::new(-1.0, 1.0);

            let delta_velocity = target_velocity - velocity.linvel.dot(right);
            let k = ((delta_velocity.abs() - player.max_speed * 1.0).max(0.0) / player.max_speed)
                .clamp(0.0, 2.0);
            let dv = delta_velocity
                .abs()
                .min(player.max_acceleration * time.delta_seconds() * (1.0 + k));

            impulse.impulse += right * delta_velocity.signum() * dv * mass.0.mass;
        }
    }

    pub fn jump_player(
        mut players: Query<(
            Entity,
            &mut ExternalImpulse,
            &Player,
            &Velocity,
            &GlobalTransform,
            &Collider,
            &ReadMassProperties,
        )>,
        rapier_context: Res<RapierContext>,
        keys: Res<Input<KeyCode>>,
        config: Res<RapierConfiguration>,
    ) {
        let gravity_direction = GravityDirection::get_from_config(&config);

        let hits_floor = |entity: Entity, pos: Vec2| -> bool {
            let dir = config.gravity.normalize();
            rapier_context
                .cast_ray(
                    pos,
                    dir,
                    1.,
                    true,
                    QueryFilter::new().exclude_collider(entity),
                )
                .is_some()
        };

        for (entity, mut ext_impulse, player, velocity, transform, collider, mass) in
            players.iter_mut()
        {
            if keys.any_pressed(player.get_buttons_jump(gravity_direction)) {
                let collider = collider.as_cuboid().unwrap();

                let mut can_jump = false;
                let intervals = 4;
                for i in 0..intervals {
                    let bottom;

                    if config.gravity.y != 0. {
                        bottom = Vec2::new(
                            collider.half_extents().x
                                * (0.9 - 1.8 * (i as f32) / ((intervals - 1) as f32)),
                            collider.half_extents().y * -config.gravity.y.signum(),
                        );
                    } else {
                        bottom = Vec2::new(
                            collider.half_extents().x * -config.gravity.x.signum(),
                            collider.half_extents().y
                                * (0.9 - 1.8 * (i as f32) / ((intervals - 1) as f32)),
                        );
                    }

                    let start_point = transform.translation().xy() - bottom;

                    if hits_floor(entity, start_point)
                        && velocity.linvel.dot(gravity_direction.get_vec()).abs() < 2.0
                    {
                        can_jump = true;
                        break;
                    }
                }

                if can_jump {
                    ext_impulse.impulse =
                        -config.gravity.normalize() * player.jump_impulse * mass.0.mass;
                }
            }
        }
    }

    fn update_rect_state(
        mut commands: Commands,
        mut query: Query<(Entity, &mut PlayerRectState, &Player, &GlobalTransform)>,
        keys: Res<Input<KeyCode>>,
        config: Res<RapierConfiguration>,
        context: Res<RapierContext>,
    ) {
        let gravity_direction = GravityDirection::get_from_config(&config);
        for (entity, mut rect_state, player, transform) in query.iter_mut() {
            let prev_rotation = rect_state.current_rotation;
            let prev_state = rect_state.current_state;

            rect_state.current_rotation = gravity_direction.get_index();

            let legs_origin = transform.translation().xy() + gravity_direction.get_vec() * 10.0;

            if keys.any_pressed(player.get_buttons_right(gravity_direction)) {
                if let Some(_) = context.cast_ray(
                    legs_origin,
                    player.get_right_direction(gravity_direction),
                    player.width * 0.6,
                    true,
                    QueryFilter::new().exclude_collider(entity),
                ) {
                    rect_state.current_state = 1;
                } else {
                    rect_state.current_state = 3;
                }
            } else if keys.any_pressed(player.get_buttons_left(gravity_direction)) {
                if let Some(_) = context.cast_ray(
                    legs_origin,
                    player.get_left_direction(gravity_direction),
                    player.width * 0.6,
                    true,
                    QueryFilter::new().exclude_collider(entity),
                ) {
                    rect_state.current_state = 0;
                } else {
                    rect_state.current_state = 2;
                }
            } else {
                // rect_state.current_state = 4;
            }

            if prev_rotation != rect_state.current_rotation
                || prev_state != rect_state.current_state
            {
                commands
                    .entity(entity)
                    .insert_bundle(rect_state.get_current_bundle());
            }
        }
    }

    pub fn get_buttons_right(&self, gravity_direction: GravityDirection) -> Vec<KeyCode> {
        match gravity_direction {
            GravityDirection::Down => vec![KeyCode::D, KeyCode::Right],
            GravityDirection::Right => vec![KeyCode::W, KeyCode::Up],
            GravityDirection::Up => vec![KeyCode::A, KeyCode::Left],
            GravityDirection::Left => vec![KeyCode::S, KeyCode::Down],
        }
    }

    pub fn get_buttons_left(&self, gravity_direction: GravityDirection) -> Vec<KeyCode> {
        match gravity_direction {
            GravityDirection::Down => vec![KeyCode::A, KeyCode::Left],
            GravityDirection::Right => vec![KeyCode::S, KeyCode::Down],
            GravityDirection::Up => vec![KeyCode::D, KeyCode::Right],
            GravityDirection::Left => vec![KeyCode::W, KeyCode::Up],
        }
    }

    pub fn get_buttons_jump(&self, gravity_direction: GravityDirection) -> Vec<KeyCode> {
        match gravity_direction {
            GravityDirection::Down => vec![KeyCode::W, KeyCode::Up, KeyCode::Space],
            GravityDirection::Right => vec![KeyCode::A, KeyCode::Left, KeyCode::Space],
            GravityDirection::Up => vec![KeyCode::S, KeyCode::Down, KeyCode::Space],
            GravityDirection::Left => vec![KeyCode::D, KeyCode::Right, KeyCode::Space],
        }
    }

    pub fn get_right_direction(&self, gravity_direction: GravityDirection) -> Vec2 {
        gravity_direction.get_vec().perp()
    }

    pub fn get_left_direction(&self, gravity_direction: GravityDirection) -> Vec2 {
        -gravity_direction.get_vec().perp()
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
            SystemSet::on_update(GameState::Game).with_system(Player::update_rect_state),
        );
    }
}
