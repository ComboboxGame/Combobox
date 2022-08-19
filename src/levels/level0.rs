use crate::core::MapBuilder;
use bevy::prelude::*;

pub fn setup_level0(builder: &mut MapBuilder, parent: &mut ChildBuilder) {
    builder.build_wall_from_to(parent, Vec2::new(-100.0, -20.0), Vec2::new(100.0, 0.0));
}
