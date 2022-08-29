use bevy::prelude::{Color, Vec2};

use crate::{
    core::{Combobox, ComboboxType, PlayerIndex, SceneBuilder},
    utils::SceneDirection,
};

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 60.0;

    builder.set_audio("audio/level11.ogg");

    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    builder.set_boundaries(-20.0, 24.0, -13.5, 20.0);

    builder.set_min_view_range(7.0);
    builder.set_ambient_light(Color::BLACK);

    // spawning player
    builder.set_spawn_point_xy(-14.5, -2., PlayerIndex::SinglePlayer);

    builder.set_spawn_point_xy(-15., -2., PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(-9., -2., PlayerIndex::TwoPlayers(1));

    builder.spawn_hint_xy(6., -9.5, "images/enter-the-code.png");

    // spawning walls
    // 1-5
    builder.spawn_wall_from_to_xy(-INF, -17., -INF, INF);
    builder.spawn_wall_from_to_xy(-INF, -16., -INF, -1.);
    builder.spawn_wall_from_to_xy(-INF, -12., -INF, -3.);
    builder.spawn_wall_from_to_xy(-INF, -7., -INF, -6.);
    builder.spawn_wall_from_to_xy(-10., -7., -INF, -3.);
    // 6-10
    builder.spawn_wall_from_to_xy(-10., 0., -8., -3.);
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, -12.);
    builder.spawn_wall_from_to_xy(16., INF, -INF, -8.);
    builder.spawn_wall_from_to_xy(18., INF, -INF, INF);
    builder.spawn_wall_from_to_xy(10., INF, 7., INF);
    // 11-15
    builder.spawn_wall_from_to_xy(-INF, INF, 10., INF);
    builder.spawn_wall_from_to_xy(-INF, 2., 8., INF);
    builder.spawn_wall_from_to_xy(-INF, -14., 3., INF);
    builder.spawn_wall_from_to_xy(-INF, -16., 2., INF);
    builder.spawn_wall_from_to_xy(-INF, -11.8, 4., 5.);
    // 16-20
    builder.spawn_wall_from_to_xy(-INF, -12., 3., 4.);
    builder.spawn_wall_from_to_xy(-10., 15., -5.5, -5.);
    builder.spawn_wall_from_to_xy(13., 15., -5., 4.);
    builder.spawn_wall_from_to_xy(0., 15., 0., 4.);
    builder.spawn_wall_from_to_xy(0., 2., 0., 5.);
    // 21-25
    builder.spawn_wall_from_to_xy(-3., 0., -8., 2.);
    builder.spawn_wall_from_to_xy(-6.2, 15., 0., 2.);
    builder.spawn_wall_from_to_xy(-8., -6., 2., 5.);
    builder.spawn_wall_from_to_xy(-10., -6., 3., 5.);
    builder.spawn_wall_from_to_xy(-10.2, -6., 4., 5.);

    // spawning boxes
    // 1-5
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        -16.5,
        -0.5,
    );
    builder.spawn_box_xy(
        Combobox::new(
            4.,
            ComboboxType::Lamp {
                color: Color::GREEN,
            },
        ),
        -11.,
        -5.,
    );
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Lamp { color: Color::RED }),
        -7.5,
        5.5,
    );
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Direction { direction: Vec2::Y }),
        -4.,
        -2.5,
    );
    builder.spawn_box_xy(Combobox::new(1., ComboboxType::Gravity), 6.5, 4.5);
    // 6-10
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Direction { direction: Vec2::Y }),
        4.5,
        4.5,
    );
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Direction { direction: Vec2::Y }),
        -1.,
        -11.5,
    );
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        1.5,
        -2.5,
    );
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        3.5,
        -2.5,
    );
    builder.spawn_box_xy(
        Combobox::new(1., ComboboxType::Standard { group: 1 }),
        5.5,
        -2.5,
    );
    // 11-15
    builder.spawn_box_xy(
        Combobox::new(1.4, ComboboxType::Lamp { color: Color::RED }),
        13.5,
        -11.5,
    );
    builder.spawn_box_xy(
        Combobox::new(
            1.4,
            ComboboxType::Lamp {
                color: Color::GREEN,
            },
        ),
        14.5,
        -11.5,
    );
    builder.spawn_box_xy(
        Combobox::new(1.4, ComboboxType::Lamp { color: Color::BLUE }),
        13.5,
        -12.5,
    );
    builder.spawn_box_xy(
        Combobox::new(
            1.4,
            ComboboxType::Lamp {
                color: Color::YELLOW,
            },
        ),
        14.5,
        -12.5,
    );
    builder.spawn_box_xy(Combobox::new(1., ComboboxType::Undo), 12.5, 6.5);

    // spawning doors
    builder.spawn_door_xy(-5.5, -1.5, 3., SceneDirection::Down, 1, 0);
    builder.spawn_door_xy(10.5, 5.5, 3., SceneDirection::Down, 2, 0);
    builder.spawn_door_xy(
        -2.5,
        -10.,
        4.,
        SceneDirection::Up,
        4 | 8 | 16,
        32 | 64 | 128,
    );

    // spawning buttons
    builder.spawn_button_xy(-7.5, -2.5, SceneDirection::Up, 1);
    builder.spawn_button_xy(6.5, 9.5, SceneDirection::Down, 2);
    for i in 0..6 {
        builder.spawn_button_xy(1.5 + i as f32 * 2., -4.5, SceneDirection::Up, 0);
        builder.spawn_button_xy(1.5 + i as f32 * 2., -11.5, SceneDirection::Up, 2_u32.pow(2 + i));
    }
    
    // spawning finish
    builder.set_finish_point_xy(-5., -10.);
}
