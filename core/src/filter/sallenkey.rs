use std::f64::consts::{FRAC_PI_2, PI};

use num::{complex::Complex64};

pub fn butterworth_poles(n: usize, omega_c: f64) -> Vec<Complex64> {

    let mut poles = Vec::with_capacity(n);
    for k in 0..n {
        let theta = FRAC_PI_2 + (2 * k + 1) as f64 * PI / (2 * n) as f64; 
        poles.push(Complex64::from_polar(omega_c, theta))
    }
    poles

}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use num::Complex;

    use super::butterworth_poles;


    #[test]
    fn test_butterworth_poles() {

        let poles = butterworth_poles(3, 1.0);

        assert_eq!(poles.len(), 3);

        // Optionally check approximate locations
        let expected_angles: Vec<f64> = vec![120.0, 180.0, 240.0]
            .into_iter()
            .map(|deg: f64| deg.to_radians())
            .collect();

        for (pole, &theta) in poles.iter().zip(expected_angles.iter()) {
            let expected = Complex::from_polar(1.0, theta);
            assert_relative_eq!(pole.re, expected.re, epsilon = 1e-12);
            assert_relative_eq!(pole.im, expected.im, epsilon = 1e-12);
        }
    }
}