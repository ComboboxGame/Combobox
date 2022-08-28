use crate::core::SceneBuilder;
use bevy::prelude::Color;

pub fn setup(builder: &mut SceneBuilder) {
    const INF: f32 = 40.0;

    builder.set_background_color(Color::rgb(0.03, 0.03, 0.03));

    // 1 -
    builder.spawn_wall_from_to_xy(-INF, 0.0, -INF, INF);
    builder.spawn_wall_from_to_xy(-INF, INF, -INF, 0.0);
    builder.spawn_wall_from_to_xy(-INF, 2.0, -INF, 3.0);
    builder.spawn_wall_from_to_xy(-INF, INF, 9.0, 11.0);
    builder.spawn_wall_from_to_xy(5.0, INF, 4.0, 6.0);
    builder.spawn_wall_from_to_xy(4.0, 5.0, 4.0, 4.5);
    builder.spawn_wall_from_to_xy(-INF, 7.0, 10.0, INF);
    builder.spawn_wall_from_to_xy(22.0, INF, 10.0, INF);
    builder.spawn_wall_from_to_xy(32.0, INF, -INF, INF);
    builder.spawn_wall_from_to_xy(23.0, INF, 5.0, 10.0);
}
