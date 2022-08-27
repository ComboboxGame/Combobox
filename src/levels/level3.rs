use crate::core::{Combobox, ComboboxType, PlayerIndex, SceneBuilder};
use bevy::prelude::*;

pub fn setup(builder: &mut SceneBuilder) {
    const S: f32 = 50.0;
    const INF: f32 = 40.0;

    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    builder.set_min_view_range(6.0 * S);

    builder.set_spawn_point_xy(1.5 * S, S, PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(0.9 * S, S, PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(2.3 * S, S, PlayerIndex::TwoPlayers(1));

    // 1 - 7
    builder.spawn_wall_from_to(Vec2::new(-INF, -INF) * S, Vec2::new(0.0, INF) * S);
    builder.spawn_wall_from_to(Vec2::new(-1.0, -4.0) * S, Vec2::new(3.0, -1.0) * S);
    builder.spawn_wall_from_to(Vec2::new(-1.0, 6.0) * S, Vec2::new(10.0, INF) * S);
    builder.spawn_wall_from_to(Vec2::new(-1.0, -3.0) * S, Vec2::new(12.0, -INF) * S);
    builder.spawn_wall_from_to(Vec2::new(9.0, 1.0) * S, Vec2::new(10.0, 20.0) * S);
    builder.spawn_wall_from_to(Vec2::new(9.0, 7.0) * S, Vec2::new(16.0, INF) * S);
    builder.spawn_wall_from_to(Vec2::new(5.0, 1.0) * S, Vec2::new(7.0, 1.5) * S);

    // box 1-3
    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 1 }),
        Vec2::new(6.5, 2.0) * S,
    );
    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Buff(3.0)),
        Vec2::new(8.0, -2.5) * S,
    );
    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 1 }),
        Vec2::new(12.0, -2.5) * S,
    );

    // 8 - 15
    builder.spawn_wall_from_to(Vec2::new(15.0, -4.0) * S, Vec2::new(17.0, -1.0) * S);
    builder.spawn_wall_from_to(Vec2::new(15.0, 1.5) * S, Vec2::new(18.0, 2.65) * S);
    builder.spawn_wall_from_to(Vec2::new(12.0, -3.3) * S, Vec2::new(15.0, -INF) * S);
    builder.spawn_wall_from_to(Vec2::new(17.0, 0.2) * S, Vec2::new(28.0, 2.0) * S);
    builder.spawn_wall_from_to(Vec2::new(16.0, 10.0) * S, Vec2::new(INF, INF) * S);
    builder.spawn_wall_from_to(Vec2::new(22.0, 6.0) * S, Vec2::new(23.0, 30.0) * S);
    builder.spawn_wall_from_to(Vec2::new(18.0, 5.0) * S, Vec2::new(20.0, 5.5) * S);
    builder.spawn_wall_from_to(Vec2::new(26.0, 0.0) * S, Vec2::new(28.0, 6.45) * S);

    // box 4 - 6
    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 2 }),
        Vec2::new(19.0, 6.0) * S,
    );
    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 2 }),
        Vec2::new(25.5, 2.5) * S,
    );
    builder.spawn_box(
        Combobox::new(1.0, ComboboxType::Standard { group: 2 }),
        Vec2::new(27.0, 7.0) * S,
    );

    // 16 - 19
    builder.spawn_wall_from_to(Vec2::new(32.0, -INF) * S, Vec2::new(INF, INF) * S);
    builder.spawn_wall_from_to(Vec2::new(15.0, -INF) * S, Vec2::new(INF, -9.0) * S);
    builder.spawn_wall_from_to(Vec2::new(20.0, -6.0) * S, Vec2::new(26.0, -4.2) * S);
    builder.spawn_wall_from_to(Vec2::new(30.0, 6.0) * S, Vec2::new(33.0, 20.0) * S);

    builder.set_finish_point(Vec2::new(23.0, -2.2) * S);
}
