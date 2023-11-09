use eframe::wgpu::{Buffer, BufferDescriptor, BufferUsages, Device};

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
