use bevy::prelude::*;
use bevy::render::render_resource::{ShaderType, UniformBuffer};
use bevy::render::Extract;
use bevy::render::renderer::{RenderDevice, RenderQueue};

#[derive(Component, Debug)]
pub struct PointLight2d {
    pub radius: f32,
    pub color: Color,
}

#[derive(ShaderType, Debug, Default)]
pub struct PointLightsUniform {
    pub lights_num: u32,
    pub positions: [Vec4; 32],
    pub colors: [Vec4; 32],
}

pub fn extract_point_lights_2d(
    point_lights: Extract<Query<(&PointLight2d, &GlobalTransform)>>,
    mut point_lights_uniform: ResMut<UniformBuffer<PointLightsUniform>>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    let mut uniform = PointLightsUniform::default();

    for (i, (point_light, transform)) in point_lights.iter().enumerate() {
        uniform.lights_num += 1;
        uniform.positions[i] = Vec4::new(transform.translation().x, transform.translation().y, transform.translation().y, point_light.radius);
        uniform.colors[i] = point_light.color.into();
    }

    point_lights_uniform.set(uniform);
    point_lights_uniform.write_buffer(&render_device, &render_queue);
}