use bevy::prelude::*;

pub enum ComboboxType {
    Standard { group: u32, can_subtract: bool },
    Undo,
    GravitySetter { direction: Vec2 },
    GravityActivator,
    Lamp { color: Color },
}

#[derive(Component)]
pub struct Combobox {
    size: u32,
    box_type: ComboboxType,
    combined_from: Vec<Combobox>,
}
