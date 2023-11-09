mod compute_stage;
mod render_stage;
mod renderer;
mod shared_stage_data;

pub use renderer::RenderCallBack;

use eframe::egui_wgpu::wgpu::*;
use eframe::wgpu::util::DeviceExt;

use renderer::RenderResources;

use self::compute_stage::{get_compute_bind_group, get_compute_pipeline};
use self::render_stage::{get_output_texture, get_render_bind_group, get_render_pipeline};
use self::shared_stage_data::{get_shared_data, get_shared_stage_bind_group};

pub fn get_render_resources(wgpu_render_state: &eframe::egui_wgpu::RenderState) -> RenderResources {
    let device = &wgpu_render_state.device;

    let shared_stage_data = get_shared_data(device);

    let (shared_bind_group_layouts, shared_stage_bind_groups) =
        get_shared_stage_bind_group(device, &shared_stage_data);

    let (texture, texture_view) = get_output_texture(device);

    let (compute_bind_group_layouts, compute_bind_groups) =
        get_compute_bind_group(device, &texture_view, texture.format());

    let (render_bind_group_layouts, render_bind_groups) =
        get_render_bind_group(device, &texture_view);
    let render_pipeline = get_render_pipeline(
        device,
        wgpu_render_state,
        &concat_bind_group_layouts(&render_bind_group_layouts, &shared_bind_group_layouts),
    );
    let compute_pipeline = get_compute_pipeline(
        device,
        &concat_bind_group_layouts(&compute_bind_group_layouts, &shared_bind_group_layouts),
    );
    let adapter = &wgpu_render_state.adapter;
    RenderResources {
        render_pipeline,
        render_bind_groups,
        compute_bind_groups,
        compute_pipeline,
        shared_stage_data,
        shared_stage_bind_groups,
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

fn concat_bind_group_layouts<'a>(
    layout1: &'a Vec<BindGroupLayout>,
    layout2: &'a Vec<BindGroupLayout>,
) -> Vec<&'a BindGroupLayout> {
    let mut refs1: Vec<&BindGroupLayout> = layout1.iter().map(|layout| layout).collect();
    let mut refs2: Vec<&BindGroupLayout> = layout2.iter().map(|layout| layout).collect();
    refs1.append(&mut refs2);

    refs1
}
