
fn main() {
    println!("Hello, world!");
    tracing_subscriber::fmt::init();
    let options = eframe::NativeOptions{
        drag_and_drop_support: true,
        initial_window_size: Some([1280., 720.].into()),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "volume renderer",
        options,
        Box::new(|cc| Box::new(app::Application::new(cc))),
    );
}
