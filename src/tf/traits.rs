use ndarray::Array2;
use ndarray_linalg::Eig;
use num::complex::Complex64;

use crate::util::poly::convolve;

#[derive(Debug, PartialEq)]
pub enum TimeDomain {
    Continous,
    Discrete { sample_time: f64 }
}

pub trait TransferFunction {

    fn time_domain(&self) -> TimeDomain;

    fn numerator(&self) -> &[f64];
    fn denominator(&self) -> &[f64];

    fn poles(&self) -> &[Complex64];
    fn zeroes(&self) -> &[Complex64];

    fn frequency_response(&self, omega: &[f64]) -> Vec<Complex64>;
    
    fn evaluate(&self, eval_point: Complex64) -> Complex64 {
        let horner = |coeffs: &[f64]| {
            coeffs.iter().fold(Complex64::new(0.0, 0.0), |acc, coeff| acc * eval_point + coeff)
        };
        horner(self.numerator()) / horner(self.denominator())
    }

    fn order(&self) -> usize {
        self.denominator().len().saturating_sub(1)
    }

}

// Based on MATLAB roots algorithm (see https://www.mathworks.com/help/matlab/ref/roots.html)
// TODO: Should this be in TF trait file?
pub fn roots(coeffs: &[f64]) -> Vec<Complex64> {

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
        if i < n-1 {
            companion_matrix[(i + 1, i)] = 1.0;
        }
    }

    // Eigenvalues of companion matrix == roots
    match companion_matrix.eig() {
        Ok((eigenvalues, _)) => eigenvalues.to_vec(),
        Err(_) => vec![],
    }
} 


pub fn coeff_from_pz(p_or_z: &[Complex64]) -> Vec<Complex64> {

    p_or_z.iter().fold(
        vec![Complex64::new(1.0, 0.0)], 
        |acc, &pz| convolve(&acc, &vec![Complex64::new(1.0, 0.0), pz])
    )
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

