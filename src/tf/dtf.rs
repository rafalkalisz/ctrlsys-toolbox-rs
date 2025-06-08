use num::complex::Complex64;

use super::{bilinear::bilinear_transform, ctf::{self, ContinousTransferFunction}, traits::roots, TimeDomain, TransferFunction};

pub struct DiscreteTransferFunction {
    numerator: Vec<f64>,
    denominator: Vec<f64>,
    poles: Vec<Complex64>,
    zeroes: Vec<Complex64>,
    sample_time: f64,
}

impl DiscreteTransferFunction {

    pub fn from_numden(numerator: Vec<f64>, denominator: Vec<f64>, sample_time: f64) -> Self {
        Self {
            poles: roots(&denominator),
            zeroes: roots(&numerator),
            numerator,
            denominator,
            sample_time,
        }
    }

    pub fn from_ctf(ctf: &ContinousTransferFunction, sample_time: f64) -> Self {
        let (numerator, denominator) = bilinear_transform(ctf.numerator(), ctf.denominator(), sample_time);
        Self::from_numden(numerator, denominator, sample_time)
    }

}

impl TransferFunction for DiscreteTransferFunction {
    fn time_domain(&self) -> TimeDomain {
        TimeDomain::Discrete { sample_time: self.sample_time }
    }

    fn numerator(&self) -> &[f64] {
        &self.numerator
    }

    fn denominator(&self) -> &[f64] {
        &self.denominator
    }

    fn poles(&self) -> &[Complex64] {
        &self.poles
    }

    fn zeroes(&self) -> &[Complex64] {
        &self.zeroes
    }

    fn frequency_response(&self, omega: &[f64]) -> Vec<Complex64> {
        todo!()
    }
    
    fn normalize_at_w(&mut self, w: f64) {
        todo!()
    }
}