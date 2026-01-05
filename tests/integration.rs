use keta::Keta;

#[test]
fn test_digits() {
    assert_eq!(12345.digits(), vec![1, 2, 3, 4, 5]);
    assert_eq!(0.digits(), vec![0]);
    assert_eq!((-123).digits(), vec![1, 2, 3]);
}

#[test]
fn test_digit_sum() {
    assert_eq!(123.digit_sum(), 6);
    assert_eq!(0.digit_sum(), 0);
}

#[test]
fn test_digit_product() {
    assert_eq!(1234.digit_product(), 24);
    assert_eq!(103.digit_product(), 0);
    assert_eq!(0.digit_product(), 0);
}

#[test]
fn test_from_digits() {
    assert_eq!(u64::from_digits(&[1, 2, 3]), 123);
    assert_eq!(u64::from_digits(&[0]), 0);
}

#[test]
fn test_reverse() {
    assert_eq!(123.reverse(), 321);
    assert_eq!((-123).reverse(), -321);
}

#[test]
fn test_digits_len() {
    assert_eq!(123.digits_len(), 3);
    assert_eq!(1000.digits_len(), 4);
    assert_eq!(0.digits_len(), 1);
    assert_eq!((-123).digits_len(), 3);
}

#[test]
fn test_nth_digit() {
    assert_eq!(12345.nth_digit(0), Some(1));
    assert_eq!(12345.nth_digit(4), Some(5));
    assert_eq!(12345.nth_digit(5), None);
}

#[test]
fn test_concat() {
    assert_eq!(12.concat(34), 1234);
    assert_eq!((-12).concat(34), -1234);
}

#[test]
fn test_contains_digit() {
    assert!(12345.contains_digit(3));
    assert!(!12345.contains_digit(9));
    assert!(0.contains_digit(0));
}

#[test]
fn test_make_max() {
    assert_eq!(2026.make_max(), 6220);
}

#[test]
fn test_make_min() {
    assert_eq!(2026.make_min(), 226);
}

#[test]
fn test_is_palindrome() {
    assert!(121.is_palindrome());
    assert!(!123.is_palindrome());
}

#[test]
fn test_digits_radix() {
    // 6 (10) -> 110 (2)
    assert_eq!(6.digits_radix(2), vec![1, 1, 0]);
    // 255 (10) -> FF (16)
    assert_eq!(255.digits_radix(16), vec![15, 15]);
    // 0 (10) -> 0 (n)
    assert_eq!(0.digits_radix(2), vec![0]);
}

#[test]
fn test_from_digits_radix() {
    assert_eq!(u64::from_digits_radix(&[1, 1, 0], 2), 6);
    assert_eq!(u64::from_digits_radix(&[15, 15], 16), 255);
}

#[test]
fn test_digit_sum_radix() {
    assert_eq!(6.digit_sum_radix(2), 2); // 110 -> 1+1+0=2
}

#[test]
fn test_digit_product_radix() {
    assert_eq!(7.digit_product_radix(2), 1); // 111 -> 1*1*1=1
    assert_eq!(6.digit_product_radix(2), 0); // 110 -> 1*1*0=0
}

#[test]
fn test_digits_len_radix() {
    assert_eq!(16.digits_len_radix(2), 5); // 10000
    assert_eq!(15.digits_len_radix(2), 4); // 1111
    assert_eq!(0.digits_len_radix(2), 1);
}
