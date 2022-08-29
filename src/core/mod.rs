use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod audio;
mod camera;
mod material;
mod scene_builder;
mod scene_objects;

pub use audio::*;
pub use camera::*;
pub use material::*;
pub use scene_builder::*;
pub use scene_objects::*;

use self::audio::AudioPlugin;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());

        #[cfg(debug_assertions)]
        app.add_plugin(RapierDebugRenderPlugin::default());

        app.init_resource::<SceneBoundaries>();

        app.add_plugin(CameraPlugin);
        app.add_plugin(AudioPlugin);
        app.add_plugin(SceneObjectsPlugin);

        app.insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 30.0,
                time_scale: 1.0,
                substeps: 1,
            },
            ..default()
        });
    }
}
