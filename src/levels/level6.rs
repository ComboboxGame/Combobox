use bevy::prelude::{Color, Vec2};

use crate::{
    core::{Combobox, ComboboxType, ElevatorType, PlayerIndex, SceneBuilder},
    utils::SceneDirection,
};

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 70.0;

    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    // spawning player
    builder.set_min_view_range(8.0);
    builder.set_spawn_point_xy(4.5, 1., PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(4.5, 1., PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(6.5, 1., PlayerIndex::TwoPlayers(1));

    // spawning walls
    // 1
    builder.spawn_wall_from_to_xy(-INF, 0., -INF, INF);
    // 2
    builder.spawn_wall_from_to_xy(-INF, 3., -INF, 2.);
    // 3
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, 0.);
    // 4
    builder.spawn_wall_from_to_xy(10., INF, -INF, 3.);
    // 5
    builder.spawn_wall_from_to_xy(33.5, INF, -INF, 4.5);
    // 6
    builder.spawn_wall_from_to_xy(34., INF, -INF, 10.);
    // 7
    builder.spawn_wall_from_to_xy(33., 34., 8., 8.5);
    // 8
    builder.spawn_wall_from_to_xy(39., INF, -INF, INF);
    // 9
    builder.spawn_wall_from_to_xy(35., INF, 14., INF);
    // 10
    builder.spawn_wall_from_to_xy(34., 35., 13., INF);
    // 11
    builder.spawn_wall_from_to_xy(32., INF, 15., INF);
    // 12
    builder.spawn_wall_from_to_xy(28., INF, 15., INF);
    // 13
    builder.spawn_wall_from_to_xy(-INF, INF, 15.5, INF);
    // 14
    builder.spawn_wall_from_to_xy(-INF, 24., 15., INF);
    // 15
    builder.spawn_wall_from_to_xy(18., 22., 6., INF);
    // 16
    builder.spawn_wall_from_to_xy(18.5, 24., 6., 11.);
    // 17
    builder.spawn_wall_from_to_xy(-INF, 22., 10., INF);
    // 18
    builder.spawn_wall_from_to_xy(8., 10., 8., INF);
    // 19
    builder.spawn_wall_from_to_xy(-INF, 10., 9., INF);
    // 20
    builder.spawn_wall_from_to_xy(-INF, 6., 6., INF);
    // 21
    builder.spawn_wall_from_to_xy(-INF, 3., 4., INF);
    // 22
    builder.spawn_wall_from_to_xy(30., 30.6, 10., 10.5);
    // 23
    builder.spawn_wall_from_to_xy(28., 30., 6., 11.);
    // 24
    builder.spawn_wall_from_to_xy(30., 31., 5., 7.);
    // 25
    builder.spawn_wall_from_to_xy(31., 32., 5., 6.);
    // 26
    builder.spawn_wall_from_to_xy(15.0, 19., 7., 12.);

    // spawning boxes
    // 1
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Direction { direction: Vec2::Y }),
        1.5,
        2.5,
    );
    // 2
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Direction { direction: Vec2::Y }),
        11.5,
        3.5,
    );
    // 3
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        15.5,
        3.5,
    );
    // 4
    builder.spawn_box_xy(
        Combobox::new(16., ComboboxType::Standard { group: 2 }),
        26.,
        5.,
    );
    // 5
    builder.spawn_box_xy(Combobox::new(1., ComboboxType::Undo), 29.5, 11.5);

    // spawn doors
    builder.spawn_door_xy(19.5, 4.5, 3., SceneDirection::Up, 1, 0);
    builder.spawn_door_xy(34.5, 11.5, 3., SceneDirection::Up, 2, 0);

    // spawn buttons
    builder.spawn_button_xy(13.5, 9.5, SceneDirection::Down, 1);
    builder.spawn_button_xy(26., 3.5, SceneDirection::Up, 2);

    // spawning elevators
    let elevator_height = 0.10;
    builder.spawn_elevator_xy(
        9.,
        0. + elevator_height - 0.05,
        9.,
        3. - elevator_height + 0.01,
        ElevatorType::Loop {
            period: 5.,
            current: 0.,
        },
    );

    // spawning finish
    builder.set_finish_point_xy(37., 12.);
}
