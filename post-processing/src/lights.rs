use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;

use crate::ColorMaterialCustom;

#[derive(Component, Debug, Default)]
pub struct PointLight2d {
    pub radius: f32,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct AmbientLight {
    pub color: Color,
}

#[derive(ShaderType, Clone, Debug, Default)]
pub struct PointLightsUniform {
    pub lights_num: u32,
    pub positions: [Vec4; 16],
    pub colors: [Vec4; 16],
    pub ambient: Vec4,
}

pub fn update_lights(
    lights_query: Query<(&PointLight2d, &GlobalTransform)>,
    handles: Query<&Handle<ColorMaterialCustom>>,
    mut materials: ResMut<Assets<ColorMaterialCustom>>,
    ambient: Res<AmbientLight>,
) {
    let mut lights = PointLightsUniform::default();

    lights.ambient = ambient.color.as_linear_rgba_f32().into();

    for (point_light, transform) in lights_query.iter() {
        if point_light.radius < 0.01 {
            continue;
        }

        let i = lights.lights_num as usize;
        lights.positions[i] = Vec4::new(transform.translation().x, transform.translation().y, transform.translation().z, point_light.radius);
        lights.colors[i] = point_light.color.as_linear_rgba_f32().into();
        lights.lights_num += 1;
    }

    for handle in handles.iter() {
        materials.get_mut(handle).unwrap().lights = lights.clone();
    }
}
