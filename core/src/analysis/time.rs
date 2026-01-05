use std::iter::Sum;

use num::Float;

use crate::tf::{TransferFunction, dtf::DiscreteTransferFunction};

#[derive(Clone, Copy, PartialEq)]
pub enum ResponseType {
    Impulse,
    Step,
    Ramp,
}

impl ResponseType {
    pub fn generate_input<T: Float>(self, count: usize, ts: T) -> Vec<T> {
        match self {
            ResponseType::Impulse => {
                let mut x_tmp = vec![T::zero(); count];
                x_tmp[0] = T::one();
                x_tmp
            }
            ResponseType::Step => vec![T::one(); count],
            ResponseType::Ramp => (0..count).map(|i| T::from(i).unwrap() * ts).collect(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ResponsePoint<T: Float> {
    pub time: T,
    pub mag: T,
}

pub trait LTIResponse<T: Float> {
    fn step(&mut self, input: T) -> T;
    fn simulate(&mut self, t_end: T) -> Vec<ResponsePoint<T>>;
    fn reset(&mut self);
}

pub struct OpenLoopResponse<'a, T: Float> {
    tf: &'a DiscreteTransferFunction<T>,
    response_type: ResponseType,
    input_state: Vec<T>,
    output_state: Vec<T>,
}

impl<'a, T: Float> OpenLoopResponse<'a, T> {
    pub fn new(tf: &'a DiscreteTransferFunction<T>, response_type: ResponseType) -> Self {
        Self {
            tf,
            response_type,
            input_state: vec![T::zero(); tf.numerator().len()],
            output_state: vec![T::zero(); tf.numerator().len()],
        }
    }
}

impl<'a, T: Float + Sum> LTIResponse<T> for OpenLoopResponse<'a, T> {
    fn step(&mut self, input: T) -> T {
        self.input_state.rotate_right(1);
        self.input_state[0] = input;

        let forward = self.tf.numerator()[..]
            .iter()
            .zip(self.input_state.iter())
            .map(|(&b, &x)| b * x)
            .sum::<T>();

        let feedback = self.tf.denominator()[1..]
            .iter()
            .zip(self.output_state.iter())
            .map(|(&a, &y)| a * y)
            .sum::<T>();

        let output = (forward - feedback) / self.tf.denominator()[0];

        self.output_state.rotate_right(1);
        if !self.output_state.is_empty() {
            self.output_state[0] = output;
        }

        output
    }

    fn reset(&mut self) {
        self.input_state.fill(T::zero());
        self.output_state.fill(T::zero());
    }

    fn simulate(&mut self, t_end: T) -> Vec<ResponsePoint<T>> {
        self.reset();
        let t_end = t_end.max(T::zero());
        // TODO: add proper handling
        let count = (t_end / self.tf.sample_time()).to_usize().unwrap() + 1;
        let input = self
            .response_type
            .generate_input(count, self.tf.sample_time());
        (0..count)
            .map(|i| ResponsePoint {
                time: T::from(i).unwrap() * self.tf.sample_time(),
                mag: self.step(input[i]),
            })
            .collect()
    }
}
