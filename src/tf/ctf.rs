use super::{TimeDomain, TransferFunction};

#[derive(Debug, Clone)]
pub struct ContinousTransferFunction {
    numerator: Vec<f64>,
    denominator: Vec<f64>,
}

impl ContinousTransferFunction {
    fn from_numden(numerator: Vec<f64>, denominator: Vec<f64>) -> Self {
        Self { numerator, denominator }
    }
}

impl TransferFunction for ContinousTransferFunction {
    fn time_domain(&self) -> super::TimeDomain {
        TimeDomain::Continous
    }

    fn numerator(&self) -> &[f64] {
        &self.numerator
    }

    fn denominator(&self) -> &[f64] {
        &self.denominator
    }
}