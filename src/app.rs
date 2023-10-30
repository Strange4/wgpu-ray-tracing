use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use eframe::egui::Vec2;
use eframe::egui_wgpu;
use eframe::{egui, Frame};

use crate::gpu::{self, RenderCallBack};

pub struct AppUI {
    render_time: Arc<AtomicU64>,
}

impl AppUI {
    pub fn new(eframe_context: &eframe::CreationContext) -> Self {
        let wgpu_render_state = eframe_context.wgpu_render_state.as_ref().unwrap();
        let resources = gpu::get_render_resources(wgpu_render_state);
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(resources);
        return AppUI {
            render_time: Arc::new(AtomicU64::new((f64::NAN).to_bits())),
        };
    }

    fn ray_tracer_ui(&mut self, ui: &mut egui::Ui) -> Vec2 {
        let size = ui.available_size();
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::focusable_noninteractive());

        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            RenderCallBack {
                render_time: self.render_time.clone(),
            },
        ));
        size
    }
}

impl eframe::App for AppUI {
    fn update(&mut self, context: &egui::Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(context, |ui| {
            let mut size = Vec2::new(0.0, 0.0);
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                size = self.ray_tracer_ui(ui);
            });
            egui::Window::new("Info")
                // .anchor(egui::Align2::CENTER_TOP, (0.0, 0.0))
                .default_size((100.0, 100.0))
                .show(context, |ui| {
                    ui.label(format!("Here is the size: {:?}", size));
                    let shader_time = f64::from_bits(self.render_time.load(Ordering::Relaxed));
                    ui.label(format!("Shader run time: {:.3} ms", shader_time));

                    if let Some(usage) = frame.info().cpu_usage {
                        ui.label(format!("egui render time: {:.3} ms", usage * 1000.0));
                    }
                });
        });
    }
}
