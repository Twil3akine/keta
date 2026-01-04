/// 桁操作を便利にするトレイト
pub trait Keta: Copy {
    /// 数値を各桁の数字(u8)のベクタに分解する
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(12345.digits(), vec![1, 2, 3, 4, 5]);
    /// assert_eq!((-12345).digits(), vec![1, 2, 3, 4, 5]); // 負の数も絶対値で分解
    /// ```
    fn digits(self) -> Vec<u8>;

    /// 数字の列から数値を復元する
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// let nums = vec![1, 2, 3];
    /// assert_eq!(u64::from_digits(&nums), 123);
    /// ```
    fn from_digits(digits: &[u8]) -> Self;

    /// 各桁の和を計算する (Happy Numberなどで使用)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(123.digit_sum(), 6);
    /// ```
    fn digit_sum(self) -> u64;

    /// 数値の並びを反転させる (回文数判定などで使用)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(123.reverse(), 321);
    /// ```
    fn reverse(self) -> Self;

    /// 桁数を返す (文字列変換より高速)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(100.digits_len(), 3);
    /// ```
    fn digits_len(self) -> u32;

    /// 上からi番目の桁を取得する (0-indexed)
    ///
    /// # Example
    /// ```
    /// use keta::Keta;
    /// assert_eq!(12345.nth_digit(0), Some(1));
    /// assert_eq!(12345.nth_digit(4), Some(5));
    /// assert_eq!(12345.nth_digit(100), None);
    /// ```
    fn nth_digit(self, i: u32) -> Option<u8>;

    /// 数値を結合する
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
                fn digits(self) -> Vec<u8> {
                    if self == 0 { return vec![0]; }
                    let mut n = self;
                    // 最大桁数分のキャパシティ確保 (u128でも約40桁)
                    let mut ret = Vec::with_capacity(20);
                    while n > 0 {
                        ret.push((n % 10) as u8);
                        n /= 10;
                    }
                    ret.reverse();
                    ret
                }

                fn from_digits(digits: &[u8]) -> Self {
                    let mut ret: $t = 0;
                    for &d in digits {
                        ret = ret * 10 + (d as $t);
                    }
                    ret
                }

                fn digit_sum(self) -> u64 {
                    let mut n = self;
                    let mut sum: u64 = 0;
                    while n > 0 {
                        sum += (n % 10) as u64;
                        n /= 10;
                    }
                    sum
                }

                fn reverse(self) -> Self {
                    let mut n = self;
                    let mut ret: $t = 0;
                    while n > 0 {
                        ret = ret * 10 + (n % 10);
                        n /= 10;
                    }
                    ret
                }

                fn digits_len(self) -> u32 {
                    if self == 0 { return 1; }
                    self.ilog10() + 1
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
                fn digits(self) -> Vec<u8> {
                    if self == 0 { return vec![0]; }
                    let mut n = self.abs();
                    let mut ret = Vec::with_capacity(20);
                    while n > 0 {
                        ret.push((n % 10) as u8);
                        n /= 10;
                    }
                    ret.reverse();
                    ret
                }

                fn from_digits(digits: &[u8]) -> Self {
                    let mut ret: $t = 0;
                    for &d in digits {
                        ret = ret * 10 + (d as $t);
                    }
                    ret
                }

                fn digit_sum(self) -> u64 {
                    let mut n = self.abs();
                    let mut sum: u64 = 0;
                    while n > 0 {
                        sum += (n % 10) as u64;
                        n /= 10;
                    }
                    sum
                }

                fn reverse(self) -> Self {
                    let mut n = self.abs();
                    let mut ret: $t = 0;
                    while n > 0 {
                        ret = ret * 10 + (n % 10);
                        n /= 10;
                    }
                    if self < 0 { -ret } else { ret }
                }

                fn digits_len(self) -> u32 {
                    if self == 0 { return 1; }
                    self.abs().ilog10() + 1
                }

                fn nth_digit(self, i: u32) -> Option<u8> {
                    let l = self.digits_len();
                    if i >= l { return None; }
                    let pow = l - 1 - i;
                    Some(((self.abs() / (10 as $t).pow(pow)) % 10) as u8)
                }

                fn concat(self, other: Self) -> Self {
                    // 負の数の結合は定義が怪しいが、絶対値として後ろにつける実装にする
                    // 例: -12 concat 34 -> -1234
                    // 例: 12 concat -34 -> 1234 (符号は前の数字に依存)
                    let shift = other.digits_len();
                    let added = other.abs();
                    let shifted = self * (10 as $t).pow(shift);
                    if self < 0 {
                        shifted - added
                    } else {
                        shifted + added
                    }
                }
            }
        )*
    };
}

// ここで型を指定して実装！
impl_keta_uint!(u8, u16, u32, u64, u128, usize);
impl_keta_int!(i8, i16, i32, i64, i128, isize);

// ----------------------------------------------------------------
// テストコード
// ----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn test_concat() {
        assert_eq!(12_u64.concat(34), 1234);
        assert_eq!(1_u64.concat(0), 10);
    }

    #[test]
    fn test_nth_digit() {
        let n = 54321;
        assert_eq!(n.nth_digit(0), Some(5)); // 上から1桁目
        assert_eq!(n.nth_digit(4), Some(1)); // 上から5桁目
        assert_eq!(n.nth_digit(5), None); // 範囲外
    }
}
