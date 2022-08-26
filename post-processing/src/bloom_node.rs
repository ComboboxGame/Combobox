use bevy::prelude::*;
use bevy::render::camera::ExtractedCamera;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType};
use bevy::render::render_phase::{TrackedRenderPass};
use bevy::render::render_resource::{
    BindGroupLayout, BindGroupLayoutEntry, BindingType, BlendState, BufferBindingType, BufferVec, CachedRenderPipelineId, ColorTargetState, ColorWrites,
    FragmentState, FrontFace, MultisampleState, PipelineCache, PolygonMode, PrimitiveState,
    RenderPipelineDescriptor, Sampler, SamplerBindingType, ShaderStages, TextureDimension,
    TextureFormat, TextureSampleType, TextureUsages, TextureView, TextureViewDimension,
    UniformBuffer, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue};
use bevy::render::texture::{CachedTexture, TextureCache};
use bevy::render::view::{ExtractedView};
use bevy::utils::HashMap;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindingResource, Extent3d, Operations, RenderPassColorAttachment, RenderPassDescriptor, TextureDescriptor,
};
use crate::CUSTOM_DOWNSCALING;
use crate::CUSTOM_UPSCALING;
use crate::CUSTOM_DEFAULT_VERT;
use crate::utils::{
    create_default_quad, create_linear_sampler, ScreenVertex,
};

pub struct BloomNode {
    query: QueryState<
        (
            &'static ExtractedCamera,
            &'static BloomTargets,
            &'static Camera2d,
        ),
        With<ExtractedView>,
    >,
    screen_quad: BufferVec<ScreenVertex>,
    sampler: Sampler,
}

impl BloomNode {
    pub const NAME: &'static str = "bloom_node";
    pub const IN_VIEW: &'static str = "view";
    pub const IN_TEXTURE: &'static str = "bloom_in_texture";
    pub const OUT_TEXTURE: &'static str = "bloom_out_texture";

    pub fn new(world: &mut World) -> Self {
        let query = world.query_filtered();
        let render_device = world.resource::<RenderDevice>();
        let render_queue = world.resource::<RenderQueue>();
        Self {
            query,
            sampler: create_linear_sampler(render_device),
            screen_quad: create_default_quad(render_device, render_queue),
        }
    }
}

impl Node for BloomNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![
            SlotInfo::new(BloomNode::IN_VIEW, SlotType::Entity),
            SlotInfo::new(BloomNode::IN_TEXTURE, SlotType::TextureView),
        ]
    }

    fn output(&self) -> Vec<SlotInfo> {
        vec![SlotInfo::new(BloomNode::OUT_TEXTURE, SlotType::TextureView)]
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

        let (_camera, bloom_targets, _camera_2d) =
            if let Ok(result) = self.query.get_manual(world, view_entity) {
                result
            } else {
                graph.set_output(Self::OUT_TEXTURE, input.clone()).unwrap();
                return Ok(());
            };

        let render_device = world.resource::<RenderDevice>();
        let render_queue = world.resource::<RenderQueue>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<BloomPipeline>();

        let mut previous = input.clone();

        // Down sampling passes
        for i in 1..bloom_targets.mips {
            let slot = if i + 1 == bloom_targets.mips { 1 } else { 0 };

            let mut uniform = UniformBuffer::from(UVec4::new(i as u32, 0,0,0));
            uniform.write_buffer(render_device, render_queue);

            let pass_descriptor = RenderPassDescriptor {
                label: Some("bloom_downscaling_pass"),
                color_attachments: &[Some(bloom_targets.get_color_attachment(slot, i))],
                depth_stencil_attachment: None,
            };

            let render_pass = render_context
                .command_encoder
                .begin_render_pass(&pass_descriptor);

            let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
                label: Some("bloom_downscaling_pass_bind_group"),
                layout: &pipeline.downscaling_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::Sampler(&self.sampler),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::TextureView(&previous.clone()),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: uniform.binding().unwrap(),
                    },
                ],
            });

            if let Some(pipeline) =
            pipeline_cache.get_render_pipeline(pipeline.downscaling_pipeline)
            {
                let mut tracked_pass = TrackedRenderPass::new(render_pass);
                tracked_pass.set_render_pipeline(pipeline);
                tracked_pass.set_bind_group(0, &bind_group, &[]);
                tracked_pass.set_vertex_buffer(0, self.screen_quad.buffer().unwrap().slice(..));
                tracked_pass.draw(0..self.screen_quad.len() as u32, 0..1);
            }

            previous = bloom_targets.get_view(0, i);
        }

        // Up sampling + blur
        for i in (1..bloom_targets.mips).rev() {
            let pass_descriptor = RenderPassDescriptor {
                label: Some("bloom_up_scaling_pass"),
                color_attachments: &[Some(bloom_targets.get_color_attachment(1, i - 1))],
                depth_stencil_attachment: None,
            };

            let mut uniform = UniformBuffer::from(UVec4::new(i as u32, 0,0,0));
            uniform.write_buffer(render_device, render_queue);

            let render_pass = render_context
                .command_encoder
                .begin_render_pass(&pass_descriptor);

            let second = if i - 1 == 0 {
                input.clone()
            } else {
                bloom_targets.get_view(0, i - 1)
            };

            let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
                label: Some("bloom_up_scaling_pass_bind_group"),
                layout: &pipeline.upscaling_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::Sampler(&self.sampler),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::TextureView(&bloom_targets.get_view(1, i)),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::TextureView(&second),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: uniform.binding().unwrap(),
                    },
                ],
            });

            if let Some(pipeline) =
            pipeline_cache.get_render_pipeline(pipeline.upscaling_pipeline)
            {
                let mut tracked_pass = TrackedRenderPass::new(render_pass);
                tracked_pass.set_render_pipeline(pipeline);
                tracked_pass.set_bind_group(0, &bind_group, &[]);
                tracked_pass.set_vertex_buffer(0, self.screen_quad.buffer().unwrap().slice(..));
                tracked_pass.draw(0..self.screen_quad.len() as u32, 0..1);
            }
        }

        graph.set_output(Self::OUT_TEXTURE, bloom_targets.get_view(1, 0)).unwrap();

        Ok(())
    }
}

#[derive(Clone)]
pub struct BloomPipeline {
    pub upscaling_layout: BindGroupLayout,
    pub upscaling_pipeline: CachedRenderPipelineId,
    pub downscaling_layout: BindGroupLayout,
    pub downscaling_pipeline: CachedRenderPipelineId,
}

impl FromWorld for BloomPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>().clone();

        let downscaling_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("downscaling_layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 3,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: std::num::NonZeroU64::new(16),
                        },
                        visibility: ShaderStages::FRAGMENT,
                        count: None,
                    },
                ],
            });

        let mut pipeline_cache = world.resource_mut::<PipelineCache>();
        let downscaling_pipeline =
            pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("downscaling_pass".into()),
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
                    shader: CUSTOM_DOWNSCALING.typed(),
                    shader_defs: vec![],
                    entry_point: "main".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::Rg11b10Float,
                        blend: Some(BlendState::ALPHA_BLENDING),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                layout: Some(vec![downscaling_layout.clone()]),
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

        let upscaling_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("upscaling_layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 3,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: std::num::NonZeroU64::new(16),
                        },
                        visibility: ShaderStages::FRAGMENT,
                        count: None,
                    },
                ],
            });

        let mut pipeline_cache = world.resource_mut::<PipelineCache>();
        let upscaling_pipeline = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some("upscaling_pass".into()),
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
                shader: CUSTOM_UPSCALING.typed(),
                shader_defs: vec![],
                entry_point: "main".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rg11b10Float,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            layout: Some(vec![upscaling_layout.clone()]),
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

        Self {
            upscaling_layout,
            upscaling_pipeline,
            downscaling_layout,
            downscaling_pipeline,
        }
    }
}

#[derive(Component)]
pub struct BloomTargets {
    pub textures: [Vec<CachedTexture>; 2],
    pub mips: u32,
}

impl BloomTargets {
    pub fn get_color_attachment(&self, slot: usize, index: u32) -> RenderPassColorAttachment {
        RenderPassColorAttachment {
            view: &self.textures[slot][index as usize].default_view,
            resolve_target: None,
            ops: Operations::default(),
        }
    }

    pub fn get_view(&self, slot: usize, index: u32) -> TextureView {
        self.textures[slot][index as usize].default_view.clone()
    }
}

pub fn prepare_bloom_targets(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut texture_cache: ResMut<TextureCache>,
    cameras: Query<(Entity, &ExtractedCamera)>,
) {
    const MIPS: u32 = 4;

    let mut textures_map = HashMap::default();

    for (entity, camera) in &cameras {
        if let Some(target_size) = camera.physical_target_size.clone() {
            let mut textures_arr = vec![];

            for j in 0..2 {
                textures_arr.push((0..MIPS).map(|i| {
                    textures_map
                        .entry((camera.target.clone(), i, j))
                        .or_insert_with(|| {
                            texture_cache.get(
                                &render_device,
                                TextureDescriptor {
                                    label: if i == 0 {
                                        Some("bloom_0")
                                    } else {
                                        Some("bloom_1")
                                    },
                                    size: Extent3d {
                                        width: target_size.x >> i,
                                        height: target_size.y >> i,
                                        depth_or_array_layers: 1,
                                    },
                                    mip_level_count: 1,
                                    sample_count: 1,
                                    dimension: TextureDimension::D2,
                                    format: TextureFormat::Rg11b10Float,
                                    usage: TextureUsages::RENDER_ATTACHMENT
                                        | TextureUsages::TEXTURE_BINDING,
                                },
                            )
                        }).clone()
                }).collect::<Vec<_>>());
            }

            commands.entity(entity).insert(BloomTargets {
                textures: [textures_arr[0].clone(), textures_arr[1].clone()],
                mips: MIPS,
            });
        }
    }
}
