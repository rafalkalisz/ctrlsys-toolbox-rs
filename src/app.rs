use eframe::{egui, App};

use crate::{plot::{bode::{bode_mag_plot, bode_phase_plot}, pz::pzplot}, tf::{ctf::ContinousTransferFunction, dtf::DiscreteTransferFunction, TimeDomain, TransferFunction}};

pub struct MainApp {
    ctf: ContinousTransferFunction,
    dtf: DiscreteTransferFunction,
    selected_time_domain: TimeDomain,
    tf_input: TfInput,
    ctf_input_order: usize,
    ctf_input_num: Vec<f64>,
    ctf_input_den: Vec<f64>,
    dtf_input_order: usize,
}

impl Default for MainApp {
    fn default() -> Self {
        let ctf = ContinousTransferFunction::from_numden(
            vec![1.0], 
            vec![1.0, 2.0, 1.0]
        );
        Self {
            dtf: DiscreteTransferFunction::from_ctf(&ctf, 1.0),
            ctf,
            selected_time_domain: TimeDomain::Continous,
            tf_input: TfInput::Continous,
            ctf_input_den: vec![0.0, 0.0, 0.0],
            ctf_input_num: vec![0.0, 0.0, 0.0],
            ctf_input_order: 2,
            dtf_input_order: 2,
        }
    }
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    pub fn handle_ctf_input(&mut self) {
        self.ctf = ContinousTransferFunction::from_numden(
            trim_coeffs(self.ctf_input_num.clone()), 
            trim_coeffs(self.ctf_input_den.clone())
        );
        self.dtf = DiscreteTransferFunction::from_ctf(&self.ctf, 1.0);
    }
}

fn trim_coeffs(coeffs: Vec<f64>) -> Vec<f64> {
    let first_nonzero = coeffs
        .iter()
        .position(|&c| c.abs() >= 1e-12)
        .unwrap_or(coeffs.len().saturating_sub(1));
    
    coeffs[first_nonzero..].to_vec()
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
                                tf_input(ui, self);
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

#[derive(Debug, PartialEq)]
enum TfInput {
    Continous,
    Discrete,
    Filter,
}

fn tf_input(ui: &mut egui::Ui, app: &mut MainApp) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut app.tf_input, TfInput::Continous, "Continuous transfer function");
        ui.selectable_value(&mut app.tf_input, TfInput::Discrete, "Discrete transfer function");
        ui.selectable_value(&mut app.tf_input, TfInput::Filter, "Filter synthesis");
    });
    
    ui.separator();

    match app.tf_input {
        TfInput::Continous => {
            ui.label("Enter continuous transfer function coefficients");
            continuous_tf_input(ui, app);
        }
        TfInput::Discrete => {
            ui.label("Enter continuous transfer function coefficients");
        }
        TfInput::Filter => {
            ui.label("Filter synthesis");
        }
    }
}

fn continuous_tf_input(ui: &mut egui::Ui, app: &mut MainApp) {
    ui.horizontal(|ui| {
        // --- Order Setter ---
        let mut order = app.ctf_input_order as u32;
        if ui.add(egui::Slider::new(&mut order, 0..=20).show_value(true).step_by(1.0)).changed() {
            app.ctf_input_order = order as usize;

            // Resize coefficient vectors (highest order first)
            app.ctf_input_num.resize(app.ctf_input_order + 1, 0.0);
            app.ctf_input_den.resize(app.ctf_input_order + 1, 0.0);
        }
    });

    ui.separator();
    ui.label("Numerator (highest order first):");
    ui.horizontal(|ui| {
        for i in 0..= app.ctf_input_order {
            let idx = app.ctf_input_order - i;
            ui.vertical(|ui| {
                ui.label(format!("b[{}]", idx));
                if ui.add(egui::DragValue::new(&mut app.ctf_input_num[i]).speed(0.1)).changed() {
                    app.handle_ctf_input();
                };
            });
        }
    });

    ui.separator();

    ui.label("Denominator (highest order first):");
    ui.horizontal(|ui| {
        for i in 0..= app.ctf_input_order {
            let idx = app.ctf_input_order - i;
            ui.vertical(|ui| {
                ui.label(format!("a[{}]", idx));
                if ui.add(egui::DragValue::new(&mut app.ctf_input_den[i]).speed(0.1)).changed() {
                    app.handle_ctf_input();
                };
            });
        }
    });


}





