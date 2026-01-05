use eframe::egui;

use ctrlsys_toolbox_core::analysis::time::LTIResponse;

// TODO: move to core/analysis/time?
const MAX_POINTS: usize = 100_000;

pub fn response_plot(ui: &mut egui::Ui, response: &mut dyn LTIResponse<f64>, t_end: f64) {
    let response_points: Vec<[f64; 2]> = response
        .simulate(t_end)
        .iter()
        .map(|point| [point.time, point.mag])
        .collect();

    egui_plot::Plot::new("impulse").show(ui, |plot_ui| {
        plot_ui.line(egui_plot::Line::new("H(s)", response_points));
    });
}
