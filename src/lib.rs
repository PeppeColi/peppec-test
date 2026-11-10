/// Returns the sum of `a` and `b`, widened to `i64` so it cannot overflow.
pub fn sum(a: i32, b: i32) -> i64 {
    i64::from(a) + i64::from(b)
}

/// How [`average_with`] rounds a non-integer average.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Rounding {
    /// Truncate toward zero (e.g. `1.5 -> 1`, `-1.5 -> -1`).
    #[default]
    TowardZero,
    /// Round toward negative infinity (e.g. `1.5 -> 1`, `-1.5 -> -2`).
    Floor,
    /// Round toward positive infinity (e.g. `1.5 -> 2`, `-1.5 -> -1`).
    Ceil,
    /// Round to the nearest integer, ties away from zero
    /// (e.g. `1.5 -> 2`, `-1.5 -> -2`).
    HalfAwayFromZero,
    /// Round to the nearest integer, ties to the even neighbour
    /// (e.g. `1.5 -> 2`, `2.5 -> 2`, `-1.5 -> -2`).
    HalfToEven,
}

/// Returns the integer average of `values`, truncated toward zero.
///
/// Equivalent to `average_with(values, Rounding::TowardZero)`.
/// An empty slice yields `0`.
pub fn average(values: &[i32]) -> i32 {
    average_with(values, Rounding::TowardZero)
}

/// Returns the integer average of `values`, rounded according to `rounding`.
///
/// The sum is computed in `i128` so it cannot overflow for any slice length.
/// An empty slice yields `0`.
pub fn average_with(values: &[i32], rounding: Rounding) -> i32 {
    if values.is_empty() {
        return 0;
    }
    let total: i128 = values.iter().map(|&v| i128::from(v)).sum();
    let len = values.len() as i128;
    let quotient = total / len;
    let remainder = total % len;
    // Direction to move away from zero when the result must be adjusted.
    let away = total.signum();
    let twice_rem = 2 * remainder.abs();

    let rounded = if remainder == 0 {
        quotient
    } else {
        match rounding {
            Rounding::TowardZero => quotient,
            Rounding::Floor if total < 0 => quotient - 1,
            Rounding::Floor => quotient,
            Rounding::Ceil if total > 0 => quotient + 1,
            Rounding::Ceil => quotient,
            Rounding::HalfAwayFromZero if twice_rem >= len => quotient + away,
            Rounding::HalfAwayFromZero => quotient,
            Rounding::HalfToEven if twice_rem > len || (twice_rem == len && quotient % 2 != 0) => {
                quotient + away
            }
            Rounding::HalfToEven => quotient,
        }
    };
    // The exact average lies within the range of the `i32` inputs, and rounding
    // to an integer cannot leave that range, so the result always fits.
    rounded as i32
}

/// Returns the median of `values`, or `None` if the slice is empty.
///
/// For an even count, the result is the mean of the two middle values.
/// The input slice is not modified.
pub fn median(values: &[i32]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        Some(f64::from(sorted[mid]))
    } else {
        Some((f64::from(sorted[mid - 1]) + f64::from(sorted[mid])) / 2.0)
    }
}

/// Returns the smallest and largest value of `values` as `(min, max)`,
/// or `None` if the slice is empty.
///
/// The slice is traversed only once.
pub fn min_max(values: &[i32]) -> Option<(i32, i32)> {
    let (&first, rest) = values.split_first()?;
    Some(
        rest.iter()
            .fold((first, first), |(min, max), &v| (min.min(v), max.max(v))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max_of_empty_is_none() {
        assert_eq!(min_max(&[]), None);
    }

    #[test]
    fn min_max_of_one_element() {
        assert_eq!(min_max(&[7]), Some((7, 7)));
    }

    #[test]
    fn min_max_of_many_values() {
        assert_eq!(min_max(&[3, -1, 9, 4]), Some((-1, 9)));
        assert_eq!(min_max(&[5, 5, 5]), Some((5, 5)));
        assert_eq!(min_max(&[i32::MAX, i32::MIN]), Some((i32::MIN, i32::MAX)));
    }

    #[test]
    fn median_of_empty_is_none() {
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn median_of_odd_count() {
        assert_eq!(median(&[5, 1, 3]), Some(3.0));
        assert_eq!(median(&[7]), Some(7.0));
    }

    #[test]
    fn median_of_even_count_is_mean_of_middle_values() {
        assert_eq!(median(&[4, 1, 3, 2]), Some(2.5));
        assert_eq!(median(&[i32::MAX, i32::MAX]), Some(f64::from(i32::MAX)));
    }

    #[test]
    fn median_does_not_modify_input() {
        let values = [3, 1, 2];
        let _ = median(&values);
        assert_eq!(values, [3, 1, 2]);
    }

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
    fn default_rounding_is_toward_zero() {
        assert_eq!(Rounding::default(), Rounding::TowardZero);
    }

    #[test]
    fn average_with_floor() {
        assert_eq!(average_with(&[1, 2], Rounding::Floor), 1);
        assert_eq!(average_with(&[-1, -2], Rounding::Floor), -2);
        assert_eq!(average_with(&[2, 4], Rounding::Floor), 3);
    }

    #[test]
    fn average_with_ceil() {
        assert_eq!(average_with(&[1, 2], Rounding::Ceil), 2);
        assert_eq!(average_with(&[-1, -2], Rounding::Ceil), -1);
        assert_eq!(average_with(&[-2, -4], Rounding::Ceil), -3);
    }

    #[test]
    fn average_with_half_away_from_zero() {
        assert_eq!(average_with(&[1, 2], Rounding::HalfAwayFromZero), 2);
        assert_eq!(average_with(&[-1, -2], Rounding::HalfAwayFromZero), -2);
        // 4/3 = 1.33.. -> 1, 5/3 = 1.66.. -> 2
        assert_eq!(average_with(&[1, 1, 2], Rounding::HalfAwayFromZero), 1);
        assert_eq!(average_with(&[1, 2, 2], Rounding::HalfAwayFromZero), 2);
        assert_eq!(average_with(&[-1, -2, -2], Rounding::HalfAwayFromZero), -2);
    }

    #[test]
    fn average_with_half_to_even() {
        // 1.5 -> 2, 2.5 -> 2, -1.5 -> -2, -2.5 -> -2
        assert_eq!(average_with(&[1, 2], Rounding::HalfToEven), 2);
        assert_eq!(average_with(&[2, 3], Rounding::HalfToEven), 2);
        assert_eq!(average_with(&[-1, -2], Rounding::HalfToEven), -2);
        assert_eq!(average_with(&[-2, -3], Rounding::HalfToEven), -2);
        // 5/3 = 1.66.. -> 2
        assert_eq!(average_with(&[1, 2, 2], Rounding::HalfToEven), 2);
    }

    #[test]
    fn average_with_all_modes_at_extremes() {
        let modes = [
            Rounding::TowardZero,
            Rounding::Floor,
            Rounding::Ceil,
            Rounding::HalfAwayFromZero,
            Rounding::HalfToEven,
        ];
        for mode in modes {
            assert_eq!(average_with(&[], mode), 0);
            assert_eq!(average_with(&[i32::MAX, i32::MAX], mode), i32::MAX);
            assert_eq!(average_with(&[i32::MIN, i32::MIN], mode), i32::MIN);
            assert!(average_with(&[i32::MAX, i32::MAX - 1], mode) >= i32::MAX - 1);
            assert!(average_with(&[i32::MIN, i32::MIN + 1], mode) <= i32::MIN + 1);
        }
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
