use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::QueryFilterFlags;

use crate::core::ComboboxBundle;
use crate::game::{GameState, Material};

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

#[derive(Component, Clone, Debug)]
pub struct Combobox {
    pub weight: f32,
    pub box_type: ComboboxType,
    pub combined_from: Vec<(Combobox, Vec2)>,
    pub local_gravity: Option<Vec2>,
}

impl Combobox {
    pub const SPAWN_TIME: f32 = 1.0;
    pub const DESPAWN_TIME: f32 = 1.0;
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
        match (&first.box_type, &second.box_type) {
            (
                ComboboxType::Standard { group: group1 },
                ComboboxType::Standard { group: group2 },
            ) => {
                if group1 != group2 {
                    return None;
                }

                let center = (first_pos + second_pos) * 0.5;
                let first_offset = first_pos - center;
                let second_offset = second_pos - center;

                let big_box = Combobox {
                    weight: first.weight + second.weight,
                    box_type: first.box_type.clone(),
                    combined_from: vec![
                        (first.clone(), first_offset),
                        (second.clone(), second_offset),
                    ],
                    local_gravity: None,
                };

                return Some(vec![(big_box, center)]);
            }
            (ComboboxType::Buf, ComboboxType::Standard { .. }) => {
                let center = (first_pos + second_pos) * 0.5;
                let first_offset = first_pos - center;
                let second_offset = second_pos - center;

                let buffed_box = Combobox {
                    weight: second.weight * 2.,
                    box_type: second.box_type.clone(),
                    combined_from: vec![
                        (first.clone(), first_offset),
                        (second.clone(), second_offset),
                    ],
                    local_gravity: None,
                };
                return Some(vec![(buffed_box, second_pos)]);
            }
            (ComboboxType::Undo, _) => {
                if second.combined_from.len() == 0 {
                    return None;
                }

                return Some(
                    second
                        .combined_from
                        .iter()
                        .map(|(c, v)| (c.clone(), *v * 1.3 + second_pos))
                        .collect(),
                );
            }
            (_, ComboboxType::Undo) | (ComboboxType::Standard { .. }, ComboboxType::Buf) => {
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
                .with_system(despawn),
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
            transform.scale = Vec3::ONE * (*animation_time / Combobox::SPAWN_TIME).clamp(0.01, 1.0);

            if *animation_time >= Combobox::SPAWN_TIME {
                new_state = Some(ComboboxState::Normal);
                commands.entity(entity).insert(RigidBody::Dynamic);
            }
        }
        if let ComboboxState::DespawningAnimation(animation_time) = &mut *combobox_state {
            *animation_time += time.delta_seconds();
            transform.scale =
                Vec3::ONE * (1.0 - *animation_time / Combobox::DESPAWN_TIME).clamp(0.01, 1.0);

            if *animation_time >= Combobox::DESPAWN_TIME {
                new_state = Some(ComboboxState::Despawned)
            }
        }

        if let Some(new_state) = new_state {
            *combobox_state = new_state;
        }
    }
}

fn pushback(
    mut comboboxes: Query<(Entity, &Combobox, &ComboboxState, &mut Transform)>,
    context: ResMut<RapierContext>,
) {
    for (entity, combobox, combobox_state, mut transform) in comboboxes.iter_mut() {
        if let ComboboxState::SpawningAnimation(_) = *combobox_state {
            let origin = transform.translation.xy();
            let directions = [
                Vec2::new(-1.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, -1.0),
                Vec2::new(0.0, 1.0),
            ];
            let half_size = combobox.world_size() * 0.5 * transform.scale.x;

            let filter: QueryFilter = QueryFilterFlags::EXCLUDE_KINEMATIC.into();

            for dir in directions {
                context.intersections_with_ray(
                    origin,
                    dir,
                    half_size * 1.02,
                    true,
                    filter.exclude_collider(entity),
                    |_, e| {
                        let depth = (half_size * 1.02 - e.toi).clamp(0.0, 50.0);
                        let offset = dir * depth * 0.1;
                        transform.translation -= Vec3::new(offset.x, offset.y, 0.0);
                        true
                    },
                );
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
    comboboxes: Query<(Entity, &Combobox, &Transform, &ComboboxState)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
) {
    'outer: for (i, (a, combobox_a, transform_a, state_a)) in comboboxes.iter().enumerate() {
        for (j, (b, combobox_b, transform_b, state_b)) in comboboxes.iter().enumerate() {
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
                        commands.spawn_bundle(ComboboxBundle::new(
                            combobox_new,
                            pos_new,
                            &mut meshes,
                            &mut materials,
                        ));
                    }
                    commands
                        .entity(a)
                        .insert(RigidBody::KinematicPositionBased)
                        .insert(ComboboxState::DespawningAnimation(0.0));
                    commands
                        .entity(b)
                        .insert(RigidBody::KinematicPositionBased)
                        .insert(ComboboxState::DespawningAnimation(0.0));
                    break 'outer;
                }
            }
        }
    }
}
