#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]
pub trait Keta: Copy {
    // ============================================================
    // 10進数ショートカット (よく使うので短い名前)
    // ============================================================

    /// 10進数で各桁の数字(u8)のベクタに分解する
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(12345.digits(), vec![1, 2, 3, 4, 5]);
    /// assert_eq!((-12345).digits(), vec![1, 2, 3, 4, 5]); // 負の数も絶対値で分解
    /// ```
    fn digits(self) -> Vec<u8>;

    /// 数字の列から数値を復元する (10進数)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// let nums = vec![1, 2, 3];
    /// assert_eq!(u64::from_digits(&nums), 123);
    /// ```
    fn from_digits(digits: &[u8]) -> Self;

    /// 10進数での各桁の和を計算する
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(123.digit_sum(), 6);
    /// ```
    fn digit_sum(self) -> u64;

    /// 10進数での各桁の積を計算する
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(1234.digit_product(), 24);
    /// assert_eq!(103.digit_product(), 0);
    /// ```
    fn digit_product(self) -> u64;

    /// 10進数での桁数を返す (ilog10を使用するため高速)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(100.digits_len(), 3);
    /// ```
    fn digits_len(self) -> u32;

    /// 数値の並びを反転させる (10進数)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(123.reverse(), 321);
    /// assert_eq!((-123).reverse(), -321); // 符号は維持
    /// ```
    fn reverse(self) -> Self;

    /// 回文数かどうか判定する (10進数)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert!(121.is_palindrome());
    /// assert!(!123.is_palindrome());
    /// ```
    fn is_palindrome(self) -> bool;

    /// 上からi番目の桁を取得する (10進数, 0-indexed)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(12345.nth_digit(0), Some(1));
    /// assert_eq!(12345.nth_digit(4), Some(5));
    /// assert_eq!(12345.nth_digit(100), None);
    /// ```
    fn nth_digit(self, i: u32) -> Option<u8>;

    /// 数値を結合する (10進数)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(12.concat(34), 1234);
    /// ```
    fn concat(self, other: Self) -> Self;

    /// 指定した数字(0-9)が含まれているか判定する (10進数)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert!(12345.contains_digit(3));
    /// assert!(!12345.contains_digit(9));
    /// ```
    fn contains_digit(self, digit: u8) -> bool;

    /// 桁を並び替えてできる「最大の数値」を返す (10進数)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(2026.make_max(), 6220);
    /// ```
    fn make_max(self) -> Self;

    /// 桁を並び替えてできる「最小の数値」を返す (10進数)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(2026.make_min(), 226); // 0226 -> 226
    /// ```
    fn make_min(self) -> Self;

    // ============================================================
    // n進数対応 (Radix)
    // ============================================================

    /// n進数で各桁の数字(u8)のベクタに分解する
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// // 6 (10進数) -> 110 (2進数)
    /// assert_eq!(6.digits_radix(2), vec![1, 1, 0]);
    /// // 255 (10進数) -> FF (16進数) -> [15, 15]
    /// assert_eq!(255.digits_radix(16), vec![15, 15]);
    /// ```
    fn digits_radix(self, base: u32) -> Vec<u8>;

    /// n進数の数字列から数値を復元する
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// // 110 (2進数) -> 6 (10進数)
    /// assert_eq!(u64::from_digits_radix(&[1, 1, 0], 2), 6);
    /// ```
    fn from_digits_radix(digits: &[u8], base: u32) -> Self;

    /// n進数での各桁の和を計算する
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// // 6 (10進数) -> 110 (2進数) -> 1+1+0 = 2
    /// assert_eq!(6.digit_sum_radix(2), 2);
    /// ```
    fn digit_sum_radix(self, base: u32) -> u64;

    /// n進数での各桁の積を計算する
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// // 7 (10進数) -> 111 (2進数) -> 1*1*1 = 1
    /// assert_eq!(7.digit_product_radix(2), 1);
    /// ```
    fn digit_product_radix(self, base: u32) -> u64;

    /// n進数での桁数を返す
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// // 16 (10進数) -> 10000 (2進数) -> 5桁
    /// assert_eq!(16.digits_len_radix(2), 5);
    /// ```
    fn digits_len_radix(self, base: u32) -> u32;

    /// 数値の並びを反転させる (n進数)
    fn reverse_radix(self, base: u32) -> Self;

    /// 回文数かどうか判定する (n進数)
    fn is_palindrome_radix(self, base: u32) -> bool;

    /// 上からi番目の桁を取得する (n進数, 0-indexed)
    fn nth_digit_radix(self, i: u32, base: u32) -> Option<u8>;

    /// 数値を結合する (n進数)
    fn concat_radix(self, other: Self, base: u32) -> Self;

    /// 指定した数字が含まれているか判定する (n進数)
    fn contains_digit_radix(self, digit: u8, base: u32) -> bool;

    /// 桁を並び替えてできる「最大の数値」を返す (n進数)
    fn make_max_radix(self, base: u32) -> Self;

    /// 桁を並び替えてできる「最小の数値」を返す (n進数)
    fn make_min_radix(self, base: u32) -> Self;
}

// ----------------------------------------------------------------
// 実装用マクロ (符号なし整数用: u32, u64...)
// ----------------------------------------------------------------
macro_rules! impl_keta_uint {
    ($($t:ty),*) => {
        $(
            impl Keta for $t {
                // --- Radix Implementations ---
                fn digits_radix(self, base: u32) -> Vec<u8> {
                    if self == 0 { return vec![0]; }
                    let mut n = self;
                    let b = base as $t;
                    let mut ret = Vec::with_capacity(20);
                    while n > 0 {
                        ret.push((n % b) as u8);
                        n /= b;
                    }
                    ret.reverse();
                    ret
                }

                fn from_digits_radix(digits: &[u8], base: u32) -> Self {
                    let mut ret: $t = 0;
                    let b = base as $t;
                    for &d in digits {
                        ret = ret * b + (d as $t);
                    }
                    ret
                }

                fn digit_sum_radix(self, base: u32) -> u64 {
                    let mut n = self;
                    let b = base as $t;
                    let mut sum: u64 = 0;
                    while n > 0 {
                        sum += (n % b) as u64;
                        n /= b;
                    }
                    sum
                }

                fn digit_product_radix(self, base: u32) -> u64 {
                    if self == 0 { return 0; }
                    let mut n = self;
                    let b = base as $t;
                    let mut prod: u64 = 1;
                    while n > 0 {
                        prod *= (n % b) as u64;
                        n /= b;
                    }
                    prod
                }

                fn digits_len_radix(self, base: u32) -> u32 {
                    if self == 0 { return 1; }
                    let mut n = self;
                    let b = base as $t;
                    let mut cnt = 0;
                    while n > 0 {
                        n /= b;
                        cnt += 1;
                    }
                    cnt
                }

                fn reverse_radix(self, base: u32) -> Self {
                    let mut n = self;
                    let b = base as $t;
                    let mut ret: $t = 0;
                    while n > 0 {
                        ret = ret * b + (n % b);
                        n /= b;
                    }
                    ret
                }

                fn is_palindrome_radix(self, base: u32) -> bool {
                    self == self.reverse_radix(base)
                }

                fn nth_digit_radix(self, i: u32, base: u32) -> Option<u8> {
                    let l = self.digits_len_radix(base);
                    if i >= l { return None; }
                    let pow = l - 1 - i;
                    let b = base as $t;
                    Some(((self / b.pow(pow)) % b) as u8)
                }

                fn concat_radix(self, other: Self, base: u32) -> Self {
                    let shift = other.digits_len_radix(base);
                    let b = base as $t;
                    self * b.pow(shift) + other
                }

                fn contains_digit_radix(self, digit: u8, base: u32) -> bool {
                    let mut n = self;
                    let b = base as $t;
                    if n == 0 { return digit == 0; }
                    while n > 0 {
                        if (n % b) as u8 == digit {
                            return true;
                        }
                        n /= b;
                    }
                    false
                }

                fn make_max_radix(self, base: u32) -> Self {
                    let mut d = self.digits_radix(base);
                    d.sort_unstable_by(|a, b| b.cmp(a));
                    Self::from_digits_radix(&d, base)
                }

                fn make_min_radix(self, base: u32) -> Self {
                    let mut d = self.digits_radix(base);
                    d.sort_unstable();
                    Self::from_digits_radix(&d, base)
                }

                // --- 10-base Shortcuts ---
                fn digits(self) -> Vec<u8> { self.digits_radix(10) }
                fn from_digits(digits: &[u8]) -> Self { Self::from_digits_radix(digits, 10) }
                fn digit_sum(self) -> u64 { self.digit_sum_radix(10) }
                fn digit_product(self) -> u64 { self.digit_product_radix(10) }
                fn digits_len(self) -> u32 {
                    if self == 0 { return 1; }
                    self.ilog10() + 1
                }
                fn reverse(self) -> Self { self.reverse_radix(10) }
                fn is_palindrome(self) -> bool { self.is_palindrome_radix(10) }
                fn nth_digit(self, i: u32) -> Option<u8> {
                    let l = self.digits_len();
                    if i >= l { return None; }
                    let pow = l - 1 - i;
                    Some(((self / (10 as $t).pow(pow)) % 10) as u8)
                }
                fn concat(self, other: Self) -> Self {
                    let shift = other.digits_len();
                    self * (10 as $t).pow(shift) + other
                }
                fn contains_digit(self, digit: u8) -> bool {
                    self.contains_digit_radix(digit, 10)
                }
                fn make_max(self) -> Self { self.make_max_radix(10) }
                fn make_min(self) -> Self { self.make_min_radix(10) }
            }
        )*
    };
}

// ----------------------------------------------------------------
// 実装用マクロ (符号付き整数用: i32, i64...)
// ----------------------------------------------------------------
macro_rules! impl_keta_int {
    ($($t:ty),*) => {
        $(
            impl Keta for $t {
                // --- Radix Implementations ---
                fn digits_radix(self, base: u32) -> Vec<u8> {
                    if self == 0 { return vec![0]; }
                    let mut n = self.abs();
                    let b = base as $t;
                    let mut ret = Vec::with_capacity(20);
                    while n > 0 {
                        ret.push((n % b) as u8);
                        n /= b;
                    }
                    ret.reverse();
                    ret
                }

                fn from_digits_radix(digits: &[u8], base: u32) -> Self {
                    let mut ret: $t = 0;
                    let b = base as $t;
                    for &d in digits {
                        ret = ret * b + (d as $t);
                    }
                    ret
                }

                fn digit_sum_radix(self, base: u32) -> u64 {
                    let mut n = self.abs();
                    let b = base as $t;
                    let mut sum: u64 = 0;
                    while n > 0 {
                        sum += (n % b) as u64;
                        n /= b;
                    }
                    sum
                }

                fn digit_product_radix(self, base: u32) -> u64 {
                    let mut n = self.abs();
                    if n == 0 { return 0; }
                    let b = base as $t;
                    let mut prod: u64 = 1;
                    while n > 0 {
                        prod *= (n % b) as u64;
                        n /= b;
                    }
                    prod
                }

                fn digits_len_radix(self, base: u32) -> u32 {
                    if self == 0 { return 1; }
                    let mut n = self.abs();
                    let b = base as $t;
                    let mut cnt = 0;
                    while n > 0 {
                        n /= b;
                        cnt += 1;
                    }
                    cnt
                }

                fn reverse_radix(self, base: u32) -> Self {
                    let mut n = self.abs();
                    let b = base as $t;
                    let mut ret: $t = 0;
                    while n > 0 {
                        ret = ret * b + (n % b);
                        n /= b;
                    }
                    if self < 0 { -ret } else { ret }
                }

                fn is_palindrome_radix(self, base: u32) -> bool {
                    self == self.reverse_radix(base)
                }

                fn nth_digit_radix(self, i: u32, base: u32) -> Option<u8> {
                    let l = self.digits_len_radix(base);
                    if i >= l { return None; }
                    let pow = l - 1 - i;
                    let b = base as $t;
                    Some(((self.abs() / b.pow(pow)) % b) as u8)
                }

                fn concat_radix(self, other: Self, base: u32) -> Self {
                    let shift = other.digits_len_radix(base);
                    let added = other.abs();
                    let b = base as $t;
                    let shifted = self * b.pow(shift);
                    if self < 0 { shifted - added } else { shifted + added }
                }

                fn contains_digit_radix(self, digit: u8, base: u32) -> bool {
                    let mut n = self.abs();
                    let b = base as $t;
                    if n == 0 { return digit == 0; }
                    while n > 0 {
                        if (n % b) as u8 == digit {
                            return true;
                        }
                        n /= b;
                    }
                    false
                }

                fn make_max_radix(self, base: u32) -> Self {
                    let mut d = self.digits_radix(base);
                    d.sort_unstable_by(|a, b| b.cmp(a));
                    Self::from_digits_radix(&d, base)
                }

                fn make_min_radix(self, base: u32) -> Self {
                    let mut d = self.digits_radix(base);
                    d.sort_unstable();
                    Self::from_digits_radix(&d, base)
                }

                // --- 10-base Shortcuts ---
                fn digits(self) -> Vec<u8> { self.digits_radix(10) }
                fn from_digits(digits: &[u8]) -> Self { Self::from_digits_radix(digits, 10) }
                fn digit_sum(self) -> u64 { self.digit_sum_radix(10) }
                fn digit_product(self) -> u64 { self.digit_product_radix(10) }
                fn digits_len(self) -> u32 {
                    if self == 0 { return 1; }
                    self.abs().ilog10() + 1
                }
                fn reverse(self) -> Self { self.reverse_radix(10) }
                fn is_palindrome(self) -> bool { self.is_palindrome_radix(10) }
                fn nth_digit(self, i: u32) -> Option<u8> {
                    let l = self.digits_len();
                    if i >= l { return None; }
                    let pow = l - 1 - i;
                    Some(((self.abs() / (10 as $t).pow(pow)) % 10) as u8)
                }
                fn concat(self, other: Self) -> Self {
                    let shift = other.digits_len();
                    let added = other.abs();
                    let shifted = self * (10 as $t).pow(shift);
                    if self < 0 { shifted - added } else { shifted + added }
                }
                fn contains_digit(self, digit: u8) -> bool {
                    self.contains_digit_radix(digit, 10)
                }
                fn make_max(self) -> Self { self.make_max_radix(10) }
                fn make_min(self) -> Self { self.make_min_radix(10) }
            }
        )*
    };
}

// 型への実装
impl_keta_uint!(u8, u16, u32, u64, u128, usize);
impl_keta_int!(i8, i16, i32, i64, i128, isize);
