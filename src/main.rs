use ctrlsys_toolbox_rs::app::MainApp;

fn main() -> eframe::Result {

    eframe::run_native(
        "Control Systems Toolbox", 
        eframe::NativeOptions::default(), 
        Box::new(|cc| Ok(Box::new(MainApp::new(cc))))
    )

}
