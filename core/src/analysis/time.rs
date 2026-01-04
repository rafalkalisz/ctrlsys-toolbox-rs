use std::iter::Sum;

use num::Float;

use crate::tf::{TransferFunction, dtf::DiscreteTransferFunction};

#[derive(Copy, Clone, PartialEq)]
pub enum ResponseType {
    Impulse,
    Step,
    Ramp,
}

pub fn generate_input<T: Float>(response_type: ResponseType, count: usize, ts: T) -> Vec<T> {
    match response_type {
        ResponseType::Impulse => {
            let mut x_tmp = vec![T::zero(); count];
            x_tmp[0] = T::one();
            x_tmp
        }
        ResponseType::Step => vec![T::one(); count],
        ResponseType::Ramp => (0..count).map(|i| T::from(i).unwrap() * ts).collect(),
    }
}

pub struct ResponsePoint<T: Float> {
    pub time: T,
    pub mag: T,
}

pub struct ResponseSimulator<'a, T: Float> {
    dtf: &'a DiscreteTransferFunction<T>,
    input_state: Vec<T>,
    output_state: Vec<T>,
}

impl<'a, T: Float + Sum> ResponseSimulator<'a, T> {
    pub fn new(dtf: &'a DiscreteTransferFunction<T>) -> Self {
        Self {
            dtf,
            input_state: vec![T::zero(); dtf.numerator().len()],
            output_state: vec![T::zero(); dtf.denominator().len()],
        }
    }

    pub fn step(&mut self, input: T) -> T {
        self.input_state.rotate_right(1);
        self.input_state[0] = input;

        let forward = self.dtf.numerator()[..]
            .iter()
            .zip(self.input_state.iter())
            .map(|(&b, &x)| b * x)
            .sum::<T>();

        let feedback = self.dtf.denominator()[1..]
            .iter()
            .zip(self.output_state.iter())
            .map(|(&a, &y)| a * y)
            .sum::<T>();

        let output = (forward - feedback) / self.dtf.denominator()[0];

        self.output_state.rotate_right(1);
        if !self.output_state.is_empty() {
            self.output_state[0] = output;
        }

        output
    }

    pub fn get_response(
        &mut self,
        response_type: ResponseType,
        count: usize,
    ) -> Vec<ResponsePoint<T>> {
        let input = generate_input(response_type, count, self.dtf.sample_time());
        (0..count)
            .map(|i| ResponsePoint {
                time: T::from(i).unwrap() * self.dtf.sample_time(),
                mag: self.step(input[i]),
            })
            .collect()
    }

    pub fn reset(&mut self) {
        self.input_state.fill(T::zero());
        self.output_state.fill(T::zero());
    }
}
