use crate::core::{Combobox, ComboboxType, ElevatorType, MapBuilder};
use bevy::prelude::*;

pub fn setup_level0(builder: &mut MapBuilder) {
    builder.spawn_wall_from_to(Vec2::new(-1000.0, 300.0), Vec2::new(1000.0, 320.0));
    builder.spawn_wall_from_to(Vec2::new(-1000.0, -20.0), Vec2::new(1000.0, 0.0));
    builder.spawn_wall_from_to(Vec2::new(-1000.0, -20.0), Vec2::new(-980.0, 300.0));
    builder.spawn_wall_from_to(Vec2::new(980.0, -20.0), Vec2::new(1000.0, 300.0));

    builder.set_boundaries(-2000.0, 2000.0, -200.0, 2000.0);

    builder.set_spawn_point_xy(0.0, 30.0, 1);

    builder.spawn_elevator_xy(
        150.0,
        -3.0,
        150.0,
        200.0,
        ElevatorType::Loop { period: 5.0 },
    );

    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 0 }),
        Vec2::new(-220.0, 40.0),
    );

    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Buf),
        Vec2::new(-380.0, 160.0),
    );
}
