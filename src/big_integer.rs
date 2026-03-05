//! Struct `BigInteger` is an arbitrary-precision unsigned integer limited only by currently
//! available heap-allocated memory.
//!

pub mod error;
mod gmp_integer;

use std::fmt::{Binary, Debug, Display, LowerHex, Octal, UpperHex};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::str::FromStr;

use crate::big_integer::error::{BigIntegerErrorKind, ParseBigIntegerError};
use crate::big_integer::gmp_integer::{ByteOrder, SWord, UWord};
use gmp_integer::MpzStruct;

/// Struct `BigInteger` is an arbitrary-precision integer.
/// It provides arithmetic operations and conversion from various RUST native types.
///
#[derive(Clone, PartialEq)]
pub struct BigInteger {
    data: MpzStruct,
}

// Implementation /////////////////////////////////////////////////////////////

impl BigInteger {
    /// Returns value `0`.
    ///
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: MpzStruct::new(),
        }
    }

    /// Returns value `1`.
    ///
    #[inline]
    pub fn one() -> Self {
        Self {
            data: MpzStruct::from_u_word(1),
        }
    }

    /// Parses an integer from a string slice with digits in a given `radix`. Returns `None`if
    /// parsing error.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 36.
    ///
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseBigIntegerError> {
        if let Some(data) = MpzStruct::from_str_radix(src, radix) {
            return Ok(Self { data });
        }

        if src.is_empty() {
            return Err(ParseBigIntegerError {
                kind: BigIntegerErrorKind::Empty,
            });
        }
        Err(ParseBigIntegerError {
            kind: BigIntegerErrorKind::InvalidDigit,
        })
    }

    ///Returns the quotient and remainder of `self/other`.
    ///
    pub fn div_rem(&self, other: &BigInteger) -> (BigInteger, BigInteger) {
        let (quotient, remainer) = self.data.div_rem(&other.data);
        (Self { data: quotient }, Self { data: remainer })
    }
}

/// Default value is zero.
impl Default for BigInteger {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

// Traits related to strings //////////////////////////////////////////////////

/// Display trait. Automatically implements to_string().
///
impl Display for BigInteger {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (str, is_nonnegative) = self.data.to_string_radix(10);
        f.pad_integral(is_nonnegative, "", &str)
    }
}

/// Debug trait.
impl Debug for BigInteger {
    // Note: currently the standard library does not allow to reliably detect {:x?}, {:X?}, so
    // decimal is always assumed. If these formats are required, one should use {:x}, {:X}.
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

/// LowerHex trait.
/// For positive values, format will be the same as for sign or unsigned RUST builtin integers.
/// Negative values are displayed with a negative sign (not its 2's complement).
///
impl LowerHex for BigInteger {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (str, is_nonnegative) = self.data.to_string_lowercase_radix(16);
        f.pad_integral(is_nonnegative, "0x", &str)
    }
}

/// UpperHex trait.
/// For positive values, format will be the same as for sign or unsigned RUST builtin integers.
/// Negative values are displayed with a negative sign (not its 2's complement).
///
impl UpperHex for BigInteger {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (str, is_nonnegative) = self.data.to_string_uppercase_radix(16);
        f.pad_integral(is_nonnegative, "0x", &str)
    }
}

/// Octal trait.
/// For positive values, format will be the same as for sign or unsigned RUST builtin integers.
/// Negative values are displayed with a negative sign (not its 2's complement).
///
impl Octal for BigInteger {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (str, is_nonnegative) = self.data.to_string_radix(8);
        f.pad_integral(is_nonnegative, "0o", &str)
    }
}

/// Binary trait.
/// For positive values, format will be the same as for sign or unsigned RUST builtin integers.
/// Negative values are displayed with a negative sign (not its 2's complement).
///
impl Binary for BigInteger {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (str, is_nonnegative) = self.data.to_string_radix(2);
        f.pad_integral(is_nonnegative, "0b", &str)
    }
}

/// From string strait.
///
impl FromStr for BigInteger {
    type Err = ParseBigIntegerError;

    #[inline]
    fn from_str(src: &str) -> Result<Self, ParseBigIntegerError> {
        Self::from_str_radix(src, 10)
    }
}

/// From trait for u128 type.
///
impl From<u128> for BigInteger {
    #[inline]
    fn from(value: u128) -> Self {
        Self {
            data: MpzStruct::from_bytes(false, &value.to_ne_bytes(), ByteOrder::NativeEndian),
        }
    }
}

/// From trait for ii128 type.
///
impl From<i128> for BigInteger {
    fn from(value: i128) -> Self {
        let (u_value, sign_is_minus) = if value >= 0 {
            (value as u128, false)
        } else {
            (-value as u128, true)
        };

        Self {
            data: MpzStruct::from_bytes(
                sign_is_minus,
                &u_value.to_ne_bytes(),
                ByteOrder::NativeEndian,
            ),
        }
    }
}

/// From trait for u64 type.
///
impl From<u64> for BigInteger {
    fn from(value: u64) -> Self {
        // Compiler will remove dead code if UWord is an u64.

        if value <= UWord::MAX {
            return Self {
                data: MpzStruct::from_u_word(value as UWord),
            };
        }

        Self {
            data: MpzStruct::from_bytes(false, &value.to_ne_bytes(), ByteOrder::NativeEndian),
        }
    }
}

/// From trait for i64 type.
///
impl From<i64> for BigInteger {
    fn from(value: i64) -> Self {
        // Compiler will remove dead code if SWord is an i64.

        if (value >= SWord::MIN) && (value <= SWord::MAX) {
            return Self {
                data: MpzStruct::from_s_word(value as SWord),
            };
        }

        let (u_value, sign_is_minus) = if value >= 0 {
            (value as u64, false)
        } else {
            (-value as u64, true)
        };

        Self {
            data: MpzStruct::from_bytes(
                sign_is_minus,
                &u_value.to_ne_bytes(),
                ByteOrder::NativeEndian,
            ),
        }
    }
}

/// From trait for u32 type.
///
impl From<u32> for BigInteger {
    #[inline]
    fn from(value: u32) -> Self {
        Self {
            data: MpzStruct::from_u_word(value as UWord),
        }
    }
}

/// From trait for i32 type.
///
impl From<i32> for BigInteger {
    #[inline]
    fn from(value: i32) -> Self {
        Self {
            data: MpzStruct::from_s_word(value as SWord),
        }
    }
}

/// From trait for u16 type.
///
impl From<u16> for BigInteger {
    #[inline]
    fn from(value: u16) -> Self {
        Self {
            data: MpzStruct::from_u_word(value as UWord),
        }
    }
}

/// From trait for i16 type.
///
impl From<i16> for BigInteger {
    #[inline]
    fn from(value: i16) -> Self {
        Self {
            data: MpzStruct::from_s_word(value as SWord),
        }
    }
}

/// From trait for u8 type.
///
impl From<u8> for BigInteger {
    #[inline]
    fn from(value: u8) -> Self {
        Self {
            data: MpzStruct::from_u_word(value as UWord),
        }
    }
}

/// From trait for i8 type.
///
impl From<i8> for BigInteger {
    #[inline]
    fn from(value: i8) -> Self {
        Self {
            data: MpzStruct::from_s_word(value as SWord),
        }
    }
}

/// From trait for bool type.
///
impl From<bool> for BigInteger {
    #[inline]
    fn from(value: bool) -> Self {
        Self {
            data: MpzStruct::from_u_word(value as UWord),
        }
    }
}

/// From trait for char type.
///
impl From<char> for BigInteger {
    #[inline]
    fn from(value: char) -> Self {
        Self {
            data: MpzStruct::from_u_word(value as UWord),
        }
    }
}

/// Negation trait for expression `-&a`.
///
impl Neg for &BigInteger {
    type Output = BigInteger;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {
            data: self.data.neg(),
        }
    }
}

// Addition traits ////////////////////////////////////////////////////////////

/// Addition trait for expression `a + &b`. Variable `a` is moved.
///
impl Add<&BigInteger> for BigInteger {
    type Output = BigInteger;

    #[inline]
    fn add(self, b: &BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.add(&b.data),
        }
    }
}

/// Addition trait for expression `self + b`. Variables `self` and `b` are moved.
///
impl Add<BigInteger> for BigInteger {
    type Output = BigInteger;

    #[inline]
    fn add(self, b: BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.add(&b.data),
        }
    }
}

/// Addition trait for expression `&self + &b`.
///
impl Add<&BigInteger> for &BigInteger {
    type Output = BigInteger;

    #[inline]
    fn add(self, b: &BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.add(&b.data),
        }
    }
}

/// Addition trait for expression `&self + b`. Variable `b` is moved.
///
impl Add<BigInteger> for &BigInteger {
    type Output = BigInteger;

    #[inline]
    fn add(self, b: BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.add(&b.data),
        }
    }
}

/// Addition assignment trait for expression `self += &b`.
///
impl AddAssign<&BigInteger> for BigInteger {
    #[inline]
    fn add_assign(&mut self, b: &BigInteger) {
        self.data.add_assign(&b.data);
    }
}

// Subtraction traits /////////////////////////////////////////////////////////

/// Substraction trait for expression `a - &b`. Variable `a` is moved.
///
impl Sub<&BigInteger> for BigInteger {
    type Output = BigInteger;

    fn sub(self, b: &BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.sub(&b.data),
        }
    }
}

/// Substraction trait for expression `self - b`. Variables `self` and `b` are moved.
///
impl Sub<BigInteger> for BigInteger {
    type Output = BigInteger;

    fn sub(self, b: BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.sub(&b.data),
        }
    }
}

/// Substraction trait for expression `&self - &b`.
///
impl Sub<&BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn sub(self, b: &BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.sub(&b.data),
        }
    }
}

/// Substraction trait for expression `&self - b`. Variable `b` is moved.
///
impl Sub<BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn sub(self, b: BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.sub(&b.data),
        }
    }
}

/// Substraction assignment trait for expression `self -= &b`.
///
impl SubAssign<&BigInteger> for BigInteger {
    fn sub_assign(&mut self, b: &BigInteger) {
        self.data.sub_assign(&b.data);
    }
}

// Multiplication traits //////////////////////////////////////////////////////

/// Multiplication trait for expression `self * &b`. Variable `self` is moved.
///
impl Mul<&BigInteger> for BigInteger {
    type Output = BigInteger;

    #[inline]
    fn mul(self, b: &BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.mul(&b.data),
        }
    }
}

/// Multiplication trait for expression `self * b`. Variables `self`  and `b` are moved.
///
impl Mul<BigInteger> for BigInteger {
    type Output = BigInteger;

    #[inline]
    fn mul(self, b: BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.mul(&b.data),
        }
    }
}

/// Multiplication trait for expression `&self * &b`.
///
impl Mul<&BigInteger> for &BigInteger {
    type Output = BigInteger;

    #[inline]
    fn mul(self, b: &BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.mul(&b.data),
        }
    }
}

/// Multiplication trait for expression `&self * b`. Variable `b` is moved.
///
impl Mul<BigInteger> for &BigInteger {
    type Output = BigInteger;

    #[inline]
    fn mul(self, b: BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.mul(&b.data),
        }
    }
}

/// Multiplication assignment trait for expression `self *= &b`.
///
impl MulAssign<&BigInteger> for BigInteger {
    #[inline]
    fn mul_assign(&mut self, b: &BigInteger) {
        self.data.mul_assign(&b.data);
    }
}

// Division traits ////////////////////////////////////////////////////////////

/// Division trait for expression `self / &b`. Variable `self` is moved.
///
impl Div<&BigInteger> for BigInteger {
    type Output = BigInteger;

    #[inline]
    fn div(self, b: &BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.div(&b.data),
        }
    }
}

/// Division trait for expression `self / b`. Variables `self`  and `b` are moved.
///
impl Div<BigInteger> for BigInteger {
    type Output = BigInteger;

    #[inline]
    fn div(self, b: BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.div(&b.data),
        }
    }
}

/// Division trait for expression `&self / &b`.
///
impl Div<&BigInteger> for &BigInteger {
    type Output = BigInteger;

    #[inline]
    fn div(self, b: &BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.div(&b.data),
        }
    }
}

/// Division trait for expression `&self / b`. Variable `b` is moved.
///
impl Div<BigInteger> for &BigInteger {
    type Output = BigInteger;

    #[inline]
    fn div(self, b: BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.div(&b.data),
        }
    }
}

/// Division assignment trait for expression `self /= &b`.
///
impl DivAssign<&BigInteger> for BigInteger {
    #[inline]
    fn div_assign(&mut self, b: &BigInteger) {
        self.data.div_assign(&b.data);
    }
}

// Remainder traits ////////////////////////////////////////////////////////////

/// Remainder trait for expression `self % &b`. Variable `self` is moved.
///
impl Rem<&BigInteger> for BigInteger {
    type Output = BigInteger;

    #[inline]
    fn rem(self, b: &BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.rem(&b.data),
        }
    }
}

/// Remainder trait for expression `self % b`. Variables `self`  and `b` are moved.
///
impl Rem<BigInteger> for BigInteger {
    type Output = BigInteger;

    #[inline]
    fn rem(self, b: BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.rem(&b.data),
        }
    }
}

/// Remainder trait for expression `&self % &b`.
///
impl Rem<&BigInteger> for &BigInteger {
    type Output = BigInteger;

    #[inline]
    fn rem(self, b: &BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.rem(&b.data),
        }
    }
}

/// Remainder trait for expression `&self % b`. Variable `b` is moved.
///
impl Rem<BigInteger> for &BigInteger {
    type Output = BigInteger;

    #[inline]
    fn rem(self, b: BigInteger) -> Self::Output {
        Self::Output {
            data: self.data.rem(&b.data),
        }
    }
}

/// Remainder assignment trait for expression `self /= &b`.
///
impl RemAssign<&BigInteger> for BigInteger {
    #[inline]
    fn rem_assign(&mut self, b: &BigInteger) {
        self.data.rem_assign(&b.data);
    }
}
