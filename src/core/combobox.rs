use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::core::ComboboxBundle;
use crate::game::{GameState, Material};

#[derive(Clone, Debug)]
pub enum ComboboxType {
    Standard { group: u32 },
    Undo,
    GravitySetter { direction: Vec2 },
    GravityActivator,
    Lamp { color: Color },
}

#[derive(Component, Clone, Debug)]
pub struct Combobox {
    pub size: f32,
    pub box_type: ComboboxType,
    pub combined_from: Vec<Combobox>,
}

impl Combobox {
    pub fn new(size: f32, box_type: ComboboxType) -> Combobox {
        Self {
            size,
            box_type,
            combined_from: vec![],
        }
    }

    pub fn merge(&self, other: &Combobox) -> Option<Vec<Combobox>> {
        match (&self.box_type, &other.box_type) {
            (_, _) => None,
        }
    }
}

pub struct ComboboxPlugin;

impl Plugin for ComboboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(combobox_system));
    }
}

fn combobox_system(
    mut commands: Commands,
    comboboxes: Query<(Entity, &Combobox, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
) {
    'outer: for (i, (a, combobox_a, transform_a)) in comboboxes.iter().enumerate() {
        for (j, (b, combobox_b, transform_b)) in comboboxes.iter().enumerate() {
            if i >= j {
                continue;
            }

            if (transform_a.translation - transform_b.translation).length()
                < (combobox_a.size + combobox_b.size) * 0.52
            {
                if let Some(merge) = combobox_a.merge(combobox_b) {
                    for combobox_new in merge {
                        let position_new =
                            (transform_a.translation + transform_b.translation).xy() * 0.5;
                        commands.spawn_bundle(ComboboxBundle::new(
                            combobox_new,
                            position_new,
                            &mut meshes,
                            &mut materials,
                        ));
                    }
                    commands.entity(a).despawn_recursive();
                    commands.entity(b).despawn_recursive();
                    break 'outer;
                }
            }
        }
    }
}
