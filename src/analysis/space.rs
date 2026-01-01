use num::Float;

pub fn linspace<T: Float>(start: T, stop: T, count: usize) -> Vec<T> {
    let spacing = (stop - start) / T::from(count - 1).unwrap();
    (0..count)
        .map(|n| start + T::from(n).unwrap() * spacing)
        .collect()
}

pub fn logspace<T: Float>(start_exp: T, stop_exp: T, count: usize) -> Vec<T> {
    linspace(start_exp, stop_exp, count)
        .iter()
        .map(|&n| T::from(10.0).unwrap().powf(n))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linpace() {
        // Given
        let (start, stop, count) = (100.0, 500.0, 5);

        // When
        let result = linspace(start, stop, count);

        // Then
        let expected = vec![100.0, 200.0, 300.0, 400.0, 500.0];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_logspace() {
        // Given
        let (start_exp, stop_exp, count) = (0.0, 5.0, 6);

        // When
        let result = logspace(start_exp, stop_exp, count);

        // Then
        let expected = vec![1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0];
        assert_eq!(expected, result);
    }
}
