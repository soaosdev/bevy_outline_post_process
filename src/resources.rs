use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    prelude::*,
    render::{
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            BindGroupLayout, BindGroupLayoutEntries, CachedRenderPipelineId, ColorTargetState,
            ColorWrites, FragmentState, MultisampleState, PipelineCache, PrimitiveState,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            TextureFormat, TextureSampleType,
        },
        renderer::RenderDevice,
    },
};

use super::components;

#[derive(Resource)]
pub struct OutlinePostProcessPipeline {
    pub layout: BindGroupLayout,
    pub screen_sampler: Sampler,
    pub normal_sampler: Sampler,
    pub depth_sampler: Sampler,
    pub pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for OutlinePostProcessPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "outline_post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    texture_2d(TextureSampleType::Depth),
                    sampler(SamplerBindingType::NonFiltering),
                    uniform_buffer::<components::OutlinePostProcessSettings>(false),
                ),
            ),
        );

        let screen_sampler = render_device.create_sampler(&SamplerDescriptor::default());
        let normal_sampler = render_device.create_sampler(&SamplerDescriptor::default());
        let depth_sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world.resource::<AssetServer>().load::<Shader>(
            "embedded://bevy_outline_post_process/../assets/shaders/outline_post_process.wgsl",
        );

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("outline_post_process_render_pipeline".into()),
                    layout: vec![layout.clone()],
                    push_constant_ranges: vec![],
                    vertex: fullscreen_shader_vertex_state(),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::bevy_default(),
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    zero_initialize_workgroup_memory: false,
                });

        Self {
            layout,
            screen_sampler,
            normal_sampler,
            depth_sampler,
            pipeline_id,
        }
    }
}
