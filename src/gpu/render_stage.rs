use eframe::egui_wgpu::wgpu::*;

use crate::ray_tracer::camera::{OUTPUT_TEXTURE_HEIGHT, OUTPUT_TEXTURE_WIDTH};

pub type RenderBindGroups = Vec<BindGroup>;
pub type RenderBindGroupLayout = Vec<BindGroupLayout>;

pub fn get_render_bind_group(
    device: &Device,
    view: &TextureView,
) -> (RenderBindGroupLayout, RenderBindGroups) {
    let texture_sampler = device.create_sampler(&SamplerDescriptor {
        label: Some("Sampler for the fragment texture"),
        mag_filter: FilterMode::Nearest, // the way to scale up a pixel in the texture. Take the nearest pixel or linearly interpolate between pixels
        min_filter: FilterMode::Nearest, // the way to scale up a pixel in the texture
        ..Default::default()
    });

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Fragment bind group layout"),
        entries: &[
            // texture
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT | ShaderStages::VERTEX,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // sampler
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("fragment bind group"),
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(&texture_sampler),
            },
        ],
    });
    (vec![bind_group_layout], vec![bind_group])
}

pub fn get_render_pipeline(
    device: &Device,
    render_state: &eframe::egui_wgpu::RenderState,
    bind_group_layouts: &[&BindGroupLayout],
) -> RenderPipeline {
    let shader = device.create_shader_module(include_wgsl!("../shaders/render.wgsl"));

    let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("The layout of the pipeline"),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vertex_main",
            buffers: &[], // the type of vertices that we want to pass to this vertex shader
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fragment_main",
            targets: &[Some(render_state.target_format.into())],
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        multiview: None,
    })
}

pub fn get_output_texture(device: &Device) -> (Texture, TextureView) {
    let texture_format = TextureFormat::Rgba8Unorm;
    let texture = device.create_texture(&TextureDescriptor {
        dimension: TextureDimension::D2,
        format: texture_format,
        label: Some("Ray tracer output texture"),
        mip_level_count: 1,
        sample_count: 1,
        usage: TextureUsages::COPY_DST
            | TextureUsages::STORAGE_BINDING
            | TextureUsages::TEXTURE_BINDING,
        size: Extent3d {
            width: OUTPUT_TEXTURE_WIDTH,
            height: OUTPUT_TEXTURE_HEIGHT,
            depth_or_array_layers: 1,
        },
        view_formats: &[],
    });
    let view = texture.create_view(&TextureViewDescriptor::default());
    (texture, view)
}
