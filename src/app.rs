use eframe::{egui::{self, ComboBox}, App};

use crate::{filter::sallenkey::butterworth_poles, plot::{bode::{bode_mag_plot, bode_phase_plot}, pz::pzplot, text::{print_coeffs, tf_text}}, tf::{ctf::ContinousTransferFunction, dtf::DiscreteTransferFunction, traits::coeff_from_pz, TimeDomain, TransferFunction}, util::poly::reduce_to_real};

pub struct MainApp {
    ctf: ContinousTransferFunction,
    dtf: DiscreteTransferFunction,
    selected_time_domain: TimeDomain,
    tf_input: TfInput,
    ctf_input_order: usize,
    ctf_input_num: Vec<f64>,
    ctf_input_den: Vec<f64>,
    dtf_input_t_sample: f64,
    filter_input_type: FilterType,
    filter_input_order: usize,
    filter_input_cutoff: f64,
    filter_input_normalize: bool,
}

impl Default for MainApp {
    fn default() -> Self {
        let ctf_input_num = vec![0.0, 0.0, 0.0, 1.0];
        let ctf_input_den = vec![1.0, 2.0, 2.0, 1.0];
        let ctf = ContinousTransferFunction::from_numden(
            ctf_input_num.clone(), 
            ctf_input_den.clone(),
        );
        Self {
            ctf_input_order: ctf_input_num.len() - 1,
            dtf_input_t_sample: 1.0,
            dtf: DiscreteTransferFunction::from_ctf(&ctf, 1.0),
            selected_time_domain: TimeDomain::Continous,
            tf_input: TfInput::Continous,
            ctf_input_num,
            ctf_input_den,
            ctf,
            filter_input_type: FilterType::Butterworth,
            filter_input_order: 3,
            filter_input_cutoff: 1.0,
            filter_input_normalize: true,
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
        self.dtf = DiscreteTransferFunction::from_ctf(&self.ctf, self.dtf_input_t_sample);
    }

    pub fn handle_dtf_input(&mut self) {
        self.dtf = DiscreteTransferFunction::from_ctf(&self.ctf, self.dtf_input_t_sample);
    }

    fn handle_filter_input(&mut self) {
        let poles = match self.filter_input_type {
            FilterType::Butterworth => butterworth_poles(self.filter_input_order, self.filter_input_cutoff),
            _ => todo!("Not implemented yet")
        };
        let den = reduce_to_real(&coeff_from_pz(&poles));
        self.ctf_input_order = den.len().saturating_sub(1);
        let mut num = vec![0.0; self.ctf_input_order];
        num.push(1.0);
        self.ctf_input_num = num;
        self.ctf_input_den = den;
        self.ctf = ContinousTransferFunction::from_numden(
            trim_coeffs(self.ctf_input_num.clone()), 
            trim_coeffs(self.ctf_input_den.clone())
        );
        if self.filter_input_normalize {
            self.ctf.normalize_at_w(0.0); // TODO: w = 0 is good for LPF, but should be different for HPF/BPF
        }
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
                                bode_mag_plot(ui, &self.ctf, 0.0, 10.0, 1000);
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
                                bode_phase_plot(ui, &self.ctf, 0.0, 10.0, 1000);
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
            ui.label("Continuous transfer function input");
            ui.separator();
            continuous_tf_input(ui, app);
        }
        TfInput::Discrete => {
            ui.label("Discrete implementation of specified transfer function");
            ui.separator();
            discrete_tf_input(ui, app);
            ui.separator();
            ui.label("Numerator coefficients");
            ui.code(print_coeffs(app.dtf.numerator()));
            ui.label("Denominator coefficients");
            ui.code(print_coeffs(app.dtf.denominator()));

        }
        TfInput::Filter => {
            ui.label("Filter synthesis");
            ui.separator();
            filter_input(ui, app);
        }
    }
}

fn continuous_tf_input(ui: &mut egui::Ui, app: &mut MainApp) {
    ui.label("Transfer function order");
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

    ui.separator();
    ui.label("Transfer function equation:");
    let ctf_text = tf_text(app.ctf.numerator(), app.ctf.denominator());
    ui.monospace(ctf_text);


}

fn discrete_tf_input(ui: &mut egui::Ui, app: &mut MainApp) {
    ui.label("Sample time (T):");
    if ui.add(
        egui::DragValue::new(&mut app.dtf_input_t_sample)
            .speed(1e-3)
            .range(1e-6..=1.0)
            .prefix("T = ")
            .suffix(" s"),
    ).changed() {
        app.handle_dtf_input();
    }
}

#[derive(Debug, PartialEq)]
enum FilterType {
    Butterworth,
    Chebyshev,
    Elliptic,
}

fn filter_input(ui: &mut egui::Ui, app: &mut MainApp) {
    ui.horizontal(|ui| {
        ui.label("Filter type");
        ComboBox::from_id_salt("input_filter_type_select")
        .selected_text(format!("{:?}", app.filter_input_type))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut app.filter_input_type, FilterType::Butterworth, "Butterworth");
            ui.selectable_value(&mut app.filter_input_type, FilterType::Chebyshev, "Chebyshev (WIP)");
            ui.selectable_value(&mut app.filter_input_type, FilterType::Elliptic, "Elliptic (WIP)");
        })
    });

    ui.horizontal(|ui| {
        ui.label("Order:");
        if ui.add(egui::DragValue::new(&mut app.filter_input_order).range(1..=20)).changed() {
            app.handle_filter_input();
        }
    });

    ui.horizontal(|ui| {
        ui.label("Cutoff frequency (normalized):");
        if ui.add(egui::DragValue::new(&mut app.filter_input_cutoff).range(0.01..=1.0).speed(0.01)).changed() {
            app.handle_filter_input();
        }
    });

    ui.horizontal(|ui| {
        ui.label("Normalize filter gain");
        if ui.checkbox(&mut app.filter_input_normalize, "").changed() {
            app.handle_filter_input();
        }
    });


}





