use crate::core::MapBuilder;
use bevy::prelude::*;

pub fn setup_level1(builder: &mut MapBuilder) {
    builder.set_background_color(Color::rgb(0.06, 0.06, 0.06));

    builder.set_boundaries(-800.0, 2000.0, -600.0, 2000.0);

    builder.spawn_wall_from_to(Vec2::new(-2500.0, -2500.0), Vec2::new(1000.0, 0.0));

    builder.spawn_wall_from_to(Vec2::new(-1500.0, -1500.0), Vec2::new(-200.0, 1000.0));
    builder.spawn_wall_from_to(Vec2::new(900.0, -1500.0), Vec2::new(1800.0, 150.0));
    builder.spawn_wall_from_to(Vec2::new(1300.0, -1500.0), Vec2::new(2000.0, 2500.0));
    builder.spawn_wall_from_to(Vec2::new(-1500.0, 800.0), Vec2::new(3000.0, 2500.0));

    builder.set_spawn_point_xy(300.0, 50.0, 0);

    builder.set_finish_point_xy(1050.0, 250.0);
}
