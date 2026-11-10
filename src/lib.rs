/// Returns the sum of `a` and `b`, widened to `i64` so it cannot overflow.
pub fn sum(a: i32, b: i32) -> i64 {
    i64::from(a) + i64::from(b)
}

/// Returns the integer average of `values`, truncated toward zero.
///
/// The sum is computed in `i128` so it cannot overflow for any slice length.
/// An empty slice yields `0`.
pub fn average(values: &[i32]) -> i32 {
    if values.is_empty() {
        return 0;
    }
    let total: i128 = values.iter().map(|&v| i128::from(v)).sum();
    // The average of `i32` values always fits in `i32`.
    (total / values.len() as i128) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_of_values() {
        assert_eq!(average(&[2, 4, 6]), 4);
    }

    #[test]
    fn average_truncates_toward_zero() {
        assert_eq!(average(&[1, 2]), 1);
        assert_eq!(average(&[-1, -2]), -1);
    }

    #[test]
    fn average_of_empty_is_zero() {
        assert_eq!(average(&[]), 0);
    }

    #[test]
    fn average_does_not_overflow() {
        assert_eq!(average(&[i32::MAX, i32::MAX]), i32::MAX);
        assert_eq!(average(&[i32::MIN, i32::MIN]), i32::MIN);
    }

    #[test]
    fn average_of_many_extreme_values() {
        let values = vec![i32::MIN; 1 << 20];
        assert_eq!(average(&values), i32::MIN);
    }

    #[test]
    fn sums_positive_numbers() {
        assert_eq!(sum(2, 3), 5);
    }

    #[test]
    fn sums_negative_numbers() {
        assert_eq!(sum(-2, -3), -5);
    }

    #[test]
    fn sums_with_zero() {
        assert_eq!(sum(0, 7), 7);
    }

    #[test]
    fn does_not_overflow_at_upper_bound() {
        assert_eq!(sum(i32::MAX, 1), i64::from(i32::MAX) + 1);
        assert_eq!(sum(i32::MAX, i32::MAX), 2 * i64::from(i32::MAX));
    }

    #[test]
    fn does_not_overflow_at_lower_bound() {
        assert_eq!(sum(i32::MIN, -1), i64::from(i32::MIN) - 1);
        assert_eq!(sum(i32::MIN, i32::MIN), 2 * i64::from(i32::MIN));
    }
}
