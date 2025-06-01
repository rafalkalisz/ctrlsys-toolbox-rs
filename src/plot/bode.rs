use eframe::egui;

use crate::{analysis::frequency::{bode_data, linspace, logspace}, tf::TransferFunction};

pub fn bode_mag_plot(ui: &mut egui::Ui, tf: &dyn TransferFunction, w_start: f64, w_stop: f64, points: usize) {
    let freq_range: Vec<f64> = linspace(w_start, w_stop, points);
    let bode_points = bode_data(tf, &freq_range);

    let mag_points: egui_plot::PlotPoints = bode_points.iter()
        .map(|point| [point.omega, point.mag_db])
        .collect();

    ui.label("Magnitude Response (dB)");
    egui_plot::Plot::new("bode_mag")
        .show(ui, |plot_ui| {
            plot_ui.line(egui_plot::Line::new("H(s)", mag_points));
        });
}

pub fn bode_phase_plot(ui: &mut egui::Ui, tf: &dyn TransferFunction, w_start: f64, w_stop: f64, points: usize) {
    let freq_range: Vec<f64> = linspace(w_start, w_stop, points);
    let bode_points = bode_data(tf, &freq_range);
    
    let phase_points: egui_plot::PlotPoints = bode_points.iter()
        .map(|point| [point.omega, point.phase_rad])
        .collect();

    ui.label("Phase Response (rad)");
    egui_plot::Plot::new("phase_mag")
        .show(ui, |plot_ui| {
            plot_ui.line(egui_plot::Line::new("H(s)", phase_points));
        });
}