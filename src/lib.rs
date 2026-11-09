/// Returns the sum of `a` and `b`.
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
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
}
