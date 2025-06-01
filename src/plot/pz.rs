use eframe::egui;
use egui_plot::{PlotItem, PlotPoint, Polygon, Text};
use num::complex::Complex64;

use crate::tf::TransferFunction;

pub fn pzplot(ui: &mut egui::Ui, tf: &dyn TransferFunction) {

    let poles = tf.poles();
    let zeroes = tf.zeroes();

    let pole_coords: Vec<[f64; 2]> = poles.iter().map(|complex| [complex.re, complex.im]).collect();
    let zero_coords: Vec<[f64; 2]> = zeroes.iter().map(|complex| [complex.re, complex.im]).collect();

    let all_points = poles.iter().chain(zeroes.iter());

    let (min_re, max_re, min_im, max_im) = all_points.fold(
        (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY),
        |(min_r, max_r, min_i, max_i), point| {
            (
                min_r.min(point.re),
                max_r.max(point.re),
                min_i.min(point.im),
                max_i.max(point.im),
            )
        }
    );

    let pole_points = egui_plot::Points::new("poles", pole_coords)
        .color(egui::Color32::MAGENTA)
        .radius(6.0)
        .shape(egui_plot::MarkerShape::Cross);

    let zero_points = egui_plot::Points::new("poles", zero_coords)
        .color(egui::Color32::CYAN)
        .radius(6.0)
        .shape(egui_plot::MarkerShape::Circle);

    let margin = 0.5;
    let x_min = min_re - margin;
    let x_max = (max_re + margin).max(margin); // Make sure the Y axis is visible
    let y_min = min_im - margin;
    let y_max = max_im + margin;

    egui_plot::Plot::new("pz_plot")
        .include_x(x_min)
        .include_x(x_max)
        .include_y(y_min)
        .include_y(y_max)
        .show(ui, |plot_ui| {
            plot_ui.points(pole_points);
            plot_ui.points(zero_points);
        });
}