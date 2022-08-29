use crate::core::{Combobox, ComboboxType, PlayerIndex, SceneBuilder};
use crate::utils::SceneDirection;
use bevy::prelude::Color;

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 60.0;

    builder.set_audio("audio/level8.ogg");

    builder.set_min_view_range(8.0);
    builder.set_ambient_light(Color::BLACK);

    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    // 1 - 5
    builder.spawn_wall_from_to_xy(-INF, 0.0, -INF, INF);
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, 0.0);
    builder.spawn_wall_from_to_xy(-INF, 2.0, -INF, 3.0);
    builder.spawn_wall_from_to_xy(-INF, INF, 9.0, 12.0);
    builder.spawn_wall_from_to_xy(5.0, INF, 4.0, 5.3);

    // additional
    builder.spawn_wall_from_to_xy(5.0, 6.0, 5.0, 6.5);

    // 5 - 10
    builder.spawn_wall_from_to_xy(4.0, 5.0, 4.0, 4.6);
    builder.spawn_wall_from_to_xy(-INF, 7.0, 10.0, INF);
    builder.spawn_wall_from_to_xy(22.0, INF, 10.0, INF);
    builder.spawn_wall_from_to_xy(30.0, INF, -INF, INF);
    builder.spawn_wall_from_to_xy(23.0, INF, 5.0, 10.0);

    // Lower buttons
    builder.spawn_button_xy(19.5, 0.5, SceneDirection::Up, 1);
    builder.spawn_button_xy(17.5, 0.5, SceneDirection::Up, 2);
    builder.spawn_button_xy(15.5, 0.5, SceneDirection::Up, 4);
    builder.spawn_button_xy(13.5, 0.5, SceneDirection::Up, 8);
    builder.spawn_button_xy(11.5, 0.5, SceneDirection::Up, 16);
    builder.spawn_button_xy(9.5, 0.5, SceneDirection::Up, 32);

    builder.spawn_door_xy(25.0, 2.0, 4.0, SceneDirection::Up, 1 | 8 | 16, 2 | 4 | 32);

    builder.spawn_box_xy(
        Combobox::new(1.0, ComboboxType::Lamp { color: Color::RED }),
        2.5,
        0.5,
    );
    builder.spawn_box_xy(
        Combobox::new(
            1.0,
            ComboboxType::Lamp {
                color: Color::GREEN,
            },
        ),
        5.5,
        0.5,
    );
    builder.spawn_box_xy(
        Combobox::new(1.0, ComboboxType::Lamp { color: Color::BLUE }),
        8.0,
        0.5,
    );

    builder.spawn_box_xy(
        Combobox::new(
            1.0,
            ComboboxType::Lamp {
                color: Color::PURPLE * 2.0,
            },
        ),
        19.5,
        6.5,
    );

    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Buff(3.0)), 9.5, 6.5);

    // Upper buttons
    builder.spawn_button_xy(19.5, 12.5, SceneDirection::Up, 0);
    builder.spawn_button_xy(17.5, 12.5, SceneDirection::Up, 0);
    builder.spawn_button_xy(15.5, 12.5, SceneDirection::Up, 0);
    builder.spawn_button_xy(13.5, 12.5, SceneDirection::Up, 0);
    builder.spawn_button_xy(11.5, 12.5, SceneDirection::Up, 0);
    builder.spawn_button_xy(9.5, 12.5, SceneDirection::Up, 0);

    // Example boxes
    builder.spawn_box_xy(
        Combobox::new(1.0, ComboboxType::Standard { group: 0 }),
        19.5,
        12.5,
    );
    builder.spawn_box_xy(
        Combobox::new(1.0, ComboboxType::Standard { group: 0 }),
        13.5,
        12.5,
    );
    builder.spawn_box_xy(
        Combobox::new(1.0, ComboboxType::Standard { group: 0 }),
        11.5,
        12.5,
    );

    builder.set_spawn_point_xy(23.5, 1.0, PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(21.5, 1.0, PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(23.5, 1.0, PlayerIndex::TwoPlayers(1));

    builder.spawn_hint_xy(15.5, 2.0, "images/enter-the-code.png");
    builder.spawn_hint_xy(25.5, 9.5, "images/code.png");

    builder.set_finish_point_xy(27.0, 2.0);
}
