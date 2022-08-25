use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::{
    ExternalImpulse, InteractionGroups, QueryFilter, RapierConfiguration, RapierContext,
    ReadMassProperties, Velocity,
};

use crate::core::direction::MapDirection;
use crate::core::{MoveKeyGroups, PlayerRectState, PLAYER_BIT, PLAYER_FILTER};
use crate::game::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayersSettings {
            player_type: [PlayerType::Color(1), PlayerType::None],
        });
        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(Player::move_player)
                .with_system(Player::jump_player)
                .with_system(Player::update_rect_state),
        );
    }
}

pub const MAX_PLAYERS_NUM: usize = 2;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PlayerType {
    #[default]
    None,
    Color(usize),
}

#[derive(Debug, Clone)]
pub struct PlayersSettings {
    pub player_type: [PlayerType; MAX_PLAYERS_NUM],
}

impl PlayerType {
    pub fn get_image_index(&self) -> usize {
        match *self {
            PlayerType::None => 0,
            PlayerType::Color(color) => color,
        }
    }

    pub fn from_image_index(index: usize) -> PlayerType {
        if index == 0 {
            PlayerType::None
        } else {
            PlayerType::Color(index)
        }
    }

    pub fn get_preview_image(&self) -> String {
        format!("images/robot-preview-{}.png", self.get_image_index())
    }

    pub fn get_states_image(&self) -> String {
        format!("images/robot-states-{}.png", self.get_image_index())
    }

    pub fn get_next(&self, banned: &[PlayerType]) -> PlayerType {
        Self::from_image_index(Self::switch(
            self.get_image_index(),
            &banned
                .iter()
                .map(|v| v.get_image_index())
                .collect::<Vec<_>>(),
            1,
        ))
    }

    pub fn get_prev(&self, banned: &[PlayerType]) -> PlayerType {
        Self::from_image_index(Self::switch(
            self.get_image_index(),
            &banned
                .iter()
                .map(|v| v.get_image_index())
                .collect::<Vec<_>>(),
            -1,
        ))
    }

    fn switch(mut color: usize, banned: &[usize], step: i32) -> usize {
        loop {
            color = (((color as i32 + step) % 7 + 7) % 7) as usize;
            if banned.iter().find(|v| **v == color).is_none() {
                break color;
            }
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub width: f32,
    pub height: f32,
    pub max_speed: f32,
    pub max_acceleration: f32,
    pub jump_impulse: f32,
    pub is_moving: bool,
    pub index: PlayerIndex,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PlayerIndex {
    #[default]
    SinglePlayer,
    TwoPlayers(usize),
}

impl PlayerIndex {
    pub fn get_number_of_players(&self) -> usize {
        match *self {
            PlayerIndex::SinglePlayer => 1,
            PlayerIndex::TwoPlayers(_) => 2,
        }
    }

    pub fn unwrap_index(&self) -> usize {
        match *self {
            PlayerIndex::SinglePlayer => 0,
            PlayerIndex::TwoPlayers(index) => index,
        }
    }
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
            index: PlayerIndex::SinglePlayer,
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
        let gravity_direction = MapDirection::gravity_direction(&*config);

        for (mut impulse, velocity, mass, mut player) in players.iter_mut() {
            let mut target_velocity = 0.0;

            let mut moving = false;

            if keys.any_pressed(player.get_buttons_left(gravity_direction)) {
                target_velocity -= player.max_speed;
                moving = true;
            }

            if keys.any_pressed(player.get_buttons_right(gravity_direction)) {
                target_velocity += player.max_speed;
                moving = true;
            }

            let right = gravity_direction.get_perp().get_vec();
            player.is_moving = moving;

            let delta_velocity = target_velocity - velocity.linvel.dot(right);
            let k = ((delta_velocity.abs() - player.max_speed * 1.0).max(0.0) / player.max_speed)
                .clamp(0.0, 2.0);
            let dv = delta_velocity
                .abs()
                .min(player.max_acceleration * time.delta_seconds() * (1.0 + k));

            impulse.impulse += right * delta_velocity.signum() * dv * mass.0.mass;
        }
    }

    pub fn find_obstacle(
        &self,
        entity: Entity,
        direction: MapDirection,
        gravity_direction: MapDirection,
        position: Vec2,
        context: &RapierContext,
    ) -> Option<(Entity, f32)> {
        const INTERVALS: u32 = 5;

        let (du, dv) = if (direction.get_index() + gravity_direction.get_index()) % 2 == 0 {
            (self.height * 0.5, self.width * 0.5)
        } else {
            (self.width * 0.5, self.height * 0.5)
        };

        let mut res = None;

        for i in 0..INTERVALS {
            let t = (i as f32 / (INTERVALS - 1) as f32) * 1.8 - 0.9;
            let origin =
                position + direction.get_vec() * du + direction.get_perp().get_vec() * t * dv;
            let dir = direction.get_vec();

            let filter = QueryFilter::new()
                .groups(InteractionGroups::new(PLAYER_BIT, PLAYER_FILTER))
                .exclude_collider(entity);

            if let Some((e, d)) = context.cast_ray(origin, dir, 100.0, true, filter) {
                if let Some((_, prev)) = res.clone() {
                    if d < prev {
                        res = Some((e, d))
                    }
                } else {
                    res = Some((e, d));
                }
            }
        }

        res
    }

    pub fn jump_player(
        mut players: Query<(
            Entity,
            &mut ExternalImpulse,
            &Player,
            &Velocity,
            &GlobalTransform,
            &ReadMassProperties,
        )>,
        context: Res<RapierContext>,
        keys: Res<Input<KeyCode>>,
        config: Res<RapierConfiguration>,
    ) {
        let gravity_direction = MapDirection::gravity_direction(&config);

        for (entity, mut ext_impulse, player, velocity, transform, mass) in players.iter_mut() {
            if keys.any_pressed(player.get_buttons_jump(gravity_direction)) {
                let collider_below = player.find_obstacle(
                    entity,
                    gravity_direction,
                    gravity_direction,
                    transform.translation().truncate(),
                    &context,
                );

                if let Some((_, dist)) = collider_below {
                    if dist < 0.1 && velocity.linvel.dot(gravity_direction.get_vec()).abs() < 2.0 {
                        ext_impulse.impulse =
                            -config.gravity.normalize() * player.jump_impulse * mass.0.mass;
                    }
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
        let gravity_direction = MapDirection::gravity_direction(&config);
        for (entity, mut rect_state, player, transform) in query.iter_mut() {
            let prev_rotation = rect_state.current_rotation;
            let prev_state = rect_state.current_state;

            rect_state.current_rotation = gravity_direction.get_index();

            let _legs_origin = transform.translation().xy() + gravity_direction.get_vec() * 10.0;

            if keys.any_pressed(player.get_buttons_right(gravity_direction)) {
                rect_state.current_state = 1;
            } else if keys.any_pressed(player.get_buttons_left(gravity_direction)) {
                rect_state.current_state = 0;
            }

            if rect_state.current_state % 2 == 0 {
                rect_state.current_state = 2;
                if let Some((_, d)) = player.find_obstacle(
                    entity,
                    gravity_direction.get_perp().get_opposite(),
                    gravity_direction,
                    transform.translation().truncate(),
                    &context,
                ) {
                    if d < 1.0 {
                        rect_state.current_state = 0;
                    }
                }
            } else {
                rect_state.current_state = 3;
                if let Some((_, d)) = player.find_obstacle(
                    entity,
                    gravity_direction.get_perp(),
                    gravity_direction,
                    transform.translation().truncate(),
                    &context,
                ) {
                    if d < 1.0 {
                        rect_state.current_state = 1;
                    }
                }
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

    pub fn get_buttons_right(&self, gravity_direction: MapDirection) -> Vec<KeyCode> {
        let mut buttons = vec![];

        if self.index == PlayerIndex::SinglePlayer || self.index == PlayerIndex::TwoPlayers(0) {
            buttons.push(MoveKeyGroups::WASD.get_key(gravity_direction.get_perp()))
        }

        if self.index == PlayerIndex::SinglePlayer || self.index == PlayerIndex::TwoPlayers(1) {
            buttons.push(MoveKeyGroups::Arrows.get_key(gravity_direction.get_perp()))
        }

        buttons
    }

    pub fn get_buttons_left(&self, gravity_direction: MapDirection) -> Vec<KeyCode> {
        self.get_buttons_right(gravity_direction.get_opposite())
    }

    pub fn get_buttons_jump(&self, gravity_direction: MapDirection) -> Vec<KeyCode> {
        let mut buttons = self.get_buttons_right(gravity_direction.get_perp());
        if self.index == PlayerIndex::SinglePlayer {
            buttons.push(KeyCode::Space);
        }
        buttons
    }

    pub fn get_right_direction(&self, gravity_direction: MapDirection) -> Vec2 {
        gravity_direction.get_vec().perp()
    }

    pub fn get_left_direction(&self, gravity_direction: MapDirection) -> Vec2 {
        -gravity_direction.get_vec().perp()
    }
}
