use bevy::prelude::Color;

use crate::core::{Combobox, ComboboxType, PlayerIndex, SceneBuilder};

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 60.0;

    builder.set_audio("audio/level1.ogg");

    builder.set_min_view_range(8.0);
    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    // spawning player
    builder.set_spawn_point_xy(5.5, 1., PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(5.5, 1., PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(7.5, 1., PlayerIndex::TwoPlayers(1));

    builder.spawn_hint_xy(4.0, 3.0, "images/controls.png");

    builder.spawn_hint_xy(14.0, 1.5, "images/controls-2.png");

    // spawning walls
    // 1
    builder.spawn_wall_from_to_xy(-INF, 0., -INF, INF);
    // 2
    builder.spawn_wall_from_to_xy(-INF, 2., -INF, 0.3);
    // 3
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, 0.);
    // 4
    builder.spawn_wall_from_to_xy(9., INF, -INF, 3.);
    // 5
    builder.spawn_wall_from_to_xy(12., 14., 5., 6.);
    // 6
    builder.spawn_wall_from_to_xy(13., 19., 6., 7.);
    // 7
    builder.spawn_wall_from_to_xy(17., 19., -INF, 7.);
    // 8
    builder.spawn_wall_from_to_xy(17., 22., -INF, 5.);
    // 9
    builder.spawn_wall_from_to_xy(30., INF, -INF, 6.5);
    // 10
    builder.spawn_wall_from_to_xy(37., INF, -INF, INF);
    // 11
    builder.spawn_wall_from_to_xy(-INF, INF, 13., INF);
    // 12
    builder.spawn_wall_from_to_xy(23., 27., 11., INF);
    // 13
    builder.spawn_wall_from_to_xy(7., 11., 11., INF);
    // 14
    builder.spawn_wall_from_to_xy(3., 7., 8., INF);
    // 15
    builder.spawn_wall_from_to_xy(-INF, 3., 6., INF);

    // spawning boxes
    // 1
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        2.5,
        0.5,
    );
    // 2
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        16.5,
        3.5,
    );
    // 3
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        21.5,
        5.5,
    );
    // 4
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        26.5,
        3.5,
    );
    // 5
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        28.5,
        3.5,
    );

    // spawning finish
    builder.set_finish_point_xy(33., 8.5);
}
