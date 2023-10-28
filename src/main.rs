mod app;
mod gpu;
mod raytracing;
use eframe;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        multisampling: 1,
        ..Default::default()
    };

    eframe::run_native(
        "some name",
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
