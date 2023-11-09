use std::sync::{atomic::AtomicU64, Arc};

use eframe::egui::Vec2;
use eframe::egui_wgpu::{self, wgpu::*};

use super::compute_stage::ComputeBindGroup;
use super::render_stage::{RenderBindGroup, RenderBuffers};
use super::shared_stage_data::{SharedStageData, SharedStageUniform};

pub struct RenderResources {
    pub render_pipeline: RenderPipeline,
    pub compute_pipeline: ComputePipeline,
    pub render_bind_groups: RenderBindGroup,
    pub render_buffers: RenderBuffers,
    pub compute_bind_groups: ComputeBindGroup,
    pub shared_stage_data: SharedStageData,
    pub time_query: Option<(QuerySet, Buffer, Buffer)>,
}

pub struct RenderCallBack {
    pub render_time: Arc<AtomicU64>,
    pub output_size: Vec2,
}

impl egui_wgpu::CallbackTrait for RenderCallBack {
    fn prepare(
        &self,
        _device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<CommandBuffer> {
        let resources: &RenderResources = resources.get().unwrap();
        if let Some((query, _, _)) = &resources.time_query {
            // write the query before computing
            encoder.write_timestamp(query, 0);
        }
        queue.write_buffer(
            &resources.shared_stage_data.size_update_buffer,
            0,
            bytemuck::cast_slice(&[SharedStageUniform {
                size: self.output_size.into(),
            }]),
        );
        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("Compute pass"),
            });
            // since the workgroups are 16x16, then the pixels are divided by 16;
            let width = (self.output_size.x / 16.0) as u32;
            let height = (self.output_size.y / 16.0) as u32;
            compute_pass.set_pipeline(&resources.compute_pipeline);
            resources.compute_bind_groups.iter().enumerate().for_each(
                |(index, compute_bind_group)| {
                    compute_pass.set_bind_group(index as u32, compute_bind_group, &[]);
                },
            );

            compute_pass.dispatch_workgroups(width, height, 1);
        }

        // well let's hope this works
        if let Some((query, _, _)) = &resources.time_query {
            encoder.write_timestamp(query, 1);
        }

        Vec::new()
    }

    fn finish_prepare(
        &self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<CommandBuffer> {
        let resources: &RenderResources = resources.get().unwrap();

        // this doesn't have to be here and could could be done in the
        // reads the buffer and stores the render time
        // Since the render pass hasn't passed yet, it will read 0 for the first frame and then the previous frame render time
        if let Some((query, read_buffer, query_buffer)) = &resources.time_query {
            encoder.resolve_query_set(query, 0..2, &query_buffer, 0);
            encoder.copy_buffer_to_buffer(query_buffer, 0, read_buffer, 0, read_buffer.size());

            let buffer_slice = read_buffer.slice(..);
            buffer_slice.map_async(MapMode::Read, |_| {});

            // wait for the map to be mapped
            device.poll(Maintain::Wait);

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
        render_pass: &mut RenderPass<'rp>,
        callback_resources: &'rp egui_wgpu::CallbackResources,
    ) {
        let resources: &RenderResources = callback_resources.get().unwrap();

        render_pass.set_pipeline(&resources.render_pipeline);
        resources
            .render_bind_groups
            .iter()
            .enumerate()
            .for_each(|(index, render_bind_group)| {
                render_pass.set_bind_group(index as u32, render_bind_group, &[]);
            });
        render_pass.draw(0..6, 0..1);
    }
}
