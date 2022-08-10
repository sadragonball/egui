fn main() {
    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
        .or_else(|_| tracing_subscriber::EnvFilter::try_new("debug"))
        .unwrap();


    tracing_subscriber::fmt().with_env_filter(filter_layer).init();

    let options = eframe::NativeOptions {
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
