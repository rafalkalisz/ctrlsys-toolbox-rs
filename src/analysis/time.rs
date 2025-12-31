use crate::tf::{TransferFunction, dtf::DiscreteTransferFunction};

#[derive(Copy, Clone, PartialEq)]
pub enum ResponseType {
    Impulse,
    Step,
    Ramp,
}

pub struct ResponsePoint {
    pub time: f64,
    pub mag: f64,
}

pub fn generate_input(response_type: ResponseType, count: usize, ts: f64) -> Vec<f64> {
    match response_type {
        ResponseType::Impulse => {
            let mut x_tmp = vec![0.0; count];
            x_tmp[0] = 1.0;
            x_tmp
        }
        ResponseType::Step => vec![1.0; count],
        ResponseType::Ramp => (0..count).map(|i| i as f64 * ts).collect(),
    }
}

pub fn open_loop_response(
    tf: &DiscreteTransferFunction,
    response_type: ResponseType,
    count: usize,
) -> Vec<ResponsePoint> {
    let a = tf.denominator();
    let b = tf.numerator();

    if a.is_empty() || a[0].abs() < f64::EPSILON {
        return vec![];
    }

    let x = generate_input(response_type, count, tf.sample_time());
    let mut y = vec![0.0; count];

    for n in 0..count {
        let forward = b
            .iter()
            .take(n + 1) // b[k], k <= n
            .zip(x[..=n].iter().rev()) // x[n-k], k <= n
            .map(|(bk, xk)| bk * xk)
            .sum::<f64>();

        let feedback = a[1..]
            .iter()
            .take(n) // a[k], 1 <= k <= n
            .zip(y[..n].iter().rev()) // y[n-k], 1 <= k <= n
            .map(|(ak, yk)| ak * yk)
            .sum::<f64>();

        y[n] = (forward - feedback) / a[0]; // Normalize to denominator
    }

    (0..count)
        .map(|i| ResponsePoint {
            time: i as f64 * tf.sample_time(),
            mag: y[i],
        })
        .collect()
}

