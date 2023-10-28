use crate::raytracing::TriangleInfo;
use eframe::{
    egui_wgpu::{self, wgpu, RenderState},
    wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device},
};

pub fn get_render_resources(
    wgpu_render_state: &RenderState,
    triangle_info: &TriangleInfo,
) -> RenderResources {
    let device = &wgpu_render_state.device;
    let (bind_group_layout, bind_group, triangle_uniform_buffer) =
        get_angle_uniform_buffer(triangle_info, device);
    let pipeline = get_pipeline(device, wgpu_render_state, &[&bind_group_layout]);
    RenderResources {
        bind_group,
        pipeline,
        uniform_buffer: triangle_uniform_buffer,
    }
}

fn get_angle_uniform_buffer(
    triangle_info: &TriangleInfo,
    device: &Device,
) -> (BindGroupLayout, BindGroup, Buffer) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind group layout for the angle of the trianlge"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Buffer for the triangle info"),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        contents: bytemuck::cast_slice(&[*triangle_info]),
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("The bind group the for triangle info"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
    });
    (bind_group_layout, bind_group, uniform_buffer)
}

fn get_pipeline(
    device: &Device,
    render_state: &eframe::egui_wgpu::RenderState,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::include_wgsl!("./shader.wgsl"));

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
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, // vertices should be interpreted as a triangle list. Also useful when you want to render trinagles next to each other
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw, // the front face of a triangle is counter clockwise
            cull_mode: None, // culling means that when the meshes are backward they aren't included in the rendering.
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}

pub struct RenderResources {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
}

pub struct RenderCallBack {
    pub triangle_info: TriangleInfo,
}

impl egui_wgpu::CallbackTrait for RenderCallBack {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &RenderResources = resources.get().unwrap();
        queue.write_buffer(
            &resources.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.triangle_info]),
        );
        Vec::new()
    }

    fn paint<'rp>(
        &'rp self,
        info: eframe::epaint::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'rp>,
        callback_resources: &'rp egui_wgpu::CallbackResources,
    ) {
        let resources: &RenderResources = callback_resources.get().unwrap();
        render_pass.set_pipeline(&resources.pipeline);
        render_pass.set_bind_group(0, &resources.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
