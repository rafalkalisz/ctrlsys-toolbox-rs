use num::complex::Complex64;

use super::{traits::roots, TimeDomain, TransferFunction};

#[derive(Debug, Clone)]
pub struct ContinousTransferFunction {
    numerator: Vec<f64>,
    denominator: Vec<f64>,
    poles: Vec<Complex64>,
    zeroes: Vec<Complex64>,
}

impl ContinousTransferFunction {
    pub fn from_numden(numerator: Vec<f64>, denominator: Vec<f64>) -> Self {
        Self {
            poles: roots(&denominator),
            zeroes: roots(&numerator),
            numerator, 
            denominator,
        }
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
    
    fn poles(&self) -> &[Complex64] {
        &self.poles
    }
    
    fn zeroes(&self) -> &[Complex64] {
        &self.zeroes
    }
    
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
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

    #[test]
    fn test_poles_zeroes() {
        // Given
        // H(s) = (s + 1) / (s + 2)(s + 3) = (s + 1) / (s^2 + 5s + 6)
        let tf = ContinousTransferFunction::from_numden(vec![1.0, 1.0], vec![1.0, 5.0, 6.0]);

        // When
        let zeroes = tf.zeroes();
        let poles = tf.poles();
    
        // Then
        // Numerator: (s + 1) => zeroes: -1
        assert_eq!(1, zeroes.len());
        assert_relative_eq!(-1.0, zeroes[0].re);
        assert_relative_eq!(0.0, zeroes[0].im);
        // Denominator: (s + 2)(s + 3) => poles: -2, -3
        // TODO: Asserted root order in Vec determined experimentally when using ndarray_linalg::eig()
        //       Unit test should be independent of the chosen algorithm implementation
        assert_eq!(2, poles.len());
        assert_relative_eq!(-3.0, poles[0].re);
        assert_relative_eq!(0.0, poles[0].im);
        assert_relative_eq!(-2.0, poles[1].re);
        assert_relative_eq!(0.0, poles[1].im);
        

    }
}
