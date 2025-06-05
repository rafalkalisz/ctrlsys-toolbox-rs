
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

fn binomial_expansion(pow: usize, negative: bool) -> Vec<f64> {
    if negative {
        NEG_PASCAL[pow][0..=pow].iter().map(|&x| x as f64).collect()
    } else {
        PASCAL[pow][0..=pow].iter().map(|&x| x as f64).collect()
    }

}

fn poly_add(a: &[f64], b: &[f64]) -> Vec<f64> {
    let len = a.len().max(b.len());
    let mut result = vec![0.0; len];
    let offset_a = len - a.len();
    let offset_b = len - b.len();

    for i in 0..a.len() {
        result[offset_a + i] += a[i];
    }
    for i in 0..b.len() {
        result[offset_b + i] += b[i];
    }
    result
}

fn convolve(f: &[f64], g: &[f64]) -> Vec<f64> {
    let mut result = vec![0.0; f.len() + g.len() - 1];
    for (i, &fi) in f.iter().enumerate() {
        for (j, &gj) in g.iter().enumerate() {
            result[i + j] += fi * gj;
        }
    }
    result
}

const fn build_pascal_triangle(max_pow: usize) -> [[isize; MAX_ORDER + 1]; MAX_ORDER + 1] {
    // Represent triangle as square array... ( ͠° ͟ʖ ͡°)
    let mut triangle = [[0; MAX_ORDER + 1]; MAX_ORDER + 1]; 
    triangle[0][0] = 1;
    let mut i = 1;
    // No for loops because const fn
    while i <= max_pow {
        triangle[i][0] = 1;
        let mut j = 1;
        while j <= i {
            triangle[i][j] = triangle[i-1][j-1] + triangle[i-1][j];
            j += 1;
        }
        i += 1;
    }
    triangle
}

const fn build_neg_pascal_triangle(max_pow: usize) -> [[isize; MAX_ORDER + 1]; MAX_ORDER + 1] {
    let mut triangle = build_pascal_triangle(max_pow);
    let mut i = 1;
    while i <= max_pow {
        if i % 2 != 0 {
            let mut j = 0;
            while j <= max_pow {
                triangle[j][i] = -1 * triangle[j][i];
                j += 1
            }
        }
        i += 1;
    }
    triangle
}

const MAX_ORDER: usize = 10;
pub const PASCAL: [[isize; MAX_ORDER + 1]; MAX_ORDER + 1] = build_pascal_triangle(MAX_ORDER);
pub const NEG_PASCAL: [[isize; MAX_ORDER + 1]; MAX_ORDER + 1] = build_neg_pascal_triangle(MAX_ORDER);


#[cfg(test)]
mod tests {
    use crate::tf::bilinear::{bilinear_transform, NEG_PASCAL, PASCAL};

    #[test]
    fn test_pascal_triangle() {
        assert_eq!(PASCAL[0][0], 1);
        assert_eq!(PASCAL[1][0], 1);
        assert_eq!(PASCAL[1][1], 1);
        assert_eq!(PASCAL[2][1], 2);
        assert_eq!(PASCAL[4][2], 6);
        assert_eq!(PASCAL[5][3], 10);
        assert_eq!(PASCAL[6][3], 20);
    }

    #[test]
    fn test_neg_pascal_triangle() {
        assert_eq!(NEG_PASCAL[0][0], 1);
        assert_eq!(NEG_PASCAL[1][0], 1);
        assert_eq!(NEG_PASCAL[1][1], -1);
        assert_eq!(NEG_PASCAL[2][1], -2);
        assert_eq!(NEG_PASCAL[4][2], 6);
        assert_eq!(NEG_PASCAL[5][3], -10);
        assert_eq!(NEG_PASCAL[6][3], -20);
    }

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