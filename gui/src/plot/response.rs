use eframe::egui;

use ctrlsys_toolbox_core::{
    analysis::time::{ResponseSimulator, ResponseType},
    tf::dtf::DiscreteTransferFunction,
};

const MAX_POINTS: usize = 100_000;

pub fn open_loop_response_plot(
    ui: &mut egui::Ui,
    tf: &DiscreteTransferFunction<f64>,
    response_type: ResponseType,
    t_end: f64,
) {
    let point_count = (t_end / tf.sample_time()) as usize + 1;
    if point_count > MAX_POINTS {
        return;
    }

    let mut simulator = ResponseSimulator::new(tf);
    let response_points: Vec<[f64; 2]> = simulator
        .get_response(response_type, point_count)
        .iter()
        .map(|point| [point.time, point.mag])
        .collect();

    egui_plot::Plot::new("impulse").show(ui, |plot_ui| {
        plot_ui.line(egui_plot::Line::new("H(s)", response_points));
    });
}
