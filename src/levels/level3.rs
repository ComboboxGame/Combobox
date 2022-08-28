use crate::core::{Combobox, ComboboxType, ElevatorType, PlayerIndex, SceneBuilder};
use bevy::prelude::*;

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 60.0;

    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    // spawning player
    builder.set_min_view_range(8.0);
    builder.set_spawn_point_xy(21.5, 3., PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(20.5, 3., PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(21.2, 3., PlayerIndex::TwoPlayers(1));

    // spawning walls
    // 1
    builder.spawn_wall_from_to_xy(-INF, 0., -INF, INF);
    // 2
    builder.spawn_wall_from_to_xy(-INF, 8., -INF, 5.);
    // 3
    builder.spawn_wall_from_to_xy(-INF, 8., -INF, 0.);
    // 4
    builder.spawn_wall_from_to_xy(10., 17., -INF, 5.);
    // 5
    builder.spawn_wall_from_to_xy(17., INF, -INF, 2.);
    // 6
    builder.spawn_wall_from_to_xy(24., INF, -INF, INF);
    // 7
    builder.spawn_wall_from_to_xy(14., INF, 12., INF);
    // 8
    builder.spawn_wall_from_to_xy(19., 21., 10., INF);
    // 9
    builder.spawn_wall_from_to_xy(11., INF, 14., INF);
    // 10
    builder.spawn_wall_from_to_xy(-INF, INF, 16., INF);
    // 11
    builder.spawn_wall_from_to_xy(-INF, 7., 10., 10.5);
    // 12
    builder.spawn_wall_from_to_xy(-INF, 3., 9., 10.5);

    // spawning boxes
    // 1
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 2 }),
        3.5,
        5.5,
    );
    // 2
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 2 }),
        5.5,
        5.5,
    );
    // 3
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        13.5,
        5.5,
    );
    // 4
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        22.5,
        2.5,
    );

    // spawning elevators
    let elevator_height = 0.1;
    // 1
    builder.spawn_elevator_xy(
        9.,
        0. + elevator_height,
        9.,
        5. - elevator_height,
        ElevatorType::Loop {
            period: 5.,
            current: 0.,
        },
    );
    // 2
    builder.spawn_elevator_xy(
        18.,
        2. + elevator_height,
        18.,
        5. - elevator_height,
        ElevatorType::Loop {
            period: 5.,
            current: 0.,
        },
    );

    // spawning finish
    builder.set_finish_point_xy(4., 12.5);
}
