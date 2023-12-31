use eframe::wgpu::*;

pub type ComputeBindGroups = Vec<BindGroup>;
pub type ComputeBindGroupLayout = Vec<BindGroupLayout>;

/**
 * it returns the bindgroup layout, the texture view needed for the fragment pass
 * and an array containing the indices of all the shared bind group entries
 */
pub fn get_compute_bind_group(
    device: &Device,
    view: &TextureView,
    texture_format: TextureFormat,
) -> (ComputeBindGroupLayout, ComputeBindGroups) {
    // the bindgroup will only be used for the compute shader part
    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Layout for the compute bind group"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::WriteOnly,
                format: texture_format,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        }],
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("Bind group for the compute bind group"),
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(view), // when you send a texture to the gpu you only send the view to the texture
        }],
    });

    (vec![bind_group_layout], vec![bind_group])
}

pub fn get_compute_pipeline(
    device: &Device,
    bind_group_layouts: &[&BindGroupLayout],
) -> ComputePipeline {
    let shader = device.create_shader_module(include_wgsl!("../shaders/raytracing.wgsl"));

    let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Compute Pipeline layout"),
        bind_group_layouts,
        push_constant_ranges: &[],
    });

    device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: Some("Compute pipeline"),
        entry_point: "compute_main",
        layout: Some(&layout),
        module: &shader,
    })
}
