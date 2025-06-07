use eframe::egui;

use crate::{plot::{bode::{bode_mag_plot, bode_phase_plot}, pz::pzplot}, tf::{ctf::ContinousTransferFunction, dtf::DiscreteTransferFunction, TimeDomain, TransferFunction}};

pub struct MainApp {
    ctf: ContinousTransferFunction,
    dtf: DiscreteTransferFunction,
    selected_time_domain: TimeDomain,
}

impl Default for MainApp {
    fn default() -> Self {
        let ctf = ContinousTransferFunction::from_numden(
            vec![1.0], 
            vec![1.0, 2.0, 2.0, 1.0]
        );
        Self {
            dtf: DiscreteTransferFunction::from_ctf(&ctf, 1.0),
            ctf,
            selected_time_domain: TimeDomain::Continous,
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
                                bode_mag_plot(ui, &self.ctf, 0.0, 10.0, 100);
                            });
                        });
                        row.col(|ui| {
                            ui.group(|ui| {
                                ui.heading("Pole-Zero Plot");
                                domain_switch(ui, &mut self.selected_time_domain);
                                match self.selected_time_domain {
                                    TimeDomain::Continous => pzplot(ui, &self.ctf),
                                    TimeDomain::Discrete { sample_time: _ } => pzplot(ui, &self.dtf),
                                }
                            });
                        });
                    });
                    body.row(480.0, |mut row| {
                        row.col(|ui| {
                            ui.group(|ui| {
                                ui.heading("Bode Plot: Phase");
                                bode_phase_plot(ui, &self.ctf, 0.0, 10.0, 100);
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

fn domain_switch(ui: &mut egui::Ui, selected: &mut TimeDomain) {
    ui.horizontal(|ui| {
        ui.label("Select domain");
        egui::ComboBox::from_id_salt("pzplot_domain_switch")
            .selected_text(match selected {
                TimeDomain::Continous => "Continous",
                TimeDomain::Discrete { sample_time: _ } => "Discrete",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(selected, TimeDomain::Continous, "Continuous");
                ui.selectable_value(selected, TimeDomain::Discrete { sample_time: 0.0 }, "Discrete");
            });
    });
}


