use crate::util::poly::{binomial_expansion, convolve, poly_add};


pub fn bilinear_transform(num_s: &[f64], den_s: &[f64], t_sample: f64) -> (Vec<f64>, Vec<f64>) {

    let m = num_s.len() - 1;
    let n = den_s.len() - 1;
    let max_ord = m.max(n);

    let mut num_z = vec![0.0; max_ord + 1];
    let mut den_z = vec![0.0; max_ord + 1];

    let scale = 2.0 / t_sample;

    // TODO: wrap in function
    // Get numerator coefficients
    for (k, &coeff) in num_s.iter().enumerate() {
        let ord = m - k;
        let mult = coeff * scale.powi(ord as i32);
        let poly_a = binomial_expansion(ord, true);
        let poly_b = binomial_expansion(max_ord - ord, false);
        let contrib: Vec<f64> = convolve(&poly_a, &poly_b).iter().map(|c| c * mult).collect();
        num_z = poly_add(&num_z, &contrib);
    }

    // Get denominator coefficients
    for (l, &coeff) in den_s.iter().enumerate() {
        let ord = n - l;
        let mult = coeff * scale.powi(ord as i32);
        let poly_a = binomial_expansion(ord, true);
        let poly_b = binomial_expansion(max_ord - ord, false);
        let contrib: Vec<f64> = convolve(&poly_a, &poly_b).iter().map(|c| c * mult).collect();
        den_z = poly_add(&den_z, &contrib);
    }

    // Normalize
    let norm = den_z[0];
    num_z.iter_mut().for_each(|c| *c /= norm);
    den_z.iter_mut().for_each(|c| *c /= norm);

    (num_z, den_z)   

}


#[cfg(test)]
mod tests {
    use crate::tf::bilinear::bilinear_transform;

    #[test]
    fn test_bilinear_transform_first_order_lowpass() {
        let num = vec![1.0];        // H(s) = 1 / (s + 1)
        let den = vec![1.0, 1.0];   // s + 1
        let sample_time = 1.0;

        let (num_z, den_z) = bilinear_transform(&num, &den, sample_time);

        assert_eq!(vec![1.0 / 3.0, 1.0 / 3.0], num_z);
        assert_eq!(vec![1.0, -1.0 / 3.0], den_z);
    }
    
}