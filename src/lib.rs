/// Returns the sum of `a` and `b`, widened to `i64` so it cannot overflow.
pub fn sum(a: i32, b: i32) -> i64 {
    i64::from(a) + i64::from(b)
}

#[cfg(test)]
mod tests {
    use super::*;

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
