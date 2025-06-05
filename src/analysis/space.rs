pub fn linspace(start: f64, stop: f64, count: usize) -> Vec<f64> {
    let spacing = (stop - start) / ((count - 1) as f64);
    (0..count).map(|n| start + (n as f64) * spacing).collect()
} 

pub fn logspace(start_exp: f64, stop_exp: f64, count: usize) -> Vec<f64> {
    linspace(start_exp, stop_exp, count).iter().map(|&n| 10.0_f64.powf(n)).collect()
}