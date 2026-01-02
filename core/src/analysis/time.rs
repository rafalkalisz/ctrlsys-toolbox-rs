use std::iter::Sum;

use num::Float;

use crate::tf::{TransferFunction, dtf::DiscreteTransferFunction};

#[derive(Copy, Clone, PartialEq)]
pub enum ResponseType {
    Impulse,
    Step,
    Ramp,
}

pub struct ResponsePoint<T: Float> {
    pub time: T,
    pub mag: T,
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

pub fn open_loop_response<T>(
    tf: &DiscreteTransferFunction<T>,
    response_type: ResponseType,
    count: usize,
) -> Vec<ResponsePoint<T>>
where
    T: Float + Sum,
{
    let a = tf.denominator();
    let b = tf.numerator();

    if a.is_empty() || a[0].abs() < T::epsilon() {
        return vec![];
    }

    let x = generate_input(response_type, count, tf.sample_time());
    let mut y = vec![T::zero(); count];

    for n in 0..count {
        let forward = b
            .iter()
            .take(n + 1) // b[k], k <= n
            .zip(x[..=n].iter().rev()) // x[n-k], k <= n
            .map(|(&bk, &xk)| bk * xk)
            .sum::<T>();

        let feedback = a[1..]
            .iter()
            .take(n) // a[k], 1 <= k <= n
            .zip(y[..n].iter().rev()) // y[n-k], 1 <= k <= n
            .map(|(&ak, &yk)| ak * yk)
            .sum::<T>();

        y[n] = (forward - feedback) / a[0]; // Normalize to denominator
    }

    (0..count)
        .map(|i| ResponsePoint {
            time: T::from(i).unwrap() * tf.sample_time(),
            mag: y[i],
        })
        .collect()
}
