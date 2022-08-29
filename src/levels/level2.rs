use bevy::prelude::Color;

use crate::core::{Combobox, ComboboxType, PlayerIndex, SceneBuilder};

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 60.0;

    builder.set_audio("audio/level2.ogg");

    builder.set_min_view_range(8.0);
    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    // spawning player
    builder.set_spawn_point_xy(10.5, 1., PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(9.5, 1., PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(11.5, 1., PlayerIndex::TwoPlayers(1));

    // spawning walls
    // 1
    builder.spawn_wall_from_to_xy(-INF, 0., -INF, INF);
    // 2
    builder.spawn_wall_from_to_xy(-INF, 7., -INF, 4.);
    // 3
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, 0.);
    // 4
    builder.spawn_wall_from_to_xy(18., INF, -INF, 4.5);
    // 5
    builder.spawn_wall_from_to_xy(24., INF, -INF, INF);
    // 6
    builder.spawn_wall_from_to_xy(15., INF, 11., INF);
    // 7
    builder.spawn_wall_from_to_xy(15., 20., 10., INF);
    // 8
    builder.spawn_wall_from_to_xy(10., 15., 12., INF);
    // 9
    builder.spawn_wall_from_to_xy(12., 14., 11., INF);
    // 10
    builder.spawn_wall_from_to_xy(8., 10., 9., INF);
    // 11
    builder.spawn_wall_from_to_xy(-INF, 10., 11., INF);
    // 12
    builder.spawn_wall_from_to_xy(-INF, 6., 8., INF);

    // spawning boxes
    // 1
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        1.5,
        4.5,
    );
    // 2
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        3.5,
        4.5,
    );
    // 3
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        5.5,
        4.5,
    );
    // 4
    builder.spawn_box_xy(
        Combobox::new(4., ComboboxType::Standard { group: 2 }),
        14.,
        1.,
    );
    // 5
    builder.spawn_box_xy(
        Combobox::new(4., ComboboxType::Standard { group: 1 }),
        17.,
        1.,
    );

    // spawning finish
    builder.set_finish_point_xy(21., 6.5);
}
