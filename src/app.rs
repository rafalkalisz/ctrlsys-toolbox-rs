use eframe::egui;

use crate::{plot::bode::bode_plot, tf::{ctf::ContinousTransferFunction, TransferFunction}};

pub struct MainApp {
    tf: Box<dyn TransferFunction>,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            tf: Box::new(ContinousTransferFunction::from_numden(vec![1.0], vec![1.0, 1.0]))
        }
    }
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                })
            });
            ui.add_space(16.0);

            egui::widgets::global_theme_preference_buttons(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bode Plot");
            bode_plot(ui, self.tf.as_ref(), 0.0, 10.0, 100);
        });

    }
}



