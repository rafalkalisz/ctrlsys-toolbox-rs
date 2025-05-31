use num::complex::Complex64;


pub enum TimeDomain {
    Continous,
    Discrete { sample_time: f64 }
}

pub trait TransferFunction {

    fn time_domain(&self) -> TimeDomain;
    fn numerator(&self) -> &[f64];
    fn denominator(&self) -> &[f64];

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



