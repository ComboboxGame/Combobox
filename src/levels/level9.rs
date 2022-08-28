use bevy::prelude::{Color, Vec2};

use crate::{
    core::{Combobox, ComboboxType, ElevatorType, PlayerIndex, SceneBuilder},
    utils::SceneDirection,
};

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 60.0;

    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    // spawning player
    builder.set_spawn_point_xy(-4.5, -5., PlayerIndex::SinglePlayer);

    // spawning walls
    // 1
    builder.spawn_wall_from_to_xy(-INF, -38., -INF, INF);
    // 2
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, -14.);
    // 3
    builder.spawn_wall_from_to_xy(-27., -25., -INF, -6.);
    // 4
    builder.spawn_wall_from_to_xy(-27., INF, -INF, -12.);
    // 5
    builder.spawn_wall_from_to_xy(-14., INF, -INF, -11.);
    // 6
    builder.spawn_wall_from_to_xy(-12., INF, -INF, -6.);
    // 7
    builder.spawn_wall_from_to_xy(-21., INF, -8., -6.);
    // 8
    builder.spawn_wall_from_to_xy(0., 10., -INF, -3.);
    // 9
    builder.spawn_wall_from_to_xy(6., 10., -INF, 0.);
    // 10
    builder.spawn_wall_from_to_xy(10., 14., -INF, -5.);
    // 11
    builder.spawn_wall_from_to_xy(14., 17., -INF, 0.);
    // 12
    builder.spawn_wall_from_to_xy(14., 22., -2., 0.);
    // 13
    builder.spawn_wall_from_to_xy(0., INF, -INF, -5.);
    // 14
    builder.spawn_wall_from_to_xy(24., 27., -INF, 0.);
    // 15
    builder.spawn_wall_from_to_xy(27., 30., -INF, -0.5);
    // 16
    builder.spawn_wall_from_to_xy(30., INF, -INF, INF);
    // 17
    builder.spawn_wall_from_to_xy(-INF, INF, 9., INF);
    // 18
    builder.spawn_wall_from_to_xy(-9., 21., 8., INF);
    // 19
    builder.spawn_wall_from_to_xy(0., 12., 6., INF);
    // 20
    builder.spawn_wall_from_to_xy(0., 4., 3.5, INF);
    // 21
    builder.spawn_wall_from_to_xy(-9., 21., 7., INF);
    // 22
    builder.spawn_wall_from_to_xy(-INF, -36., 7., INF);
    // 23
    builder.spawn_wall_from_to_xy(-35., -30., -11., 0.);
    // 24
    builder.spawn_wall_from_to_xy(-33., -27., -2., 2.);
    // 25
    builder.spawn_wall_from_to_xy(-31., -17., -1.5, 4.);
    // 26
    builder.spawn_wall_from_to_xy(-14.5, -11., -1., 4.);
    // 27
    builder.spawn_wall_from_to_xy(-13., -10., -3., 3.);
    // 28
    builder.spawn_wall_from_to_xy(-14.5, 0., 0., 2.);
    // 29
    builder.spawn_wall_from_to_xy(26., 28., 4., 5.);
    // 30
    builder.spawn_wall_from_to_xy(27.5, 28., 0.5, 5.);
    // 31
    builder.spawn_wall_from_to_xy(-24., -23., 4., 6.);

    // spawning boxes
    // 1
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        -0.5,
        -5.5,
    );
    // 2
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Direction { direction: Vec2::Y }),
        -15.5,
        -5.5,
    );
    // 3
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        -12.5,
        4.5,
    );
    // 4
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        -17.5,
        -11.5,
    );
    // 5
    builder.spawn_box_xy(Combobox::new(1., ComboboxType::Buff(4.)), -13., -10.5);
    // 6
    builder.spawn_box_xy(Combobox::new(1., ComboboxType::Undo), -29.5, 4.5);
    // 7
    builder.spawn_box_xy(Combobox::new(1., ComboboxType::Gravity), -30.5, -16.5);
    // 8
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Direction { direction: Vec2::Y }),
        -35.5,
        -16.5,
    );
    // 9
    builder.spawn_box_xy(
        Combobox {
            weight: 1.,
            box_type: ComboboxType::Undo,
            local_gravity: Some(Vec2::Y),
            combined_from: vec![],
        },
        28.5,
        8.5,
    );
    // 10
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        29.,
        0.5,
    );
    // 11
    builder.spawn_box_xy(
        Combobox::new(0.9, ComboboxType::Direction { direction: Vec2::Y }),
        27.5,
        0.5,
    );
    // 12
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        19.5,
        -4.5,
    );

    // spawning doors
    builder.spawn_door_xy(-11.5, -4.5, 3., SceneDirection::Up, 1, 0);
    builder.spawn_door_xy(-28.5, -6.5, 3., SceneDirection::Right, 2 | 4, 0);
    builder.spawn_door_xy(12., -0.5, 4., SceneDirection::Left, 8 | 16, 0);

    // spawning buttons
    builder.spawn_button_xy(-8.5, -5.5, SceneDirection::Up, 1);
    builder.spawn_button_xy(-18.5, -2., SceneDirection::Down, 2);
    builder.spawn_button_xy(-20.5, -2., SceneDirection::Down, 4);
    builder.spawn_button_xy(15.5, 0.5, SceneDirection::Up, 8);
    builder.spawn_button_xy(17.5, 0.5, SceneDirection::Up, 16);

    // spawning elevators
    let elevator_height = 0.1;
    builder.spawn_elevator_xy(
        -23.,
        -12. + elevator_height,
        -23.,
        -6. - elevator_height,
        ElevatorType::Loop {
            period: 6.,
            current: 0.,
        },
    );
    builder.spawn_elevator_xy(
        23.,
        -5. + elevator_height,
        23.,
        4. - elevator_height,
        ElevatorType::Loop {
            period: 6.,
            current: 0.,
        },
    );

    // spawning finish
    builder.set_finish_point_xy(12., -3.);
}
