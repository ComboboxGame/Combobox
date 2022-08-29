use bevy::prelude::Color;

use crate::core::{Combobox, ComboboxType, PlayerIndex, SceneBuilder};

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 60.0;

    builder.set_audio("audio/level4.ogg");

    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    // spawning player
    builder.set_min_view_range(8.0);
    builder.set_spawn_point_xy(2., 4., PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(1.0, 4., PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(3.0, 4., PlayerIndex::TwoPlayers(1));

    // spawning walls
    // 1
    builder.spawn_wall_from_to_xy(-INF, 0., -INF, INF);
    // 2
    builder.spawn_wall_from_to_xy(-INF, 4., -INF, 3.);
    // 3
    builder.spawn_wall_from_to_xy(-INF, 10.5, -INF, 1.);
    // 4
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, 0.5);
    // 5
    builder.spawn_wall_from_to_xy(15., 19., -INF, 5.);
    // 6
    builder.spawn_wall_from_to_xy(15., INF, -INF, 3.);
    // 7
    builder.spawn_wall_from_to_xy(26., INF, -INF, 6.5);
    // 8
    builder.spawn_wall_from_to_xy(31., INF, -INF, INF);
    // 9
    builder.spawn_wall_from_to_xy(-INF, INF, 13., INF);
    // 10
    builder.spawn_wall_from_to_xy(23., 25., 10., INF);
    // 11
    builder.spawn_wall_from_to_xy(-INF, 25., 12., INF);
    // 12
    builder.spawn_wall_from_to_xy(-INF, 12., 10., INF);
    // 13
    builder.spawn_wall_from_to_xy(9., 10., 5.5, INF);
    // 14
    builder.spawn_wall_from_to_xy(-INF, 10., 9., INF);
    // 15
    builder.spawn_wall_from_to_xy(-INF, 3., 8., INF);
    // 16
    builder.spawn_wall_from_to_xy(6., 9., 3.5, 4.);
    // 17
    builder.spawn_wall_from_to_xy(10., 13.5, 6., 6.5);
    // 18
    builder.spawn_wall_from_to_xy(19., 21., 7.5, 8.);

    // spawning boxes
    // 1
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        6.5,
        1.5,
    );
    // 2
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        10.,
        1.5,
    );
    // 3
    builder.spawn_box_xy(Combobox::new(1., ComboboxType::Buff(3.)), 7.5, 5.5);
    // 4
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        12.,
        7.,
    );
    // 5
    builder.spawn_box_xy(Combobox::new(1., ComboboxType::Buff(3.)), 20., 8.5);

    // spawning finish
    builder.set_finish_point_xy(29., 8.5);
}
