use ctrlsys_toolbox_rs::app::MainApp;

fn main() -> eframe::Result {

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_fullscreen(true),
        ..Default::default()
    };

    eframe::run_native(
        "Control Systems Toolbox", 
        native_options,
        Box::new(|cc| Ok(Box::new(MainApp::new(cc))))
    )

}
