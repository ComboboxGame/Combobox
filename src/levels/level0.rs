use crate::core::{Combobox, ComboboxType, MapBuilder};
use bevy::prelude::*;

pub fn setup_level0(builder: &mut MapBuilder) {
    builder.spawn_wall_from_to(Vec2::new(-1000.0, -20.0), Vec2::new(1000.0, 0.0));

    for i in 0..1 {
        builder.spawn_box(
            Combobox::new(50.0, ComboboxType::Standard { group: 0 }),
            Vec2::new(-600.0 + i as f32 * 80., 40.0),
        );
    }

    builder.spawn_box(
        Combobox::new(50.0, ComboboxType::Buf),
        Vec2::new(-100.0, 40.0),
    );   
    
    builder.spawn_box(
        Combobox::new(50.0, ComboboxType::Undo),
        Vec2::new(100.0, 40.0),
    ); 
}
