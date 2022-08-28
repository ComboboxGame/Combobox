use bevy::prelude::Color;
use crate::core::{Combobox, ComboboxType, PlayerIndex, SceneBuilder};
use crate::Vec2;

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 60.0;

    builder.set_min_view_range(8.0);
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
    builder.spawn_wall_from_to_xy(-INF, -16.0, -INF, INF);
    builder.spawn_wall_from_to_xy(-16.0, -14.0, 6.0, INF);
    builder.spawn_wall_from_to_xy(-13.0, -12.0, 6.0, INF);
    builder.spawn_wall_from_to_xy(-11.0, -10.0, 6.0, 10.0);
    builder.spawn_wall_from_to_xy(-9.0, -8.0, 6.0, 10.0);
    builder.spawn_wall_from_to_xy(-7.0, -5.0, 6.0, 10.0);

    // 13 - 18
    builder.spawn_wall_from_to_xy(-6.0, 1.0, 6.0, 7.0);
    builder.spawn_wall_from_to_xy(0.0, 1.0, 0.0, 7.0);
    builder.spawn_wall_from_to_xy(1.0, 3.0, -6.0, 4.0);
    builder.spawn_wall_from_to_xy(-INF, -12.0, 10.0, INF);
    builder.spawn_wall_from_to_xy(-INF, INF, 14.0, INF);
    builder.spawn_wall_from_to_xy(-2.0, INF, 11.0, INF);

    // 19 - 24
    builder.spawn_wall_from_to_xy(-11.0, -5.0, 10.0, 11.0);
    builder.spawn_wall_from_to_xy(2.0, 3.0, 10.0, 11.0);
    builder.spawn_wall_from_to_xy(17.0, 18.0, 18.0, 21.0);
    builder.spawn_wall_from_to_xy(21.0, INF, 18.0, 21.0);
    builder.spawn_wall_from_to_xy(17.0, INF, -1.0, 18.0);
    builder.spawn_wall_from_to_xy(20.0, INF, -4.0, -1.0);

    // 25 - 30
    builder.spawn_wall_from_to_xy(7.0, 8.0, -1.0, 6.0);
    builder.spawn_wall_from_to_xy(13.0, 14.0, -1.0, 6.0);
    builder.spawn_wall_from_to_xy(7.0, 14.0, -1.0, 0.0);
    builder.spawn_wall_from_to_xy(7.0, 14.0, 5.0, 6.0);
    builder.spawn_wall_from_to_xy(3.0, 14.0, -6.0, -4.0);
    builder.spawn_wall_from_to_xy(17.0, INF, -INF, -4.0);

    // 31 - 34
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, -10.0);
    builder.spawn_wall_from_to_xy(-4.0, -2.0, -INF, -6.5);
    builder.spawn_wall_from_to_xy(-INF, -4.0, -INF, -6.0);
    builder.spawn_wall_from_to_xy(-INF, -14.0, -INF, -2.0);

    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Lamp {color: Color::RED}), -7.5, -0.5);
    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Lamp {color: Color::BLUE}), -9.5, -0.5);
    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Lamp {color: Color::YELLOW}), -11.5, -0.5);
    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Lamp {color: Color::GREEN}), -13.5, -0.5);

    builder.spawn_box_xy(Combobox::new(0.98, ComboboxType::Gravity), -15.5, -0.5);
    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Direction {direction: Vec2::Y}), -2.5, 0.5);
    builder.spawn_box_xy(Combobox::new(0.9, ComboboxType::Direction {direction: Vec2::Y}), -1.0, 0.5);

    builder.set_spawn_point_xy(-5.5, 1.0, PlayerIndex::SinglePlayer);
    builder.set_spawn_point_xy(-6.0, 1.0, PlayerIndex::TwoPlayers(0));
    builder.set_spawn_point_xy(-4.5, 1.0, PlayerIndex::TwoPlayers(1));

    builder.set_finish_point_xy(-12.0, -4.0);



}
