use std::sync::{atomic::AtomicU64, Arc};

use eframe::{
    egui_wgpu::{self, wgpu, RenderState},
    wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device, QuerySet},
};

use crate::ray_tracer::camera::{OUTPUT_TEXTURE_HEIGHT, OUTPUT_TEXTURE_WIDTH};

pub fn get_render_resources(wgpu_render_state: &RenderState) -> RenderResources {
    let device = &wgpu_render_state.device;
    let (compute_bind_group_layout, compute_bind_group, texture_view) = compute_bind_group(device);
    let (fragment_bind_group_layout, fragment_bind_group) =
        fragment_bind_group(device, &texture_view);
    let render_pipeline =
        get_render_pipeline(device, wgpu_render_state, &[&fragment_bind_group_layout]);
    let compute_pipeline = get_compute_pipeline(device, &[&compute_bind_group_layout]);
    let adapter = &wgpu_render_state.adapter;
    RenderResources {
        fragment_bind_group,
        render_pipeline,
        compute_bind_group,
        compute_pipeline,
        time_query: get_time_query(device, &adapter),
    }
}

fn fragment_bind_group(device: &Device, view: &wgpu::TextureView) -> (BindGroupLayout, BindGroup) {
    let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Sampler for the fragment texture"),
        mag_filter: wgpu::FilterMode::Nearest, // the way to scale up a pixel in the texture. Take the nearest pixel or linearly interpolate between pixels
        min_filter: wgpu::FilterMode::Nearest, // the way to scale up a pixel in the texture
        ..Default::default()
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Fragment bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("fragment bind group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture_sampler),
            },
        ],
    });
    (bind_group_layout, bind_group)
}

fn compute_bind_group(device: &Device) -> (BindGroupLayout, BindGroup, wgpu::TextureView) {
    let texture_format = wgpu::TextureFormat::Rgba8Unorm;
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        dimension: wgpu::TextureDimension::D2,
        format: texture_format,
        label: Some("Ray tracer output texture"),
        mip_level_count: 1,
        sample_count: 1,
        usage: wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::TEXTURE_BINDING,
        size: wgpu::Extent3d {
            width: OUTPUT_TEXTURE_WIDTH,
            height: OUTPUT_TEXTURE_HEIGHT,
            depth_or_array_layers: 1,
        },
        view_formats: &[],
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    // the bindgroup will only be used for the compute shader part
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Layout for the ray tracing output texture"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::StorageTexture {
                access: wgpu::StorageTextureAccess::WriteOnly,
                format: texture_format,
                view_dimension: wgpu::TextureViewDimension::D2,
            },
            count: None,
        }],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind group for the output texture"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&view), // when you send a texture to the gpu you only send the view to the texture
        }],
    });

    (bind_group_layout, bind_group, view)
}

/**
 * The query for a timestamp to see how long the compute shader lasts
 */
fn get_time_query(device: &Device, adapter: &wgpu::Adapter) -> Option<(QuerySet, Buffer, Buffer)> {
    let features = adapter.features();

    if !features.contains(wgpu::Features::TIMESTAMP_QUERY) {
        return None;
    }

    let read_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Buffer for copying and reading the time information"),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        contents: &[0; 16],
    });

    let write_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Buffer for querying the time"),
        usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
        contents: &[0; 16],
    });

    let query_set = device.create_query_set(&wgpu::QuerySetDescriptor {
        label: Some("Query set for a time stamp"),
        count: 2,
        ty: wgpu::QueryType::Timestamp,
    });

    Some((query_set, read_buffer, write_buffer))
}

fn get_compute_pipeline(
    device: &Device,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::ComputePipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("./raytracing.wgsl"));

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline layout"),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute pipeline"),
        entry_point: "compute_main",
        layout: Some(&layout),
        module: &shader,
    })
}

fn get_render_pipeline(
    device: &Device,
    render_state: &eframe::egui_wgpu::RenderState,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("./render.wgsl"));

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("The layout of the pipeline"),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vertex_main",
            buffers: &[], // the type of vertices that we want to pass to this vertex shader
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fragment_main",
            targets: &[Some(render_state.target_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}

pub struct RenderResources {
    render_pipeline: wgpu::RenderPipeline,
    compute_pipeline: wgpu::ComputePipeline,
    fragment_bind_group: wgpu::BindGroup,
    compute_bind_group: wgpu::BindGroup,
    time_query: Option<(wgpu::QuerySet, Buffer, Buffer)>,
}

pub struct RenderCallBack {
    pub render_time: Arc<AtomicU64>,
}

impl egui_wgpu::CallbackTrait for RenderCallBack {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &RenderResources = resources.get().unwrap();
        if let Some((query, _, _)) = &resources.time_query {
            // write the query before computing
            encoder.write_timestamp(query, 0);
        }

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute pass"),
            });
            compute_pass.set_pipeline(&resources.compute_pipeline);
            compute_pass.set_bind_group(0, &resources.compute_bind_group, &[]);
            compute_pass.dispatch_workgroups(
                OUTPUT_TEXTURE_WIDTH / 16,
                OUTPUT_TEXTURE_HEIGHT / 16,
                1,
            );
        }

        // well let's hope this works
        if let Some((query, _, _)) = &resources.time_query {
            encoder.write_timestamp(query, 1);
        }

        Vec::new()
    }

    fn finish_prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &RenderResources = resources.get().unwrap();

        // this doesn't have to be here and could could be done in the
        // reads the buffer and stores the render time
        // Since the render pass hasn't passed yet, it will read 0 for the first frame and then the previous frame render time
        if let Some((query, read_buffer, query_buffer)) = &resources.time_query {
            encoder.resolve_query_set(query, 0..2, &query_buffer, 0);
            encoder.copy_buffer_to_buffer(query_buffer, 0, read_buffer, 0, read_buffer.size());

            let buffer_slice = read_buffer.slice(..);
            buffer_slice.map_async(wgpu::MapMode::Read, |_| {});

            // wait for the map to be mapped
            device.poll(wgpu::Maintain::Wait);

            let period = queue.get_timestamp_period();
            let time_stamp_raw = buffer_slice.get_mapped_range();
            let time_stamp_data: &[u64] = bytemuck::cast_slice(&*time_stamp_raw);
            let time = (time_stamp_data[1] - time_stamp_data[0]) as f64 * period as f64 * 1e-6;
            self.render_time
                .store(time.to_bits(), std::sync::atomic::Ordering::SeqCst);

            drop(time_stamp_raw); // have to drop the view into the buffer before unmapping
            read_buffer.unmap();
        }
        Vec::new()
    }

    fn paint<'rp>(
        &'rp self,
        _info: eframe::epaint::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'rp>,
        callback_resources: &'rp egui_wgpu::CallbackResources,
    ) {
        let resources: &RenderResources = callback_resources.get().unwrap();

        render_pass.set_pipeline(&resources.render_pipeline);
        render_pass.set_bind_group(0, &resources.fragment_bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}
