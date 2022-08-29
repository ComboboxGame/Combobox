use bevy::prelude::Color;

use crate::{
    core::{Combobox, ComboboxType, PlayerIndex, SceneBuilder},
    utils::SceneDirection,
};

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 70.0;

    builder.set_audio("audio/level5.ogg");

    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    // spawning player
    builder.set_min_view_range(8.0);
    builder.set_spawn_point_xy(3.5, 2., PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(3.0, 2., PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(5.0, 2., PlayerIndex::TwoPlayers(1));

    // spawning walls
    // 1
    builder.spawn_wall_from_to_xy(-INF, 0., -INF, INF);
    // 2
    builder.spawn_wall_from_to_xy(-INF, 2., -INF, 2.);
    // 3
    builder.spawn_wall_from_to_xy(-INF, 8., -INF, 1.);
    // 4
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, 0.);
    // 5
    builder.spawn_wall_from_to_xy(30., INF, -INF, 4.);
    // 6
    builder.spawn_wall_from_to_xy(36., INF, -INF, INF);
    // 7
    builder.spawn_wall_from_to_xy(-INF, INF, 12., INF);
    // 8
    builder.spawn_wall_from_to_xy(17., 26., 9., INF);
    // 9
    builder.spawn_wall_from_to_xy(23., 24., 2.5, INF);
    // 10
    builder.spawn_wall_from_to_xy(20., 24., 2.5, 5.);
    // 11
    builder.spawn_wall_from_to_xy(14., 24., 2.5, 4.);
    // 12
    builder.spawn_wall_from_to_xy(-INF, 26., 10., INF);
    // 13
    builder.spawn_wall_from_to_xy(-INF, 7., 8., INF);
    // 14
    builder.spawn_wall_from_to_xy(-INF, 3., 5., INF);

    // spawning boxes
    // 1
    builder.spawn_box_xy(Combobox::new(1., ComboboxType::Buff(3.)), 1.5, 2.5);
    // 2
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        6.5,
        1.5,
    );
    // 3
    builder.spawn_box_xy(Combobox::new(1., ComboboxType::Buff(3.)), 12.5, 0.5);
    // 4
    builder.spawn_box_xy(Combobox::new(1., ComboboxType::Undo), 15.5, 4.5);
    // 5
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        21.5,
        5.5,
    );
    // 6
    builder.spawn_box_xy(
        Combobox::new(2., ComboboxType::Standard { group: 2 }),
        28.,
        1.,
    );

    // spawn doors
    builder.spawn_door_xy(20.5, 1.25, 2.5, SceneDirection::Up, 1, 0);

    // spawn buttons
    builder.spawn_button_xy(18.5, 4.5, SceneDirection::Up, 1);

    // spawning finish
    builder.set_finish_point_xy(33., 6.);
}
