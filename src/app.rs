use crate::raytracing::TriangleInfo;
use eframe::egui::Vec2;
use eframe::egui_wgpu;
use eframe::{egui, Frame};

use crate::gpu::{self, RenderCallBack};

pub struct AppUI {
    triangle_info: TriangleInfo,
}

impl AppUI {
    pub fn new(eframe_context: &eframe::CreationContext) -> Self {
        let wgpu_render_state = eframe_context.wgpu_render_state.as_ref().unwrap();
        let triangle_info = TriangleInfo { angle: 0.0 };
        let resources = gpu::get_render_resources(wgpu_render_state, &triangle_info);
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(resources);
        return AppUI { triangle_info };
    }

    fn ray_tracer_ui(&mut self, ui: &mut egui::Ui) -> Vec2 {
        let size = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::drag());
        self.triangle_info.angle += response.drag_delta().x * 0.01;

        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            RenderCallBack {
                triangle_info: self.triangle_info,
            },
        ));
        size
    }
}

impl eframe::App for AppUI {
    fn update(&mut self, context: &egui::Context, _frame: &mut Frame) {
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
                });
        });
    }
}
