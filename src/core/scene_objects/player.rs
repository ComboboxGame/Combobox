use crate::core::{collision_groups, Combobox, PlayerRectState, GRAVITY_FORCE};
use crate::states::LevelState;
use crate::utils::SceneDirection;
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayersSettings {
            player_type: [PlayerType::Color(1), PlayerType::None],
        });
        app.add_system_set(
            SystemSet::on_update(LevelState::Level)
                .with_system(move_player)
                .with_system(jump_player)
                .with_system(update_rect_state)
                .with_system(grab),
        );
    }
}

pub enum MoveKeyGroups {
    WASD,
    Arrows,
}

impl MoveKeyGroups {
    pub fn get_key(&self, direction: SceneDirection) -> KeyCode {
        let keys = match *self {
            MoveKeyGroups::WASD => [KeyCode::S, KeyCode::D, KeyCode::W, KeyCode::A],
            MoveKeyGroups::Arrows => [KeyCode::Down, KeyCode::Right, KeyCode::Up, KeyCode::Left],
        };
        keys[direction.get_index() as usize]
    }

    pub fn get_grab_key(&self) -> KeyCode {
        match *self {
            MoveKeyGroups::WASD => KeyCode::LControl,
            MoveKeyGroups::Arrows => KeyCode::RControl,
        }
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
    pub jump_height: f32,
    pub is_moving: bool,
    pub ungrab_time: f32,

    // Weird concept of player index.
    // Contains index of player in current set of players
    // while also containing info about current number of players.
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
            width: 54.0,
            height: 90.0,
            max_speed: 160.,
            max_acceleration: 1800.0,
            jump_height: 110.,
            is_moving: false,
            ungrab_time: 0.0,
            index: PlayerIndex::SinglePlayer,
        }
    }
}

impl Player {
    pub fn find_obstacle(
        &self,
        entity: Entity,
        direction: SceneDirection,
        gravity_direction: SceneDirection,
        position: Vec2,
        context: &RapierContext,
        range: [f32; 2],
    ) -> Option<(Entity, f32)> {
        const INTERVALS: u32 = 5;

        let (du, dv) = if (direction.get_index() + gravity_direction.get_index()) % 2 == 0 {
            (self.height * 0.5, self.width * 0.5)
        } else {
            (self.width * 0.5, self.height * 0.5)
        };

        let mut res = None;

        for i in 0..INTERVALS {
            let t = ((i as f32 / (INTERVALS - 1) as f32) * (range[1] - range[0]) + range[0]) * 2.0
                - 1.0;
            let origin =
                position + direction.get_vec() * du + direction.get_perp().get_vec() * t * dv;
            let dir = direction.get_vec();

            let filter = QueryFilter::new()
                .groups(collision_groups::PLAYER_I)
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

    pub fn get_buttons_right(&self, gravity_direction: SceneDirection) -> Vec<KeyCode> {
        let mut buttons = vec![];

        if self.index == PlayerIndex::SinglePlayer || self.index == PlayerIndex::TwoPlayers(0) {
            buttons.push(MoveKeyGroups::WASD.get_key(gravity_direction.get_perp()))
        }

        if self.index == PlayerIndex::SinglePlayer || self.index == PlayerIndex::TwoPlayers(1) {
            buttons.push(MoveKeyGroups::Arrows.get_key(gravity_direction.get_perp()))
        }

        buttons
    }

    pub fn get_buttons_grab(&self) -> Vec<KeyCode> {
        let mut buttons = vec![];

        if self.index == PlayerIndex::SinglePlayer || self.index == PlayerIndex::TwoPlayers(0) {
            buttons.push(MoveKeyGroups::WASD.get_grab_key())
        }

        if self.index == PlayerIndex::SinglePlayer || self.index == PlayerIndex::TwoPlayers(1) {
            buttons.push(MoveKeyGroups::Arrows.get_grab_key())
        }

        buttons
    }

    pub fn get_buttons_left(&self, gravity_direction: SceneDirection) -> Vec<KeyCode> {
        self.get_buttons_right(gravity_direction.get_opposite())
    }

    pub fn get_buttons_jump(&self, gravity_direction: SceneDirection) -> Vec<KeyCode> {
        let mut buttons = self.get_buttons_right(gravity_direction.get_perp());
        if self.index == PlayerIndex::SinglePlayer {
            buttons.push(KeyCode::Space);
        }
        buttons
    }

    pub fn get_right_direction(&self, gravity_direction: SceneDirection) -> Vec2 {
        gravity_direction.get_vec().perp()
    }

    pub fn get_left_direction(&self, gravity_direction: SceneDirection) -> Vec2 {
        -gravity_direction.get_vec().perp()
    }
}

fn grab(
    mut commands: Commands,
    mut players: Query<(Entity, &GlobalTransform, &mut Player, Option<&ImpulseJoint>)>,
    boxes: Query<(&GlobalTransform, &Combobox), With<Combobox>>,
    context: Res<RapierContext>,
    keys: Res<Input<KeyCode>>,
    config: ResMut<RapierConfiguration>,
    time: Res<Time>,
) {
    let gravity_direction = SceneDirection::from_gravity_direction(&config);

    for (entity, transform, mut player, maybe_joint) in players.iter_mut() {
        if maybe_joint.is_none() {
            player.ungrab_time += time.delta_seconds();
        } else {
            player.ungrab_time = 0.0;
        }

        if maybe_joint.is_none()
            && keys.any_pressed(player.get_buttons_grab())
            && player.ungrab_time > 0.2
        {
            let mut dir = gravity_direction.get_perp();

            for _ in 0..2 {
                let collider = player.find_obstacle(
                    entity,
                    dir,
                    gravity_direction,
                    transform.translation().truncate(),
                    &context,
                    [0.45, 0.55],
                );

                if let Some((e, d)) = collider {
                    if d < 5.0 {
                        if let Ok((t, c)) = boxes.get(e) {
                            let mut offset = (t.translation() - transform.translation()).truncate();

                            let dist = gravity_direction.get_perp().get_vec().dot(offset).abs();
                            let target_dist = (player.width + c.world_size()) * 0.5;

                            offset += (target_dist - dist) * gravity_direction.get_perp().get_vec();

                            offset -= gravity_direction.get_vec() * 5.0;

                            let joint = FixedJointBuilder::new().local_anchor2(offset);
                            commands.entity(entity).insert(ImpulseJoint::new(e, joint));
                            break;
                        }
                    }
                }

                dir = dir.get_opposite();
            }
        }

        if keys.any_just_released(player.get_buttons_grab()) {
            commands.entity(entity).remove::<ImpulseJoint>();
        }
    }
}

fn move_player(
    mut players: Query<(
        &mut ExternalImpulse,
        &Velocity,
        &ReadMassProperties,
        &mut Player,
        Option<&ImpulseJoint>,
    )>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    config: ResMut<RapierConfiguration>,
) {
    let gravity_direction = SceneDirection::from_gravity_direction(&*config);

    for (mut impulse, velocity, mass, mut player, maybe_joint) in players.iter_mut() {
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

        let max_acceleration = player.max_acceleration
            * if maybe_joint.is_none() || target_velocity.abs() < 0.1 {
                1.0
            } else {
                0.3
            };

        let dv = delta_velocity
            .abs()
            .min(max_acceleration * time.delta_seconds() * (1.0 + k));

        impulse.impulse += right * delta_velocity.signum() * dv * mass.0.mass;
    }
}

fn jump_player(
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
    let gravity_direction = SceneDirection::from_gravity_direction(&config);

    for (entity, mut ext_impulse, player, velocity, transform, mass) in players.iter_mut() {
        if keys.any_pressed(player.get_buttons_jump(gravity_direction)) {
            let collider_below = player.find_obstacle(
                entity,
                gravity_direction,
                gravity_direction,
                transform.translation().truncate(),
                &context,
                [0.05, 0.95],
            );

            if let Some((_, dist)) = collider_below {
                let proj = velocity.linvel.dot(gravity_direction.get_vec());
                let jump_velocity = (2.0 * player.jump_height * GRAVITY_FORCE).sqrt();

                let delta = jump_velocity + proj;

                if dist < 0.1 {
                    ext_impulse.impulse = -config.gravity.normalize() * delta * mass.0.mass;
                }
            }
        }
    }
}

fn update_rect_state(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut PlayerRectState,
        &Player,
        &GlobalTransform,
        Option<&ImpulseJoint>,
    )>,
    keys: Res<Input<KeyCode>>,
    config: Res<RapierConfiguration>,
    context: Res<RapierContext>,
) {
    let gravity_direction = SceneDirection::from_gravity_direction(&config);
    for (entity, mut rect_state, player, transform, maybe_joint) in query.iter_mut() {
        let prev_rotation = rect_state.current_rotation;
        let prev_state = rect_state.current_state;

        rect_state.current_rotation = gravity_direction.get_index();

        let _legs_origin = transform.translation().xy() + gravity_direction.get_vec() * 10.0;

        if keys.any_pressed(player.get_buttons_right(gravity_direction)) {
            rect_state.current_state = 1;
        } else if keys.any_pressed(player.get_buttons_left(gravity_direction)) {
            rect_state.current_state = 0;
        }

        if rect_state.current_state < 4 {
            if rect_state.current_state % 2 == 0 {
                rect_state.current_state = 2;
                if let Some((_, d)) = player.find_obstacle(
                    entity,
                    gravity_direction.get_perp().get_opposite(),
                    gravity_direction,
                    transform.translation().truncate(),
                    &context,
                    [0.05, 0.95],
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
                    [0.05, 0.95],
                ) {
                    if d < 1.0 {
                        rect_state.current_state = 1;
                    }
                }
            }
        }

        if let Some(joint) = maybe_joint {
            let anchor = joint.data.raw.local_anchor2();
            let anchor = Vec2::new(anchor.x, anchor.y);
            let right = gravity_direction.get_perp().get_vec();
            if anchor.dot(right) < 0.0 {
                rect_state.current_state = 0;
            } else {
                rect_state.current_state = 1;
            }
        }

        if prev_rotation != rect_state.current_rotation || prev_state != rect_state.current_state {
            commands
                .entity(entity)
                .insert_bundle(rect_state.get_current_bundle());
        }
    }
}
