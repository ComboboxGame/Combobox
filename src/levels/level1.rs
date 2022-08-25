use crate::core::{Combobox, ComboboxType, MapBuilder, PlayerIndex};
use bevy::prelude::*;

pub fn setup_level1(builder: &mut MapBuilder) {
    builder.set_background_color(Color::rgb(0.06, 0.06, 0.06));

    builder.spawn_wall_from_to(Vec2::new(-1500.0, -1500.0), Vec2::new(1000.0, 0.0));

    builder.spawn_wall_from_to(Vec2::new(-1500.0, -1500.0), Vec2::new(-200.0, 1000.0));
    builder.spawn_wall_from_to(Vec2::new(900.0, -1500.0), Vec2::new(1800.0, 300.0));
    builder.spawn_wall_from_to(Vec2::new(1300.0, -1500.0), Vec2::new(2000.0, 2000.0));
    builder.spawn_wall_from_to(Vec2::new(-1500.0, 800.0), Vec2::new(2000.0, 2000.0));

    //builder.set_boundaries(-2000.0, 2000.0, -200.0, 2000.0);

    builder.set_spawn_point_xy(300.0, 50.0, PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(300.0, 50.0, PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(390.0, 50.0, PlayerIndex::TwoPlayers(1));

    builder.set_finish_point_xy(1050.0, 400.0);

    builder.spawn_box(
        Combobox::new(4.0, ComboboxType::Standard { group: 0 }),
        Vec2::new(100.0, 50.0),
    );

    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 0 }),
        Vec2::new(200.0, 50.0),
    );

    builder.spawn_box(
        Combobox::new(2.0, ComboboxType::Standard { group: 0 }),
        Vec2::new(500.0, 50.0),
    );
}
