//! Struct `BigInteger` is an arbitrary-precision unsigned integer limited only by currently
//! available heap-allocated memory.
//!

pub mod error;
mod gmp_integer;

use core::fmt;
use std::error::Error;
use std::fmt::{Binary, Debug, Display, Formatter, LowerHex, Octal, UpperHex};
use std::num::{IntErrorKind, ParseIntError};
use std::ops::{Add, AddAssign, Mul};
use std::str::FromStr;

use crate::big_integer::error::{BigIntegerErrorKind, ParseBigIntegerError};
use gmp_integer::MpzStruct;

/// Struct `BigInteger` is an arbitrary-precision integer.
/// It provides operation similar to RUST operators for interger primitive types (i32, i64, i128...).
///
//TODO: #[derive(Clone, PartialEq)]
pub struct BigInteger {
    data: MpzStruct,
}

// Implementation /////////////////////////////////////////////////////////////

impl BigInteger {
    /// Returns value `0`.
    ///
    pub fn zero() -> Self {
        BigInteger {
            data: MpzStruct::new(),
        }
    }

    /// Returns value `1`.
    ///
    pub fn one() -> Self {
        BigInteger {
            data: MpzStruct::from_u32(1),
        }
    }

    /// Parses an integer from a string slice with digits in a given `radix`. Returns `None`if
    /// parsing error.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 36.
    ///
    pub fn from_str_radix(src: &str, radix: u32) -> Result<BigInteger, ParseBigIntegerError> {
        if let Some(data) = MpzStruct::from_str_radix(src, radix) {
            return Ok(BigInteger { data });
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
}

/// Default value is zero.
impl Default for BigInteger {
    fn default() -> Self {
        BigInteger::zero()
    }
}

// Traits related to strings //////////////////////////////////////////////////

/// Display trait. Automatically implements to_string().
///
impl Display for BigInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad_integral(true, "", &self.data.to_string_radix(10))
    }
}

/// Debug trait.
impl Debug for BigInteger {
    // Note: currently the standard library does not allow to reliably detect {:x?}, {:X?},  so
    // decimal is always assumed. If these formats are required, one should use {:x}, {:X}.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad_integral(true, "", &self.data.to_string_radix(10))
    }
}

/// LowerHex trait.
///
impl LowerHex for BigInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad_integral(true, "0x", &self.data.to_string_lowercase_radix(16))
    }
}

/// UpperHex trait.
///
impl UpperHex for BigInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad_integral(true, "0x", &self.data.to_string_uppercase_radix(16))
    }
}

/// Octal trait.
///
impl Octal for BigInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad_integral(true, "0o", &self.data.to_string_radix(8))
    }
}

/// Binary trait.
///
impl Binary for BigInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad_integral(true, "0b", &self.data.to_string_radix(2))
    }
}

/// From string strait.
///
impl FromStr for BigInteger {
    type Err = ParseBigIntegerError;

    fn from_str(src: &str) -> Result<BigInteger, ParseBigIntegerError> {
        Self::from_str_radix(src, 10)
    }
}

/// From trait for u128 type.
///
impl From<u128> for BigInteger {
    fn from(value: u128) -> Self {
        BigInteger {
            data: u128_to_big_unsigned(value),
        }
    }
}

/// From trait for u64 type.
///
impl From<u64> for BigInteger {
    fn from(value: u64) -> Self {
        BigInteger {
            data: u64_to_big_unsigned(value),
        }
    }
}

/// From trait for u32 type.
///
impl From<u32> for BigInteger {
    fn from(value: u32) -> Self {
        BigInteger {
            data: u32_to_big_unsigned(value),
        }
    }
}

/// From trait for u16 type.
///
impl From<u16> for BigInteger {
    fn from(value: u16) -> Self {
        BigInteger {
            data: u16_to_big_unsigned(value),
        }
    }
}

/// From trait for u8 type.
///
impl From<u8> for BigInteger {
    fn from(value: u8) -> Self {
        BigInteger {
            data: u8_to_big_unsigned(value),
        }
    }
}

/// From trait for bool type.
///
impl From<bool> for BigInteger {
    fn from(value: bool) -> Self {
        BigInteger {
            data: u32_to_big_unsigned(u32::from(value)),
        }
    }
}

/// From trait for char type.
///
impl From<char> for BigInteger {
    fn from(value: char) -> Self {
        BigInteger {
            data: u32_to_big_unsigned(u32::from(value)),
        }
    }
}

// Addition traits ////////////////////////////////////////////////////////////

/// Addition trait for expression `a + &b`. Variable `a` is moved.
///
impl Add<&BigInteger> for BigInteger {
    type Output = BigInteger;

    fn add(mut self, b: &BigInteger) -> Self::Output {
        add_assign_internal(&mut self.data, &b.data);
        self
    }
}

/// Addition trait for expression `a + b`. Variables `a`  and `b` are moved.
///
impl Add<BigInteger> for BigInteger {
    type Output = BigInteger;

    fn add(mut self, b: BigInteger) -> Self::Output {
        add_assign_internal(&mut self.data, &b.data);
        self
    }
}

/// Addition trait for expression `&a + &b`.
///
impl Add<&BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn add(self, b: &BigInteger) -> Self::Output {
        BigInteger {
            data: add_internal(&self.data, &b.data),
        }
    }
}

/// Addition trait for expression `&a + b`. Variable `b` is moved.
///
impl Add<BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn add(self, b: BigInteger) -> Self::Output {
        BigInteger {
            data: add_internal(&self.data, &b.data),
        }
    }
}

/// Addition assignment trait for expression `a += &b`.
///
impl AddAssign<&BigInteger> for BigInteger {
    fn add_assign(&mut self, b: &BigInteger) {
        add_assign_internal(&mut self.data, &b.data);
    }
}

// Multiplication traits //////////////////////////////////////////////////////

/// Multiplication trait for expression `a * &b`. Variable `a` is moved.
///
impl Mul<&BigInteger> for BigInteger {
    type Output = BigInteger;

    fn mul(mut self, b: &BigInteger) -> Self::Output {
        mul_assign_internal(&mut self.data, &b.data);
        self
    }
}

/// Multiplication trait for expression `a * b`. Variables `a`  and `b` are moved.
///
impl Mul<BigInteger> for BigInteger {
    type Output = BigInteger;

    fn mul(mut self, b: BigInteger) -> Self::Output {
        mul_assign_internal(&mut self.data, &b.data);
        self
    }
}

/// Multiplication trait for expression `&a * &b`.
///
impl Mul<&BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn mul(self, b: &BigInteger) -> Self::Output {
        BigInteger {
            data: mul_internal(&self.data, &b.data),
        }
    }
}

/// Multiplication trait for expression `&a * b`. Variable `b` is moved.
///
impl Mul<BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn mul(self, b: BigInteger) -> Self::Output {
        BigInteger {
            data: mul_internal(&self.data, &b.data),
        }
    }
}
