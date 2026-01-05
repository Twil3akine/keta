# Keta

A lightweight, zero-dependency Rust crate for digit operations, designed for competitive programming (AtCoder, etc.).

[![Crates.io](https://img.shields.io/crates/v/keta.svg)](https://crates.io/crates/keta)
[![Documentation](https://docs.rs/keta/badge.svg)](https://docs.rs/keta)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Zero Dependencies**: Depends only on `std`. Fast compile times.
- **Digit Manipulation**: Easily decompose numbers into digits (`digits()`), sum them (`digit_sum()`), or reverse them (`reverse()`).
- **Radix Support**: Handle binary, octal, hexadecimal, or any base-N operations (`digits_radix(2)`).
- **Useful Utilities**: Product of digits (`digit_product()`), check digit existence (`contains_digit()`), or rearrange digits (`make_max()`, `make_min()`).
- **Competitive Programming Ready**: Optimized for speed, perfect for problems involving digit sums, palindromes, or base conversion.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
keta = "0.3.2"
```

## Usage

```rust
use keta::Keta;

fn main() {
    // 1. Decomposition (Decimal & Radix)
    let n = 12345;
    assert_eq!(n.digits(), vec![1, 2, 3, 4, 5]);
    assert_eq!(6.digits_radix(2), vec![1, 1, 0]); // 6 in binary is 110

    // 2. Aggregation (Sum & Product)
    assert_eq!(n.digit_sum(), 15);        // 1+2+3+4+5 = 15
    assert_eq!(123.digit_product(), 6);   // 1*2*3 = 6
    assert_eq!(6.digit_sum_radix(2), 2);  // 1+1+0 = 2

    // 3. Rearrangement & Properties
    assert_eq!(2026.make_max(), 6220);
    assert_eq!(2026.make_min(), 226);
    assert!(12345.contains_digit(3));
    assert!(12321.is_palindrome());
    assert_eq!(n.reverse(), 54321);
}

```

## License

This project is licensed under the [MIT LICENSE](LICENSE).
