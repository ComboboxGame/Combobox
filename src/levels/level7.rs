use crate::core::{Combobox, ComboboxType, ElevatorType, PlayerIndex, SceneBuilder};
use crate::utils::SceneDirection;
use bevy::prelude::{Color, Vec2};

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 60.0;

    builder.set_audio("audio/level7.ogg");

    builder.set_min_view_range(7.0);

    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    // 1 - 10
    builder.spawn_wall_from_to_xy(-INF, -4.0, -INF, INF);
    // skip 2
    // skip 3
    builder.spawn_wall_from_to_xy(-INF, 11.0, -INF, -2.0);
    builder.spawn_wall_from_to_xy(-INF, 0.0, 2.0, INF);
    builder.spawn_wall_from_to_xy(-INF, 2.0, 5.0, INF);
    builder.spawn_wall_from_to_xy(-INF, 21.0, 9.0, INF);
    builder.spawn_wall_from_to_xy(10.0, 14.0, 6.0, 15.0);
    builder.spawn_wall_from_to_xy(7.0, 14.0, -INF, 2.0);
    builder.spawn_wall_from_to_xy(13.0, 29.0, 0.0, 2.0);

    // 11 - 15
    builder.spawn_wall_from_to_xy(21.0, 23.0, 0.0, 5.4);
    builder.spawn_wall_from_to_xy(23.0, 29.0, 2.0, 4.0);
    builder.spawn_wall_from_to_xy(21.0, 23.0, 8.5, 14.0);
    builder.spawn_wall_from_to_xy(20.0, INF, 11.0, INF);
    builder.spawn_wall_from_to_xy(31.0, INF, 9.0, INF);

    // 16 - 19
    builder.spawn_wall_from_to_xy(32.0, INF, -INF, INF);
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, -8.0);
    builder.spawn_wall_from_to_xy(10.0, 22.0, -INF, -7.0);
    builder.spawn_wall_from_to_xy(26.0, 27.0, -6.0, -3.0);
    builder.spawn_wall_from_to_xy(16.0, 17.0, 4.5, 5.0);

    builder.spawn_box_xy(
        Combobox::new(1.0, ComboboxType::Direction { direction: Vec2::Y }),
        -1.0,
        0.5,
    );

    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Gravity), 5.5, -1.5);

    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Undo), 11.5, 2.5);

    builder.spawn_button_xy(11.5, 2.5, SceneDirection::Up, 1);

    builder.spawn_door_xy(13.5, 4.0, 4.0, SceneDirection::Up, 1, 0);

    builder.spawn_box_xy(
        Combobox::new(1.0, ComboboxType::Direction { direction: Vec2::Y }),
        16.5,
        5.5,
    );

    builder.spawn_box_xy(
        Combobox::new(1.0, ComboboxType::Direction { direction: Vec2::X }),
        24.5,
        4.5,
    );

    // builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Gravity), 27.5, 4.5);

    let mut undo = Combobox::new(1.0, ComboboxType::Undo);
    undo.local_gravity = Some(Vec2::X);

    builder.spawn_box_xy(undo, 25.5, -4.5);

    builder.spawn_elevator_xy(
        32.0,
        -7.0,
        26.0,
        -7.0,
        ElevatorType::Loop {
            period: 5.0,
            current: 0.0,
        },
    );

    builder.set_finish_point_xy(18.0, -2.0);

    builder.set_spawn_point_xy(1.5, -1.0, PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(1.5, -1.0, PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(3.5, -1.0, PlayerIndex::TwoPlayers(1));
}
