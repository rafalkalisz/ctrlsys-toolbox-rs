use crate::tf::{dtf::DiscreteTransferFunction, TransferFunction};

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

pub fn open_loop_response(tf: &DiscreteTransferFunction, response_type: ResponseType, count: usize) -> Vec<ResponsePoint> {

    let a = tf.denominator();
    let b = tf.numerator();

    if a.is_empty() || a[0].abs() < f64::EPSILON {
        return vec![]
    }
    
    let x = match response_type {
        ResponseType::Impulse => {
            let mut x_tmp = vec![0.0; count];
            x_tmp[0] = 1.0;
            x_tmp
        }
        ResponseType::Step => vec![1.0; count],
        ResponseType::Ramp => (0..count).map(|i| i as f64 * tf.sample_time()).collect(),
    };

    let mut y = vec![0.0; count];

    for n in 0..count {
        let forward = b.iter().enumerate().map(|(k, &bk)| {
            if n >= k { bk * x[n - k] } else { 0.0 }
        }).sum::<f64>();
        let feedback = a[1..].iter().enumerate().map(|(k, &ak)| {
            if n > k { ak * y[n - k - 1] } else { 0.0 }
        }).sum::<f64>();
        
        y[n] = (forward - feedback) / a[0]; // Normalize to denominator
    };

    (0..count).map(|i| ResponsePoint {
        time: i as f64 * tf.sample_time(),
        mag: y[i],
    }).collect()

}