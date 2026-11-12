# peppec-test

A small Rust library of integer helper functions.

## Command line

The `peppec-test` program exposes some of the functions on the command line. The first
argument is the command, the rest are its numbers:

```text
usage: peppec-test <command> [numbers...]
```

The only command so far is `median`, which prints the median of the numbers:

```console
$ peppec-test median 3 1 2
2
```

`peppec-test --help` prints the usage. An unknown command, a value that is not a 32-bit
integer, or a missing list of numbers prints a one-line error on the error stream and exits
with code 2.

## API

All functions live in `src/lib.rs` and are used through the `peppec_test` crate.

### `sum`

Returns the sum of two `i32` values, widened to `i64` so it cannot overflow.

```rust
use peppec_test::sum;

assert_eq!(sum(i32::MAX, 1), i64::from(i32::MAX) + 1);
```

### `average`

Returns the integer average of a slice, truncated toward zero (empty slice gives `0`).

```rust
use peppec_test::average;

assert_eq!(average(&[1, 2]), 1);
assert_eq!(average(&[-1, -2]), -1);
```

### `average_with`

Returns the integer average of a slice, rounded according to a `Rounding` mode (empty slice gives `0`).

```rust
use peppec_test::{average_with, Rounding};

assert_eq!(average_with(&[1, 2], Rounding::Ceil), 2);
assert_eq!(average_with(&[2, 3], Rounding::HalfToEven), 2);
```

`Rounding` is a public enum with the variants `TowardZero` (the default), `Floor`, `Ceil`,
`HalfAwayFromZero` and `HalfToEven`.

### `median`

Returns the median of a slice as `Option<f64>`, or `None` if it is empty.

```rust
use peppec_test::median;

assert_eq!(median(&[5, 1, 3]), Some(3.0));
assert_eq!(median(&[4, 1, 3, 2]), Some(2.5));
assert_eq!(median(&[]), None);
```

### `min_max`

Returns the smallest and largest value of a slice as `Option<(i32, i32)>`, or `None` if it is empty.

```rust
use peppec_test::min_max;

assert_eq!(min_max(&[3, -1, 9, 4]), Some((-1, 9)));
assert_eq!(min_max(&[]), None);
```

### `range`

Returns the difference between the largest and smallest value of a slice as `Option<i64>` (widened so it cannot overflow), or `None` if it is empty.

```rust
use peppec_test::range;

assert_eq!(range(&[3, -1, 9, 4]), Some(10));
assert_eq!(range(&[i32::MIN, i32::MAX]), Some(u32::MAX as i64));
assert_eq!(range(&[]), None);
```

### `count_even`

Returns how many values of a slice are even, as `usize` (zero and negative even numbers count; empty slice gives `0`).

```rust
use peppec_test::count_even;

assert_eq!(count_even(&[1, 2, 3, 4]), 2);
assert_eq!(count_even(&[-2, 0, i32::MIN, i32::MAX]), 3);
assert_eq!(count_even(&[]), 0);
```

### `count_odd`

Returns how many values of a slice are odd, as `usize` (negative odd numbers count; empty slice gives `0`).

```rust
use peppec_test::count_odd;

assert_eq!(count_odd(&[1, 2, 3, 4]), 2);
assert_eq!(count_odd(&[-1, 0, i32::MIN, i32::MAX]), 2);
assert_eq!(count_odd(&[]), 0);
```

### `sum_of_squares`

Returns the sum of the squares of a slice, as `i64` (saturating, so it never overflows; empty slice gives `0`).

```rust
use peppec_test::sum_of_squares;

assert_eq!(sum_of_squares(&[1, -2, 3]), 14);
assert_eq!(sum_of_squares(&[i32::MIN, i32::MIN]), i64::MAX);
assert_eq!(sum_of_squares(&[]), 0);
```
