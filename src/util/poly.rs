
use std::ops::{AddAssign, Mul};
use num::{complex::Complex64, Zero};

pub fn binomial_expansion(pow: usize, negative: bool) -> Vec<f64> {
    if negative {
        NEG_PASCAL[pow][0..=pow].iter().map(|&x| x as f64).collect()
    } else {
        PASCAL[pow][0..=pow].iter().map(|&x| x as f64).collect()
    }

}

pub fn poly_add(a: &[f64], b: &[f64]) -> Vec<f64> {
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

pub fn convolve<T>(f: &[T], g: &[T]) -> Vec<T>
where 
    T: Copy + AddAssign + Mul<Output = T> + Zero,
{
    let mut result = vec![T::zero(); f.len() + g.len() - 1];
    for (i, &fi) in f.iter().enumerate() {
        for (j, &gj) in g.iter().enumerate() {
            result[i + j] += fi * gj;
        }
    }
    result
}

pub fn reduce_to_real(coeffs: &[Complex64]) -> Vec<f64> {
    coeffs.iter().map(|&c| c.re).collect()
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

const MAX_ORDER: usize = 20;
pub const PASCAL: [[isize; MAX_ORDER + 1]; MAX_ORDER + 1] = build_pascal_triangle(MAX_ORDER);
pub const NEG_PASCAL: [[isize; MAX_ORDER + 1]; MAX_ORDER + 1] = build_neg_pascal_triangle(MAX_ORDER);



#[cfg(test)]
mod tests {
    use crate::util::poly::{NEG_PASCAL, PASCAL};

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
}