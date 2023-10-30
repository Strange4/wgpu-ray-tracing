mod app;
mod gpu;
mod raytracing;

use eframe;
use eframe::egui_wgpu::wgpu;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    use std::sync::Arc;

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        multisampling: 1,
        wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
            device_descriptor: Arc::new(|_| wgpu::DeviceDescriptor {
                features: wgpu::Features::TIMESTAMP_QUERY,
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "real time ray tracer",
        options,
        Box::new(|cc| Box::new(app::AppUI::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(app::AppUI::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
