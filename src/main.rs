mod app;
use eframe;
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        multisampling: 4,
        ..Default::default()
    };
    eframe::run_native(
        "some name",
        options,
        Box::new(|cc| Box::new(app::AppUI::new(cc))),
    )
}
