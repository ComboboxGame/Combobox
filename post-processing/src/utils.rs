use bevy::core::{Pod, Zeroable};
use bevy::render::render_resource::{BufferUsages, BufferVec, FilterMode, Sampler};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use wgpu::{AddressMode, SamplerDescriptor};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ScreenVertex {
    pub position: [f32; 3],
}

pub fn create_default_quad(
    render_device: &RenderDevice,
    render_queue: &RenderQueue,
) -> BufferVec<ScreenVertex> {
    let mut screen_quad = BufferVec::new(BufferUsages::VERTEX);
    // Triangle 1
    screen_quad.push(ScreenVertex {
        position: [0.0, 0.0, 0.0],
    });
    screen_quad.push(ScreenVertex {
        position: [0.0, 1.0, 0.0],
    });
    screen_quad.push(ScreenVertex {
        position: [1.0, 1.0, 0.0],
    });
    // Triangle 2
    screen_quad.push(ScreenVertex {
        position: [0.0, 0.0, 0.0],
    });
    screen_quad.push(ScreenVertex {
        position: [1.0, 0.0, 0.0],
    });
    screen_quad.push(ScreenVertex {
        position: [1.0, 1.0, 0.0],
    });
    screen_quad.write_buffer(render_device, render_queue);
    return screen_quad;
}

pub fn create_default_sampler(render_device: &RenderDevice) -> Sampler {
    render_device.create_sampler(&SamplerDescriptor {
        label: Some("default_sampler"),
        mag_filter: FilterMode::Nearest,
        min_filter: FilterMode::Nearest,
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        ..Default::default()
    })
}

pub fn create_linear_sampler(render_device: &RenderDevice) -> Sampler {
    render_device.create_sampler(&SamplerDescriptor {
        label: Some("linear_sampler"),
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        ..Default::default()
    })
}
