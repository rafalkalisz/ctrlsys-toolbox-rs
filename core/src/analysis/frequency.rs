use num::Float;

use crate::tf::TransferFunction;

#[derive(Debug, Clone, Copy)]
pub struct BodePoint<T: Float> {
    pub omega: T,
    pub mag_db: T,
    pub phase_rad: T,
}

pub fn bode_data<T: Float>(tf: &dyn TransferFunction<T>, omega: &[T]) -> Vec<BodePoint<T>> {
    tf.frequency_response(omega)
        .iter()
        .zip(omega.iter())
        .map(|(h, &w)| BodePoint {
            omega: w,
            mag_db: T::from(20.0).unwrap() * h.norm().log10(),
            phase_rad: h.arg(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::tf::ctf::ContinousTransferFunction;
    use approx::assert_relative_eq;
    use std::f64::consts::FRAC_PI_4;

    use super::*;

    #[test]
    fn test_bode_data() {
        // Given
        // H(s) = 1 / (s + 1) (1st-order low-pass filter)
        // omega = 1 rad/s
        let tf = ContinousTransferFunction::<f64>::from_numden(&[1.0], &[1.0, 1.0]);
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
