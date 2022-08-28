use bevy::prelude::Color;
use crate::core::{Combobox, ComboboxType, PlayerIndex, SceneBuilder};
use crate::utils::SceneDirection;
use crate::Vec2;

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 60.0;

    builder.set_min_view_range(10.0);
    builder.set_ambient_light(Color::BLACK);

    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    // 1 - 6
    builder.spawn_wall_from_to_xy(-15.0, -14.0, -1.0, 0.0);
    builder.spawn_wall_from_to_xy(-13.0, -12.0, -1.0, 0.0);
    builder.spawn_wall_from_to_xy(-11.0, -10.0, -1.0, 0.0);
    builder.spawn_wall_from_to_xy(-9.0, -8.0, -1.0, 0.0);
    builder.spawn_wall_from_to_xy(-7.0, 0.0, -1.0, 0.0);
    builder.spawn_wall_from_to_xy(-INF, 3.0, -2.0, -1.0);

    // 7 - 12
    builder.spawn_wall_from_to_xy(-INF, -17.0, -INF, INF);
    builder.spawn_wall_from_to_xy(-17.0, -14.0, 6.5, INF);
    builder.spawn_wall_from_to_xy(-13.0, -12.1, 6.0, INF);
    builder.spawn_wall_from_to_xy(-10.8, -10.0, 6.0, 10.0);
    builder.spawn_wall_from_to_xy(-9.0, -8.0, 6.0, 10.0);
    builder.spawn_wall_from_to_xy(-7.0, -5.0, 6.0, 10.0);

    // 13 - 18
    builder.spawn_wall_from_to_xy(-6.0, 1.0, 6.0, 7.0);
    builder.spawn_wall_from_to_xy(0.0, 1.0, -2.0, 7.0);
    builder.spawn_wall_from_to_xy(1.0, 2.0, -6.0, 4.0);
    builder.spawn_wall_from_to_xy(-INF, -12.1, 10.0, INF);
    builder.spawn_wall_from_to_xy(-INF, INF, 14.0, INF);
    builder.spawn_wall_from_to_xy(-2.0, INF, 11.0, INF);

    // 19 - 24
    builder.spawn_wall_from_to_xy(-10.8, -5.0, 10.0, 11.0);
    builder.spawn_wall_from_to_xy(2.0, 3.0, 10.0, 11.0);
    builder.spawn_wall_from_to_xy(17.0, 18.0, 8.0, 11.0);
    builder.spawn_wall_from_to_xy(21.0, INF, 8.0, 11.0);
    builder.spawn_wall_from_to_xy(17.0, INF, -1.0, 8.0);
    builder.spawn_wall_from_to_xy(20.0, INF, -4.0, -1.0);

    // 25 - 30
    builder.spawn_wall_from_to_xy(7.0, 8.0, -1.0, 6.0);
    builder.spawn_wall_from_to_xy(13.0, 14.0, -1.0, 6.0);
    builder.spawn_wall_from_to_xy(7.0, 14.0, -1.0, 0.0);
    builder.spawn_wall_from_to_xy(7.0, 14.0, 5.0, 6.0);
    builder.spawn_wall_from_to_xy(2.0, 14.0, -6.0, -4.0);
    builder.spawn_wall_from_to_xy(17.0, INF, -INF, -4.0);

    // 31 - 34
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, -10.0);
    builder.spawn_wall_from_to_xy(-4.0, -2.0, -INF, -6.5);
    builder.spawn_wall_from_to_xy(-INF, -4.0, -INF, -6.0);
    builder.spawn_wall_from_to_xy(-INF, -14.0, -INF, -2.0);

    builder.spawn_wall_from_to_xy(5.0, 5.9, 8.5, 9.0);

    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Lamp {color: Color::RED * 1.3}), -7.5, -0.5);
    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Lamp {color: Color::BLUE * 1.3}), -9.5, -0.5);
    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Lamp {color: Color::YELLOW * 1.2}), -11.5, -0.5);
    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Lamp {color: Color::GREEN * 1.1}), -13.5, -0.5);

    builder.spawn_box_xy(Combobox::new(0.98, ComboboxType::Gravity), -16.5, -0.5);
    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Direction {direction: Vec2::Y}), -2.5, 0.5);
    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Direction {direction: Vec2::Y}), -1.0, 0.5);

    builder.spawn_box_xy(Combobox::new(0.85, ComboboxType::Direction {direction: Vec2::NEG_Y}), 4.4, 10.5);
    builder.spawn_box_xy(Combobox::new(0.85, ComboboxType::Direction {direction: Vec2::X}), 6.6, 10.5);
    builder.spawn_box_xy(Combobox::new(0.85, ComboboxType::Gravity), 5.5, 8.0);


    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Lamp {color: Color::CYAN * 1.5}), 11.5, 2.5);
    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Lamp {color: Color::ORANGE * 1.5}), 19.5, 9.5);

    builder.spawn_box_xy(Combobox::new(0.98, ComboboxType::Undo), 19.0, -2.5);

    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Buff(3.0)), 9.5, -9.5);

    builder.spawn_box_xy(Combobox {
        weight: 1.0,
        box_type: ComboboxType::Lamp {color: Color::LIME_GREEN * 1.5},
        combined_from: vec![],
        local_gravity: Some(Vec2::NEG_Y),
    }, 3.5, -9.5);

    builder.spawn_button_xy(0.5, -9.5, SceneDirection::Up, 1);

    builder.spawn_door_xy(-4.5, -4.5, 3.0, SceneDirection::Up, 1, 0);


    builder.set_spawn_point_xy(-5.5, 1.0, PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(-6.0, 1.0, PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(-4.5, 1.0, PlayerIndex::TwoPlayers(1));

    builder.set_finish_point_xy(-12.0, -4.0);



}
