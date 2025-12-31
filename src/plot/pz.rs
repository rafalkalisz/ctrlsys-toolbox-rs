use eframe::egui::{self, Color32};

use crate::tf::{TimeDomain, TransferFunction};

pub fn pzplot(ui: &mut egui::Ui, tf: &dyn TransferFunction<f64>) {
    let poles = tf.poles();
    let zeroes = tf.zeroes();

    let pole_coords: Vec<[f64; 2]> = poles
        .iter()
        .map(|complex| [complex.re, complex.im])
        .collect();
    let zero_coords: Vec<[f64; 2]> = zeroes
        .iter()
        .map(|complex| [complex.re, complex.im])
        .collect();

    let pole_points = egui_plot::Points::new("poles", pole_coords)
        .color(egui::Color32::MAGENTA)
        .radius(6.0)
        .shape(egui_plot::MarkerShape::Cross);

    let zero_points = egui_plot::Points::new("poles", zero_coords)
        .color(egui::Color32::CYAN)
        .radius(6.0)
        .shape(egui_plot::MarkerShape::Circle);

    egui_plot::Plot::new("pz_plot")
        .data_aspect(1.0)
        .show(ui, |plot_ui| {
            plot_ui.points(pole_points);
            plot_ui.points(zero_points);
            if let TimeDomain::Discrete { sample_time: _ } = tf.time_domain() {
                let num_points = 200;
                let unit_circle_points: egui_plot::PlotPoints = (0..=num_points)
                    .map(|i| {
                        let theta = i as f64 * std::f64::consts::TAU / num_points as f64;
                        [theta.cos(), theta.sin()]
                    })
                    .collect();

                let circle_line = egui_plot::Line::new("Discrete unit circle", unit_circle_points)
                    .color(Color32::LIGHT_GRAY);

                plot_ui.line(circle_line);
            }
        });
}

