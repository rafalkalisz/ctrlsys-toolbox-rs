use num::complex::Complex64;

use super::{TimeDomain, TransferFunction};

#[derive(Debug, Clone)]
pub struct ContinousTransferFunction {
    numerator: Vec<f64>,
    denominator: Vec<f64>,
}

impl ContinousTransferFunction {
    pub fn from_numden(numerator: Vec<f64>, denominator: Vec<f64>) -> Self {
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
    
    fn frequency_response(&self, omega_range: &[f64]) -> Vec<num::complex::Complex64> {
        omega_range.iter().map(|&w| {
            let s = Complex64::new(0.0, w);
            self.evaluate(s)
        }).collect()
    }
    
}

#[cfg(test)]
mod tests {
    use num::complex::Complex64;

    use super::*;

    #[test]
    fn test_evaluate() {
        // Given
        // H(s) = 1 / (s + 1), s = j
        let tf = ContinousTransferFunction::from_numden(vec![1.0], vec![1.0, 1.0]);
        let s = Complex64::new(0.0, 1.0);
        
        // When
        let result = tf.evaluate(s);
        
        // Then
        // H(s) = 1 / (1 + j)
        let expected = Complex64::new(1.0, 0.0) / Complex64::new(1.0, 1.0);
        assert_eq!(expected, result)
    }

    #[test]
    fn test_order() {
        // Given
        // H(s) = 1 / (s^2 + s + 1)
        let tf = ContinousTransferFunction::from_numden(vec![1.0], vec![1.0, 1.0, 1.0]);
        
        // When
        let order = tf.order();

        // Then
        assert_eq!(2, order)
    }

    #[test]
    fn test_order_empty_denominator() {
        // Given
        // H(s) = 1
        let tf = ContinousTransferFunction::from_numden(vec![1.0], vec![]);

        // When
        let order = tf.order();

        // Then
        assert_eq!(0, order)
    }
}
