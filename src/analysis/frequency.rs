use crate::tf::TransferFunction;

#[derive(Debug, Clone, Copy)]
pub struct BodePoint {
    pub omega: f64,
    pub mag_db: f64,
    pub phase_rad: f64,
}

pub fn bode_data(tf: &dyn TransferFunction, omega: &[f64]) -> Vec<BodePoint> {
    tf.frequency_response(omega)
        .iter()
        .zip(omega.iter())
        .map(|(h, &w)| BodePoint {
            omega: w,
            mag_db: 20.0 * h.norm().log10(),
            phase_rad: h.arg(),
        })
        .collect()
}

pub fn linspace(start: f64, stop: f64, count: usize) -> Vec<f64> {
    let spacing = (stop - start) / ((count - 1) as f64);
    (0..count).map(|n| start + (n as f64) * spacing).collect()
} 

pub fn logspace(start_exp: f64, stop_exp: f64, count: usize) -> Vec<f64> {
    linspace(start_exp, stop_exp, count).iter().map(|&n| 10.0_f64.powf(n)).collect()
}



#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_4;
    use approx::assert_relative_eq;
    use crate::tf::ctf::ContinousTransferFunction;

    use super::*;

    #[test]
    fn test_linpace() {
        // Given
        let (start, stop, count) = (100.0, 500.0, 5);

        // When
        let result = linspace(start, stop, count);

        // Then
        let expected = vec![100.0, 200.0, 300.0, 400.0, 500.0];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_logspace() {
        // Given
        let (start_exp, stop_exp, count) = (0.0, 5.0, 6);

        // When
        let result = logspace(start_exp, stop_exp, count);

        // Then
        let expected = vec![1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_bode_data() {
        // Given
        // H(s) = 1 / (s + 1) (1st-order low-pass filter)
        // omega = 1 rad/s
        let tf = ContinousTransferFunction::from_numden(vec![1.0], vec![1.0, 1.0]);
        let omega = vec![1.0];

        // When
        let bode_points = bode_data(&tf, &omega);
        let cutoff = bode_points[0];

        // Then
        // Magnitude ~= 20log_10(1/sqrt(2)) ~= -3.01
        // Phase ~= -pi/4 = -45 degrees
        assert_relative_eq!(-3.01, cutoff.mag_db, epsilon = 1e-3);
        assert_relative_eq!(-FRAC_PI_4, cutoff.phase_rad, epsilon = 1e-2)
    }
}