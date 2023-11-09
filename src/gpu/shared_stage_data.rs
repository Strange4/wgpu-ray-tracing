use eframe::wgpu::*;

pub type SharedStageBindGroup = Vec<BindGroup>;
pub type SharedStageBindGroupLayout = Vec<BindGroupLayout>;

pub struct SharedStageData {
    pub size_update_buffer: Buffer,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SharedStageUniform {
    pub size: [f32; 2],
}

pub fn get_shared_data(device: &Device) -> SharedStageData {
    let size_update_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("The buffer containing information for the fragment stange"),
        size: std::mem::size_of::<SharedStageUniform>() as u64,
        mapped_at_creation: false,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });
    SharedStageData { size_update_buffer }
}

pub fn get_shared_stage_bind_group(
    device: &Device,
    shared_stage_data: &SharedStageData,
) -> (SharedStageBindGroupLayout, SharedStageBindGroup) {
    let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Shared stage bind group layout"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("Shared stage bind group"),
        layout: &layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: shared_stage_data.size_update_buffer.as_entire_binding(),
        }],
    });

    (vec![layout], vec![group])
}
