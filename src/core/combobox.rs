use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier2d::prelude::*;

use bevy_rapier2d::rapier::prelude::QueryFilterFlags;

use crate::core::GRAVITY;
use crate::core::{ComboboxBundle, Material, COMBOBOX_BIT, COMBOBOX_FILTER};
use crate::game::GameState;

#[derive(Clone, Debug)]
pub enum ComboboxType {
    Standard { group: u32 },
    Buf,
    Undo,
    Direction { direction: Vec2 },
    Gravity,
    Lamp { color: Color },
}

#[derive(Clone, Debug, PartialEq, Component)]
pub enum ComboboxState {
    Normal,
    SpawningAnimation(f32),
    DespawningAnimation(f32),
    Despawned,
}

impl ComboboxState {
    pub const SPAWN_TIME: f32 = 0.25;
    pub const DESPAWN_TIME: f32 = 0.25;

    fn evaluate_bezier(a: Vec2, b: Vec2, t: f32) -> f32 {
        a.x * (1.0 - t).powf(3.0)
            + 3.0 * a.y * (1.0 - t).powf(2.0) * t
            + 3.0 * b.x * (1.0 - t) * t * t
            + b.y * t * t * t
    }

    pub fn get_scale(&self) -> f32 {
        match *self {
            ComboboxState::Normal => 1.0,
            ComboboxState::SpawningAnimation(time) => {
                let a = Vec2::new(0.0, 1.0);
                let b = Vec2::new(1.35, 1.0);
                let t = (time / Self::SPAWN_TIME).clamp(0.01, 1.0);
                Self::evaluate_bezier(a, b, t)
            }
            ComboboxState::DespawningAnimation(time) => {
                let a = Vec2::new(0.0, 1.0);
                let b = Vec2::new(1.35, 1.0);
                let t = 1.0 - (time / Self::SPAWN_TIME).clamp(0.01, 1.0);
                Self::evaluate_bezier(a, b, t)
            }
            ComboboxState::Despawned => 0.01,
        }
    }

    pub fn get_scale_ahead(&self, ahead: f32) -> f32 {
        match *self {
            ComboboxState::Normal => 1.0,
            ComboboxState::SpawningAnimation(time) => {
                let a = Vec2::new(0.0, 1.0);
                let b = Vec2::new(1.00, 1.0);
                let t = ((time + ahead) / Self::SPAWN_TIME).clamp(0.01, 1.0);
                Self::evaluate_bezier(a, b, t)
            }
            ComboboxState::DespawningAnimation(time) => {
                let a = Vec2::new(0.0, 1.0);
                let b = Vec2::new(1.00, 1.0);
                let t = 1.0 - ((time + ahead) / Self::SPAWN_TIME).clamp(0.01, 1.0);
                Self::evaluate_bezier(a, b, t)
            }
            ComboboxState::Despawned => 0.01,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Combobox {
    pub weight: f32,
    pub box_type: ComboboxType,
    pub combined_from: Vec<(Combobox, Vec2)>,
    pub local_gravity: Option<Vec2>,
}

impl Combobox {
    pub const DEFAULT_SIZE: f32 = 50.0;

    pub fn new(weight: f32, box_type: ComboboxType) -> Combobox {
        Self {
            weight,
            box_type,
            combined_from: vec![],
            local_gravity: None,
        }
    }

    pub fn world_size(&self) -> f32 {
        self.weight.sqrt() * Self::DEFAULT_SIZE
    }

    pub fn merge(
        first: &Combobox,
        first_pos: Vec2,
        second: &Combobox,
        second_pos: Vec2,
    ) -> Option<Vec<(Combobox, Vec2)>> {
        let center = (first_pos * first.world_size() + second_pos * second.world_size())
            / (first.world_size() + second.world_size());
        let first_offset = first_pos - center;
        let second_offset = second_pos - center;

        match (&first.box_type, &second.box_type) {
            (
                ComboboxType::Standard { group: group1 },
                ComboboxType::Standard { group: group2 },
            ) => {
                if group1 != group2 {
                    return None;
                }

                let mut gravity = None;
                if let Some(first_gravity) = first.local_gravity {
                    if let Some(second_gravity) = second.local_gravity {
                        if first_gravity == second_gravity {
                            gravity = Some(first_gravity.clone());
                        }
                    }
                }

                let big_box = Combobox {
                    weight: first.weight + second.weight,
                    box_type: first.box_type.clone(),
                    combined_from: vec![
                        (first.clone(), first_offset),
                        (second.clone(), second_offset),
                    ],
                    local_gravity: gravity,
                };

                return Some(vec![(big_box, center)]);
            }
            (ComboboxType::Buf, ComboboxType::Standard { .. }) => {
                let buffed_box = Combobox {
                    weight: second.weight * 2.,
                    box_type: second.box_type.clone(),
                    combined_from: vec![
                        (first.clone(), first_offset),
                        (second.clone(), second_offset),
                    ],
                    local_gravity: second.local_gravity,
                };
                return Some(vec![(buffed_box, second_pos)]);
            }
            (ComboboxType::Direction { direction }, ComboboxType::Standard { .. }) => {
                let direction_box = Combobox {
                    weight: second.weight,
                    box_type: second.box_type.clone(),
                    combined_from: vec![
                        (first.clone(), first_offset),
                        (second.clone(), second_offset),
                    ],
                    local_gravity: Some(direction.clone()),
                };

                return Some(vec![(direction_box, second_pos)]);
            }
            (ComboboxType::Gravity, ComboboxType::Direction { direction }) => {
                let gravity_box = Combobox {
                    weight: first.weight,
                    box_type: first.box_type.clone(),
                    combined_from: vec![
                        (first.clone(), first_offset),
                        (second.clone(), second_offset),
                    ],
                    local_gravity: Some(direction.clone()),
                };

                return Some(vec![(gravity_box, first_pos)]);
            }
            (ComboboxType::Undo, _) => {
                if second.combined_from.len() == 0 {
                    return None;
                }

                return Some(
                    second
                        .combined_from
                        .iter()
                        .map(|(c, v)| (c.clone(), *v * 1.9 + second_pos))
                        .collect(),
                );
            }
            (_, ComboboxType::Undo)
            | (ComboboxType::Standard { .. }, ComboboxType::Buf)
            | (ComboboxType::Standard { .. }, ComboboxType::Direction { .. })
            | (ComboboxType::Direction { .. }, ComboboxType::Gravity) => {
                Self::merge(second, second_pos, first, first_pos)
            }
            (_, _) => None,
        }
    }
}

pub struct ComboboxPlugin;

impl Plugin for ComboboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(merge)
                .with_system(animation)
                .with_system(pushback)
                .with_system(despawn)
                .with_system(change_direction)
                .with_system(change_gravity),
        );
    }
}

fn animation(
    mut commands: Commands,
    mut comboboxes: Query<(Entity, &mut ComboboxState, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut combobox_state, mut transform) in comboboxes.iter_mut() {
        let mut new_state = None;

        if let ComboboxState::SpawningAnimation(animation_time) = &mut *combobox_state {
            *animation_time += time.delta_seconds();

            if *animation_time >= ComboboxState::SPAWN_TIME {
                new_state = Some(ComboboxState::Normal);
                commands.entity(entity).insert(RigidBody::Dynamic);
                commands
                    .entity(entity)
                    .insert(CollisionGroups::new(COMBOBOX_BIT, COMBOBOX_FILTER));
            }
        }
        if let ComboboxState::DespawningAnimation(animation_time) = &mut *combobox_state {
            *animation_time += time.delta_seconds();

            if *animation_time >= ComboboxState::DESPAWN_TIME {
                new_state = Some(ComboboxState::Despawned)
            }
        }

        if let Some(new_state) = new_state {
            *combobox_state = new_state;
        }

        transform.scale = Vec3::ONE * combobox_state.get_scale();
    }
}

fn pushback(
    mut comboboxes: Query<(Entity, &Combobox, &ComboboxState, &mut Transform)>,
    mut bodies: Query<(
        &mut ExternalImpulse,
        &Velocity,
        &ReadMassProperties,
        &RigidBody,
    )>,
    context: ResMut<RapierContext>,
    time: Res<Time>,
) {
    for (entity, combobox, combobox_state, mut transform) in comboboxes.iter_mut() {
        if let ComboboxState::SpawningAnimation(_) = *combobox_state {
            let directions = [
                Vec2::new(-1.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, -1.0),
                Vec2::new(0.0, 1.0),
            ];
            let offsets = [-0.76, -0.45, 0.0, 0.45, 0.76];
            let half_size = combobox.world_size() * 0.5 * combobox_state.get_scale_ahead(0.1);
            let half_size2 = combobox.world_size() * 0.5 * combobox_state.get_scale_ahead(1.0);

            let filter: QueryFilter = QueryFilterFlags::EXCLUDE_KINEMATIC.into();

            for offset in offsets {
                for dir in directions {
                    let origin = transform.translation.xy() + dir.perp() * offset * half_size;
                    context.intersections_with_ray(
                        origin,
                        dir,
                        half_size2 * 1.2,
                        true,
                        filter.exclude_collider(entity),
                        |v, e| {
                            if let Ok((mut impulse, velocity, mass, RigidBody::Dynamic)) =
                                bodies.get_mut(v)
                            {
                                let depth = (half_size2 * 1.01 - e.toi).clamp(0.0, 50.0);
                                let velocity = dir.dot(velocity.linvel);
                                impulse.impulse +=
                                    dir * time.delta_seconds() * depth * mass.0.mass * 300.0;
                                impulse.impulse -=
                                    dir * time.delta_seconds() * velocity * mass.0.mass * 5.0;
                            } else {
                                let depth = (half_size * 1.01 - e.toi).clamp(0.0, 50.0);
                                let offset = dir * depth * 0.3;
                                transform.translation -= Vec3::new(offset.x, offset.y, 0.0);
                            }
                            true
                        },
                    );
                }
            }
        }
    }
}

fn change_direction(
    comboboxes: Query<(&Combobox, &RapierRigidBodyHandle)>,
    mut context: ResMut<RapierContext>,
) {
    for (combobox, handle) in comboboxes.iter() {
        if let Some(mut gravity) = combobox.local_gravity {
            if let Some(rb) = context.bodies.get_mut(handle.0) {
                gravity = gravity * GRAVITY * rb.mass();
                rb.set_gravity_scale(0., true);
                rb.reset_forces(true);
                rb.add_force(Vec2::new(gravity.x, gravity.y).into(), true);
            }
        }
    }
}

fn change_gravity(
    comboboxes: Query<&Combobox>,
    rigidbodies: Query<&RapierRigidBodyHandle>,
    mut config: ResMut<RapierConfiguration>,
    mut context: ResMut<RapierContext>,
) {
    let mut gravity_change_flag = false;
    for combobox in comboboxes.iter() {
        if matches!(combobox.box_type, ComboboxType::Gravity) {
            if let Some(gravity) = combobox.local_gravity {
                config.gravity = gravity * GRAVITY;
                gravity_change_flag = true;
            }
        }
    }

    if gravity_change_flag {
        for handle in rigidbodies.iter() {
            if let Some(rb) = context.bodies.get_mut(handle.0) {
                rb.wake_up(true);
            }
        }
    }
}

fn despawn(mut commands: Commands, mut comboboxes: Query<(Entity, &ComboboxState)>) {
    for (entity, combobox_state) in comboboxes.iter_mut() {
        if *combobox_state == ComboboxState::Despawned {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn merge(
    mut commands: Commands,
    comboboxes: Query<(Entity, &Parent, &Combobox, &Transform, &ComboboxState)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
    mut assets: ResMut<AssetServer>,
    audio: Res<Audio>,
) {
    'outer: for (i, (a, parent, combobox_a, transform_a, state_a)) in comboboxes.iter().enumerate()
    {
        for (j, (b, _, combobox_b, transform_b, state_b)) in comboboxes.iter().enumerate() {
            if i >= j {
                continue;
            }

            if *state_a != ComboboxState::Normal || *state_b != ComboboxState::Normal {
                continue;
            }

            let pos_a = transform_a.translation.xy();
            let pos_b = transform_b.translation.xy();

            if (pos_a - pos_b).abs().max_element()
                < (combobox_a.world_size() + combobox_b.world_size()) * 0.52
            {
                if let Some(merge) = Combobox::merge(combobox_a, pos_a, combobox_b, pos_b) {
                    for (combobox_new, pos_new) in merge {
                        let id = commands
                            .spawn_bundle(ComboboxBundle::new(
                                combobox_new,
                                pos_new,
                                &mut meshes,
                                &mut materials,
                                &mut assets,
                            ))
                            .id();
                        commands.entity(parent.get()).add_child(id);

                        audio.play(assets.load("audio/box_join.ogg"));
                    }
                    commands
                        .entity(a)
                        .insert(RigidBody::KinematicPositionBased)
                        .insert(ComboboxState::DespawningAnimation(0.0))
                        .insert(CollisionGroups::new(0, 0));
                    commands
                        .entity(b)
                        .insert(RigidBody::KinematicPositionBased)
                        .insert(ComboboxState::DespawningAnimation(0.0))
                        .insert(CollisionGroups::new(0, 0));
                    break 'outer;
                }
            }
        }
    }
}
