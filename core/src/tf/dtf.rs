use num::{Float, complex::Complex};

use super::{
    TimeDomain, TransferFunction,
    bilinear::bilinear_transform,
    ctf::{self, ContinousTransferFunction},
    traits::roots,
};

pub struct DiscreteTransferFunction<T: Float> {
    numerator: Vec<T>,
    denominator: Vec<T>,
    poles: Vec<Complex<T>>,
    zeroes: Vec<Complex<T>>,
    sample_time: T,
}

impl<T: Float> DiscreteTransferFunction<T> {
    pub fn from_numden(numerator: Vec<T>, denominator: Vec<T>, sample_time: T) -> Self {
        Self {
            poles: roots(&denominator),
            zeroes: roots(&numerator),
            numerator,
            denominator,
            sample_time,
        }
    }

    pub fn from_ctf(ctf: &ContinousTransferFunction<T>, sample_time: T) -> Self {
        let (numerator, denominator) =
            bilinear_transform(ctf.numerator(), ctf.denominator(), sample_time);
        Self::from_numden(numerator, denominator, sample_time)
    }

    pub fn sample_time(&self) -> T {
        self.sample_time
    }
}

impl<T: Float> TransferFunction<T> for DiscreteTransferFunction<T> {
    fn time_domain(&self) -> TimeDomain<T> {
        TimeDomain::Discrete {
            sample_time: self.sample_time,
        }
    }

    fn numerator(&self) -> &[T] {
        &self.numerator
    }

    fn denominator(&self) -> &[T] {
        &self.denominator
    }

    fn poles(&self) -> &[Complex<T>] {
        &self.poles
    }

    fn zeroes(&self) -> &[Complex<T>] {
        &self.zeroes
    }

    fn frequency_response(&self, omega: &[T]) -> Vec<Complex<T>> {
        todo!()
    }

    fn normalize_at_w(&mut self, w: T) {
        todo!()
    }
}
