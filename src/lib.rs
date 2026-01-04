#![doc = include_str!("../README.md")]
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

    /// 10進数での桁数を返す (ilog10を使用するため高速)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(100.digits_len(), 3);
    /// ```
    fn digits_len(self) -> u32;

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

    /// n進数での桁数を返す
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// // 16 (10進数) -> 10000 (2進数) -> 5桁
    /// assert_eq!(16.digits_len_radix(2), 5);
    /// ```
    fn digits_len_radix(self, base: u32) -> u32;

    // ============================================================
    // ユーティリティ
    // ============================================================

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
}

// ----------------------------------------------------------------
// 実装用マクロ (符号なし整数用: u32, u64...)
// ----------------------------------------------------------------
macro_rules! impl_keta_uint {
    ($($t:ty),*) => {
        $(
            impl Keta for $t {
                // --- n進数 実装 ---
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

                // --- 10進数ショートカット ---
                fn digits(self) -> Vec<u8> { self.digits_radix(10) }
                fn from_digits(digits: &[u8]) -> Self { Self::from_digits_radix(digits, 10) }
                fn digit_sum(self) -> u64 { self.digit_sum_radix(10) }

                // 10進数桁数だけは ilog10 で高速化
                fn digits_len(self) -> u32 {
                    if self == 0 { return 1; }
                    self.ilog10() + 1
                }

                // --- ユーティリティ ---
                fn reverse(self) -> Self {
                    let mut n = self;
                    let mut ret: $t = 0;
                    while n > 0 {
                        ret = ret * 10 + (n % 10);
                        n /= 10;
                    }
                    ret
                }

                fn is_palindrome(self) -> bool {
                    self == self.reverse()
                }

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
            }
        )*
    };
}

// ----------------------------------------------------------------
// 実装用マクロ (符号付き整数用: i32, i64...)
// ※ abs()を使う必要があるため分けている
// ----------------------------------------------------------------
macro_rules! impl_keta_int {
    ($($t:ty),*) => {
        $(
            impl Keta for $t {
                // --- n進数 実装 ---
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

                // --- 10進数ショートカット ---
                fn digits(self) -> Vec<u8> { self.digits_radix(10) }
                fn from_digits(digits: &[u8]) -> Self { Self::from_digits_radix(digits, 10) }
                fn digit_sum(self) -> u64 { self.digit_sum_radix(10) }

                fn digits_len(self) -> u32 {
                    if self == 0 { return 1; }
                    self.abs().ilog10() + 1
                }

                // --- ユーティリティ ---
                fn reverse(self) -> Self {
                    let mut n = self.abs();
                    let mut ret: $t = 0;
                    while n > 0 {
                        ret = ret * 10 + (n % 10);
                        n /= 10;
                    }
                    if self < 0 { -ret } else { ret }
                }

                fn is_palindrome(self) -> bool {
                    self == self.reverse()
                }

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
            }
        )*
    };
}

// 型への実装
impl_keta_uint!(u8, u16, u32, u64, u128, usize);
impl_keta_int!(i8, i16, i32, i64, i128, isize);

// ----------------------------------------------------------------
// テストコード
// ----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    // --- 10進数 基本テスト ---
    #[test]
    fn test_digits() {
        assert_eq!(12345.digits(), vec![1, 2, 3, 4, 5]);
        assert_eq!(0.digits(), vec![0]);
        assert_eq!((-123_i64).digits(), vec![1, 2, 3]);
    }

    #[test]
    fn test_digit_sum() {
        assert_eq!(12345.digit_sum(), 15);
        assert_eq!(999.digit_sum(), 27);
    }

    #[test]
    fn test_from_digits() {
        let v = vec![1, 2, 3];
        assert_eq!(u64::from_digits(&v), 123);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(12345.reverse(), 54321);
        assert_eq!((-123_i64).reverse(), -321); // 符号維持
    }

    #[test]
    fn test_digits_len() {
        assert_eq!(1.digits_len(), 1);
        assert_eq!(10.digits_len(), 2);
        assert_eq!(100.digits_len(), 3);
        assert_eq!(0.digits_len(), 1);
        assert_eq!((-123).digits_len(), 3); // 符号無視
    }

    #[test]
    fn test_concat() {
        assert_eq!(12_u64.concat(34), 1234);
        assert_eq!(1_u64.concat(0), 10);
        assert_eq!((-12).concat(34), -1234);
    }

    #[test]
    fn test_nth_digit() {
        let n = 54321;
        assert_eq!(n.nth_digit(0), Some(5)); // 上から1桁目
        assert_eq!(n.nth_digit(4), Some(1)); // 上から5桁目
        assert_eq!(n.nth_digit(5), None); // 範囲外
    }

    #[test]
    fn test_is_palindrome() {
        assert!(121.is_palindrome());
        assert!(!123.is_palindrome());
        assert!(1.is_palindrome());
        assert!(0.is_palindrome());
    }

    // --- n進数 (Radix) テスト ---
    #[test]
    fn test_digits_radix() {
        // 6 (10進数) -> 110 (2進数)
        assert_eq!(6.digits_radix(2), vec![1, 1, 0]);
        // 255 (10進数) -> FF (16進数) -> [15, 15]
        assert_eq!(255.digits_radix(16), vec![15, 15]);
        // 0 は [0]
        assert_eq!(0.digits_radix(5), vec![0]);
    }

    #[test]
    fn test_from_digits_radix() {
        // 110 (2進数) -> 6
        assert_eq!(u64::from_digits_radix(&[1, 1, 0], 2), 6);
        // FF (16進数) -> 255
        assert_eq!(u64::from_digits_radix(&[15, 15], 16), 255);
    }

    #[test]
    fn test_digit_sum_radix() {
        // 6 = 110(2) -> 1+1+0 = 2
        assert_eq!(6.digit_sum_radix(2), 2);
    }

    #[test]
    fn test_digits_len_radix() {
        // 16 = 10000(2) -> 5桁
        assert_eq!(16.digits_len_radix(2), 5);
        // 255 = FF(16) -> 2桁
        assert_eq!(255.digits_len_radix(16), 2);
    }
}
