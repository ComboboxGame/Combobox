use bevy::prelude::*;
use bevy::render::camera::ExtractedCamera;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType};
use bevy::render::render_phase::TrackedRenderPass;
use bevy::render::render_resource::{
    BindGroupLayout, BindGroupLayoutEntry, BindingType, BlendState, BufferVec,
    CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, FrontFace,
    MultisampleState, PipelineCache, PolygonMode, PrimitiveState, RenderPipelineDescriptor,
    Sampler, SamplerBindingType, ShaderStages, TextureFormat, TextureSampleType,
    TextureViewDimension, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue};
use bevy::render::texture::BevyDefault;
use bevy::render::view::{ExtractedView, ViewTarget};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindingResource,
    Operations, RenderPassColorAttachment, RenderPassDescriptor,
};

use crate::CUSTOM_DEFAULT_VERT;
use crate::CUSTOM_TONEMAPPING;
use crate::utils::{create_default_quad, create_default_sampler, ScreenVertex};

pub struct ToneMappingNode {
    query: QueryState<
        (
            &'static ExtractedCamera,
            &'static ViewTarget,
            &'static Camera2d,
        ),
        With<ExtractedView>,
    >,
    screen_quad: BufferVec<ScreenVertex>,
    sampler: Sampler,
}

impl ToneMappingNode {
    pub const NAME: &'static str = "tone_mapping";
    pub const IN_VIEW: &'static str = "view";
    pub const IN_TEXTURE: &'static str = "tone_mapping_in_texture";

    pub fn new(world: &mut World) -> Self {
        let query = world.query_filtered();
        let render_device = world.resource::<RenderDevice>();
        let render_queue = world.resource::<RenderQueue>();

        Self {
            query,
            sampler: create_default_sampler(render_device),
            screen_quad: create_default_quad(render_device, render_queue),
        }
    }
}

impl Node for ToneMappingNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![
            SlotInfo::new(ToneMappingNode::IN_VIEW, SlotType::Entity),
            SlotInfo::new(ToneMappingNode::IN_TEXTURE, SlotType::TextureView),
        ]
    }

    fn update(&mut self, world: &mut World) {
        self.query.update_archetypes(world);
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let view_entity = graph.get_input_entity(Self::IN_VIEW)?;
        let input = graph.get_input_texture(Self::IN_TEXTURE)?;

        let (camera, target, _camera_2d) =
            if let Ok(result) = self.query.get_manual(world, view_entity) {
                result
            } else {
                return Ok(());
            };
        let pass_descriptor = RenderPassDescriptor {
            label: Some("tone_mapping_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &target.view,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
        };

        let render_pass = render_context
            .command_encoder
            .begin_render_pass(&pass_descriptor);

        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ToneMappingPipeline>();

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: Some("final_pass_bind_group"),
            layout: &pipeline.layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&self.sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(input),
                },
            ],
        });

        if let Some(pipeline) = pipeline_cache.get_render_pipeline(pipeline.pipeline) {
            let mut tracked_pass = TrackedRenderPass::new(render_pass);
            if let Some(viewport) = camera.viewport.as_ref() {
                tracked_pass.set_camera_viewport(viewport);
            }
            tracked_pass.set_render_pipeline(pipeline);
            tracked_pass.set_bind_group(0, &bind_group, &[]);
            tracked_pass.set_vertex_buffer(0, self.screen_quad.buffer().unwrap().slice(..));
            tracked_pass.draw(0..self.screen_quad.len() as u32, 0..1);
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct ToneMappingPipeline {
    pub layout: BindGroupLayout,
    pub pipeline: CachedRenderPipelineId,
}

impl FromWorld for ToneMappingPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>().clone();
        let layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("tone_mapping_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: TextureViewDimension::D2,
                        sample_type: TextureSampleType::Float { filterable: false },
                    },
                    count: None,
                },
            ],
        });

        let mut pipeline_cache = world.resource_mut::<PipelineCache>();

        let pipeline = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some("tone_mapping_pass".into()),
            vertex: VertexState {
                shader: CUSTOM_DEFAULT_VERT.typed(),
                entry_point: "main".into(),
                shader_defs: vec![],
                buffers: vec![VertexBufferLayout::from_vertex_formats(
                    VertexStepMode::Vertex,
                    vec![VertexFormat::Float32x3],
                )],
            },
            fragment: Some(FragmentState {
                shader: CUSTOM_TONEMAPPING.typed(),
                shader_defs: vec![],
                entry_point: "main".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            layout: Some(vec![layout.clone()]),
            primitive: PrimitiveState {
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        Self { layout, pipeline }
    }
}
