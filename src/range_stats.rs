/// Returns the spread of `values`: the largest value minus the smallest one.
///
/// Returns `None` if the slice is empty, or if the difference does not fit
/// in `i64`. A single value has a spread of `0`.
pub fn rs_spread(values: &[i64]) -> Option<i64> {
    let max = values.iter().copied().max()?;
    let min = values.iter().copied().min()?;
    max.checked_sub(min)
}

/// Returns the midrange of `values`: the sum of the smallest and the largest
/// value, divided by two and truncated toward zero.
///
/// Returns `None` if the slice is empty. A single value is its own midrange.
pub fn rs_midrange(values: &[i64]) -> Option<i64> {
    let max = values.iter().copied().max()?;
    let min = values.iter().copied().min()?;
    // The sum is computed in `i128` so it cannot overflow. The midpoint lies
    // between `min` and `max`, so the result always fits in `i64`.
    let mid = (i128::from(min) + i128::from(max)) / 2;
    Some(mid as i64)
}

/// Returns the largest gap of `values`: the largest difference between two
/// neighbouring values once the slice is sorted.
///
/// Returns `None` if the slice is empty, or if any gap does not fit in `i64`
/// (a larger gap would overflow too, so the result is `None` in that case as
/// well). A single value has no neighbours and its largest gap is `0`.
pub fn rs_max_gap(values: &[i64]) -> Option<i64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let mut largest = 0;
    for pair in sorted.windows(2) {
        largest = largest.max(pair[1].checked_sub(pair[0])?);
    }
    Some(largest)
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

    #[test]
    fn rs_midrange_of_empty_is_none() {
        assert_eq!(rs_midrange(&[]), None);
    }

    #[test]
    fn rs_midrange_of_one_value_is_that_value() {
        assert_eq!(rs_midrange(&[7]), Some(7));
        assert_eq!(rs_midrange(&[-7]), Some(-7));
    }

    #[test]
    fn rs_midrange_of_several_values() {
        // (-1 + 9) / 2 = 4
        assert_eq!(rs_midrange(&[3, -1, 9, 4]), Some(4));
        // (-10 + -3) / 2 = -6.5 -> -6
        assert_eq!(rs_midrange(&[-10, -3, -7]), Some(-6));
        assert_eq!(rs_midrange(&[5, 5, 5]), Some(5));
    }

    #[test]
    fn rs_midrange_truncates_toward_zero() {
        // 3 / 2 = 1.5 -> 1
        assert_eq!(rs_midrange(&[1, 2]), Some(1));
        // -3 / 2 = -1.5 -> -1
        assert_eq!(rs_midrange(&[-1, -2]), Some(-1));
    }

    #[test]
    fn rs_midrange_of_sum_that_overflows_i64() {
        assert_eq!(rs_midrange(&[i64::MAX, i64::MAX]), Some(i64::MAX));
        assert_eq!(rs_midrange(&[i64::MIN, i64::MIN]), Some(i64::MIN));
        // MIN + MAX = -1 -> truncated to 0
        assert_eq!(rs_midrange(&[i64::MIN, i64::MAX]), Some(0));
        // (MAX - 1 + MAX) / 2 = MAX - 1
        assert_eq!(rs_midrange(&[i64::MAX - 1, i64::MAX]), Some(i64::MAX - 1));
        // MIN + -1 overflows; (MIN - 1) / 2 = -2^62 after truncation
        assert_eq!(rs_midrange(&[i64::MIN, -1]), Some(i64::MIN / 2));
    }

    #[test]
    fn rs_max_gap_of_empty_is_none() {
        assert_eq!(rs_max_gap(&[]), None);
    }

    #[test]
    fn rs_max_gap_of_one_value_is_zero() {
        assert_eq!(rs_max_gap(&[7]), Some(0));
        assert_eq!(rs_max_gap(&[-7]), Some(0));
    }

    #[test]
    fn rs_max_gap_of_several_values() {
        // Sorted: 1, 4, 10, 20 -> gaps 3, 6, 10
        assert_eq!(rs_max_gap(&[10, 1, 4, 20]), Some(10));
        // Sorted: -10, -7, -3 -> gaps 3, 4
        assert_eq!(rs_max_gap(&[-3, -10, -7]), Some(4));
        assert_eq!(rs_max_gap(&[5, 5, 5]), Some(0));
        assert_eq!(rs_max_gap(&[1, 2, 3, 4]), Some(1));
    }

    #[test]
    fn rs_max_gap_at_extremes_without_overflow() {
        assert_eq!(rs_max_gap(&[-1, 0, i64::MAX]), Some(i64::MAX));
        assert_eq!(rs_max_gap(&[i64::MAX - 1, i64::MAX]), Some(1));
    }

    #[test]
    fn rs_max_gap_overflowing_difference_is_none() {
        assert_eq!(rs_max_gap(&[i64::MIN, i64::MAX]), None);
        assert_eq!(rs_max_gap(&[i64::MIN, 0, i64::MAX]), None);
        // The gap from -1 to MAX overflows even though MIN to -1 fits.
        assert_eq!(rs_max_gap(&[i64::MIN, -1, i64::MAX]), None);
    }
}
