// Integration test for `big_integer`.

use big_integer::big_integer::BigInteger;
use big_integer::big_integer::error::BigIntegerErrorKind;
use std::str::FromStr;

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn div_by_zero_1() {
    let a = BigInteger::from_str("32712965432211132231").unwrap();
    let b = a / BigInteger::zero();
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn div_by_zero_2() {
    let a = BigInteger::from_str("32712965432211132231").unwrap();
    let b = a / &BigInteger::zero();
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn div_by_zero_3() {
    let a = BigInteger::from_str("32712965432211132231").unwrap();
    let b = &a / BigInteger::zero();
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn div_by_zero_4() {
    let a = BigInteger::from_str("32712965432211132231").unwrap();
    let b = &a / &BigInteger::zero();
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn div_by_zero_assign() {
    let mut a = BigInteger::from_str("32712965432211132231").unwrap();
    a /= &BigInteger::zero();
}

#[test]
#[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
fn rem_by_zero_1() {
    let a = BigInteger::from_str("32712965432211132231").unwrap();
    let b = a % BigInteger::zero();
}

#[test]
#[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
fn rem_by_zero_2() {
    let a = BigInteger::from_str("32712965432211132231").unwrap();
    let b = a % &BigInteger::zero();
}

#[test]
#[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
fn rem_by_zero_3() {
    let a = BigInteger::from_str("32712965432211132231").unwrap();
    let b = &a % BigInteger::zero();
}

#[test]
#[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
fn rem_by_zero_4() {
    let a = BigInteger::from_str("32712965432211132231").unwrap();
    let b = &a % &BigInteger::zero();
}

#[test]
#[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
fn rem_by_zero_assign() {
    let mut a = BigInteger::from_str("32712965432211132231").unwrap();
    a %= &BigInteger::zero();
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn div_rem_by_zero_assign() {
    let mut a = BigInteger::from_str("32712965432211132231").unwrap();
    let qr = a.div_rem(&BigInteger::zero());
}
