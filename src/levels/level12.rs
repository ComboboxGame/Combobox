use bevy::prelude::Color;
use crate::core::{Combobox, ComboboxType, PlayerIndex, SceneBuilder};
use crate::utils::SceneDirection;
use crate::Vec2;

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 90.0;

    builder.set_audio("audio/level12git .ogg");

    builder.set_boundaries(-13.0, 28.0, -60.0, 10.0);

    builder.set_min_view_range(6.5);

    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    builder.spawn_hint_xy(-6.0, 4.5, "images/final.png");
    builder.spawn_hint_xy(-3.0, -4.0, "images/choose-wisely.png");
    builder.spawn_hint_xy(-3.0, -42.0, "images/you-did-it.png");

    // 1 - 5
    builder.spawn_wall_from_to_xy(-9.0, 0.0, 0.0, 1.0);
    builder.spawn_wall_from_to_xy(-10.0, -9.0, -INF, 1.0);
    builder.spawn_wall_from_to_xy(-INF, -10.0, -INF, -4.0);
    builder.spawn_wall_from_to_xy(-INF, -11.0, -INF, INF);
    builder.spawn_wall_from_to_xy(-INF, -9.0, 7.0, INF);

    // 6 - 10
    builder.spawn_wall_from_to_xy(-INF, INF, 8.0, INF);
    builder.spawn_wall_from_to_xy(-10.0, -9.0, 3.0, INF);
    builder.spawn_wall_from_to_xy(-3.0, INF, 6.0, INF);
    builder.spawn_wall_from_to_xy(17.0, INF, 2.0, INF);
    builder.spawn_wall_from_to_xy(24.0, INF, -INF, INF);

    // 11 - 15
    builder.spawn_wall_from_to_xy(15.0, INF, -INF, -10.0);
    builder.spawn_wall_from_to_xy(9.0, 15.0, -INF, -12.0);
    builder.spawn_wall_from_to_xy(2.0, 9.0, -40.0, -8.0);
    builder.spawn_wall_from_to_xy(2.0, 5.0, -40.0, -6.0);
    builder.spawn_wall_from_to_xy(-2.0, 0.0, -39.0, -6.0);

    // 16 - 20
    builder.spawn_wall_from_to_xy(-6.0, -4.0, -39.0, -6.0);
    builder.spawn_wall_from_to_xy(-INF, -8.0, -INF, -6.0);
    builder.spawn_wall_from_to_xy(-3.0, 0.0, 0.0, 3.0);
    builder.spawn_wall_from_to_xy(0.0, 12.0, -1.0, 2.0);
    builder.spawn_wall_from_to_xy(9.0, 12.0, -3.0, 2.0);

    // 21
    builder.spawn_wall_from_to_xy(11.0, 21.0, -2.0, -5.0);
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, -44.0);


    builder.set_spawn_point_xy(-7.8, -2.0, PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(-7.0, 2.0, PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(-5.5, 2.0, PlayerIndex::TwoPlayers(1));

    builder.set_finish_point_xy(7.0, -42.0);

    builder.spawn_button_xy(-7.5, 7.5, SceneDirection::Down, 1);

    builder.spawn_door_xy(-0.5, 4.5, 3.0, SceneDirection::Down, 1, 0);

    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Standard {group: 1}), -2.5, 3.5);

    for i in 0..3 {
        builder.spawn_box_xy(Combobox {
            weight: 0.9,
            box_type: ComboboxType::Direction {direction: Vec2::NEG_Y},
            combined_from: vec![],
            local_gravity: Some(Vec2::Y),
        }, -10.5, 3.5 + i as f32);
    }

    for i in 0..4 {
        builder.spawn_box_xy(Combobox {
            weight: 0.9,
            box_type: ComboboxType::Direction {direction: Vec2::Y},
            combined_from: vec![],
            local_gravity: Some(Vec2::NEG_Y),
        }, -10.5, -0.5 - i as f32);
    }

    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Gravity), -9.5, 1.5);

    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Standard {group: 1}), 1.5, 3.5);
    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Standard {group: 1}), 3.5, 3.5);
    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Standard {group: 1}), 5.5, 3.5);
    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Standard {group: 1}), 7.5, 3.5);
    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Standard {group: 1}), 9.5, 3.5);
    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Buff(4.0)), 11.5, 3.5);


    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Gravity), 18.5, -1.5);
    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Direction {direction: Vec2::NEG_Y}), 20.5, -1.5);


    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Standard {group: 1}), 19.0, -8.5);
    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Standard {group: 2}), 18.5, -9.5);
    builder.spawn_box_xy(Combobox::new(1.0, ComboboxType::Standard {group: 3}), 19.5, -9.5);
}
