mod headlines;

use eframe::epaint::Vec2;
use headlines::{Headlines};

fn main() {
    tracing_subscriber::fmt::init();

    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540., 960.));

    eframe::run_native(
        "Headings",
        native_options,
        Box::new(|cc| {
            let mut app = Headlines::new(cc);
            app.config_fonts(&cc.egui_ctx);
            app.fetch_news();
            Box::new(app)
        }),
    )
}
