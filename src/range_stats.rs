/// Returns the spread of `values`: the largest value minus the smallest one.
///
/// Returns `None` if the slice is empty, or if the difference does not fit
/// in `i64`. A single value has a spread of `0`.
pub fn rs_spread(values: &[i64]) -> Option<i64> {
    let max = values.iter().copied().max()?;
    let min = values.iter().copied().min()?;
    max.checked_sub(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rs_spread_of_empty_is_none() {
        assert_eq!(rs_spread(&[]), None);
    }

    #[test]
    fn rs_spread_of_one_value_is_zero() {
        assert_eq!(rs_spread(&[7]), Some(0));
        assert_eq!(rs_spread(&[-7]), Some(0));
    }

    #[test]
    fn rs_spread_of_several_values() {
        assert_eq!(rs_spread(&[3, -1, 9, 4]), Some(10));
        assert_eq!(rs_spread(&[-10, -3, -7]), Some(7));
        assert_eq!(rs_spread(&[5, 5, 5]), Some(0));
    }

    #[test]
    fn rs_spread_at_extremes_without_overflow() {
        assert_eq!(rs_spread(&[i64::MIN, -1]), Some(i64::MAX));
        assert_eq!(rs_spread(&[0, i64::MAX]), Some(i64::MAX));
    }

    #[test]
    fn rs_spread_overflowing_difference_is_none() {
        assert_eq!(rs_spread(&[i64::MIN, i64::MAX]), None);
        assert_eq!(rs_spread(&[i64::MIN, 0, i64::MAX]), None);
        assert_eq!(rs_spread(&[i64::MIN, 1]), None);
    }
}
