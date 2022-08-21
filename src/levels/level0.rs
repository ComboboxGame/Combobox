use crate::core::{Combobox, ComboboxType, MapBuilder};
use bevy::prelude::*;

pub fn setup_level0(builder: &mut MapBuilder) {
    builder.spawn_wall_from_to(Vec2::new(-1000.0, -20.0), Vec2::new(1000.0, 0.0));

    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 0 }),
        Vec2::new(-220.0, 40.0),
    );

    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Buf),
        Vec2::new(-220.0, 160.0),
    );

    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 0 }),
        Vec2::new(-320.0, 40.0),
    );

    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 0 }),
        Vec2::new(-320.0, 160.0),
    );

    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 0 }),
        Vec2::new(-320.0, 250.0),
    );

    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 0 }),
        Vec2::new(-320.0, 350.0),
    );

    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 0 }),
        Vec2::new(-320.0, 450.0),
    );
}
