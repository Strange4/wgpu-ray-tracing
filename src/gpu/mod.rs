mod compute_stage;
mod render_stage;
mod renderer;
mod shared_stage_data;

pub use renderer::RenderCallBack;

use eframe::egui_wgpu::wgpu::*;
use eframe::wgpu::util::DeviceExt;

use renderer::RenderResources;

use self::compute_stage::{get_compute_pipeline, get_ray_tracing_bind_group};
use self::render_stage::{get_output_texture, get_render_bind_group, get_render_pipeline};
use self::shared_stage_data::get_shared_data;

pub fn get_render_resources(wgpu_render_state: &eframe::egui_wgpu::RenderState) -> RenderResources {
    let device = &wgpu_render_state.device;

    let shared_stage_data = get_shared_data(device);

    let (texture, texture_view) = get_output_texture(device);

    let (compute_bind_group_layout, compute_bind_group) =
        get_ray_tracing_bind_group(device, &texture_view, texture.format());

    let (output_bind_group_layout, output_bind_group, output_buffers) =
        get_render_bind_group(device, &texture_view, &shared_stage_data);

    let render_pipeline = get_render_pipeline(device, wgpu_render_state, &output_bind_group_layout);
    let compute_pipeline = get_compute_pipeline(device, &compute_bind_group_layout);
    let adapter = &wgpu_render_state.adapter;
    RenderResources {
        render_pipeline,
        render_bind_groups: output_bind_group,
        compute_bind_groups: compute_bind_group,
        compute_pipeline,
        render_buffers: output_buffers,
        shared_stage_data,
        time_query: get_time_query(device, &adapter),
    }
}

/**
 * The query for a timestamp to see how long the compute shader lasts
 */
fn get_time_query(device: &Device, adapter: &Adapter) -> Option<(QuerySet, Buffer, Buffer)> {
    let features = adapter.features();

    if !features.contains(Features::TIMESTAMP_QUERY) {
        return None;
    }

    let read_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
        label: Some("Buffer for copying and reading the time information"),
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        contents: &[0; 16],
    });

    let write_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
        label: Some("Buffer for querying the time"),
        usage: BufferUsages::QUERY_RESOLVE | BufferUsages::COPY_SRC,
        contents: &[0; 16],
    });

    let query_set = device.create_query_set(&QuerySetDescriptor {
        label: Some("Query set for a time stamp"),
        count: 2,
        ty: QueryType::Timestamp,
    });

    Some((query_set, read_buffer, write_buffer))
}
