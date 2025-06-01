use eframe::egui;

use crate::{plot::{bode::{bode_mag_plot, bode_phase_plot}, pz::pzplot}, tf::{ctf::ContinousTransferFunction, TransferFunction}};

pub struct MainApp {
    tf: Box<dyn TransferFunction>,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            tf: Box::new(ContinousTransferFunction::from_numden(vec![1.0, 1.0], vec![1.0, 5.0, 6.0]))
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

            egui::widgets::global_theme_preference_buttons(ui);
        });


        egui::CentralPanel::default().show(ctx, |ui| {

            egui_extras::TableBuilder::new(ui)
                .columns(egui_extras::Column::remainder(), 2)
                .striped(true)
                .body(|mut body| {
                    body.row(480.0, |mut row| {
                        row.col(|ui| {
                            ui.group(|ui| {
                                ui.heading("Bode Plot: Magnitude");
                                bode_mag_plot(ui, self.tf.as_ref(), 0.0, 10.0, 100);
                            });
                        });
                        row.col(|ui| {
                            ui.group(|ui| {
                                ui.heading("Pole-Zero Plot");
                                pzplot(ui, self.tf.as_ref());
                            });
                        });
                    });
                    body.row(480.0, |mut row| {
                        row.col(|ui| {
                            ui.group(|ui| {
                                ui.heading("Bode Plot: Phase");
                                bode_phase_plot(ui, self.tf.as_ref(), 0.0, 10.0, 100);
                            });
                        });
                        row.col(|ui| {
                            ui.group(|ui| {
                                ui.label("Impulse response placeholder");
                            });
                        });
                    });
                    body.row(480.0, |mut row| {
                        row.col(|ui| {
                            ui.group(|ui| {
                                ui.label("Transfer function input placeholder");
                            });
                        });
                        row.col(|ui| {
                            ui.group(|ui| {
                                ui.label("PID controller input placeholder");
                            });
                        });
                    });
                });

        });

    }
}



