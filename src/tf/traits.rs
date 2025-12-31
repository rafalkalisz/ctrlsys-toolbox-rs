use ndarray::Array2;
use ndarray_linalg::Eig;
use num::{Complex, Float};

use crate::util::poly::convolve;

#[derive(Debug, PartialEq)]
pub enum TimeDomain<T: Float> {
    Continous,
    Discrete { sample_time: T },
}

pub trait TransferFunction<T: Float> {
    fn time_domain(&self) -> TimeDomain<T>;

    fn numerator(&self) -> &[T];
    fn denominator(&self) -> &[T];

    fn poles(&self) -> &[Complex<T>];
    fn zeroes(&self) -> &[Complex<T>];

    fn frequency_response(&self, omega: &[T]) -> Vec<Complex<T>>;

    fn evaluate(&self, eval_point: Complex<T>) -> Complex<T> {
        let horner = |coeffs: &[T]| {
            coeffs
                .iter()
                .fold(Complex::<T>::new(T::zero(), T::zero()), |acc, coeff| {
                    acc * eval_point + coeff
                })
        };
        horner(self.numerator()) / horner(self.denominator())
    }

    fn order(&self) -> usize {
        self.denominator().len().saturating_sub(1)
    }

    fn normalize_at_w(&mut self, w: T);
}

// Based on MATLAB roots algorithm (see https://www.mathworks.com/help/matlab/ref/roots.html)
// Kept as f64 to avoid propagating Lapack trait requirements
// TODO: Should this be in TF trait file?
fn roots_f64(coeffs: &[f64]) -> Vec<Complex<f64>> {
    if coeffs.len() <= 1 {
        return Vec::new();
    }

    // Make coeffs monic (normalize to highest-order coefficient)
    let monic_coeffs: Vec<f64> = coeffs.iter().map(|coeff| coeff / coeffs[0]).collect();

    // Construct companion matrix
    let n = coeffs.len() - 1;
    let mut companion_matrix = Array2::<f64>::zeros((n, n));
    for i in 0..n {
        // First row: negative coefficients from n-1 to 0
        companion_matrix[(0, i)] = -monic_coeffs[i + 1];
        // Remaining rows: subdiagonal matrix
        if i < n - 1 {
            companion_matrix[(i + 1, i)] = 1.0;
        }
    }

    // Eigenvalues of companion matrix == roots
    match companion_matrix.eig() {
        Ok((eigenvalues, _)) => eigenvalues.to_vec(),
        Err(_) => vec![],
    }
}

pub fn roots<T: Float>(coeffs: &[T]) -> Vec<Complex<T>> {
    let coeffs_f64: Vec<f64> = coeffs.iter().map(|&x| x.to_f64().unwrap()).collect();
    roots_f64(&coeffs_f64)
        .into_iter()
        .map(|c| Complex::new(T::from(c.re).unwrap(), T::from(c.im).unwrap()))
        .collect()
}

pub fn coeff_from_pz<T: Float>(p_or_z: &[Complex<T>]) -> Vec<Complex<T>> {
    p_or_z
        .iter()
        .fold(vec![Complex::<T>::new(T::one(), T::zero())], |acc, &pz| {
            let poly = vec![Complex::<T>::new(T::one(), T::zero()), -pz];
            convolve(&acc, &poly)
        })
}

#[cfg(test)]
mod tests {
    use super::coeff_from_pz;
    use num::complex::Complex64;

    #[test]
    fn test_coeff_from_pz() {
        let z1 = Complex64::new(-1.0, 2.0);
        let z2 = Complex64::new(-1.0, -2.0);
        println!("{:?}", coeff_from_pz(&vec![z1, z2]));
    }
}
