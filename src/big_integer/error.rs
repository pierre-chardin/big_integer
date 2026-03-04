//! Error types for conversion to big integers.

use core::fmt;
use std::error::Error;
use std::fmt::Display;

/// An error returned when parsing a string fails
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseBigIntegerError {
    pub(super) kind: BigIntegerErrorKind,
}

/// Enum to store the various types of errors that can cause parsing a big integer to fail.
///
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum BigIntegerErrorKind {
    /// Value being parsed is empty.
    ///
    Empty,
    /// Contains an invalid digit.
    InvalidDigit,
}

impl ParseBigIntegerError {
    /// Outputs the  cause of parsing a big integer failing.
    ///
    pub fn kind(&self) -> &BigIntegerErrorKind {
        &self.kind
    }
}

impl Display for ParseBigIntegerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            BigIntegerErrorKind::Empty => "cannot parse integer from empty string",
            BigIntegerErrorKind::InvalidDigit => "invalid digit found in string",
        }
        .fmt(f)
    }
}

impl Error for ParseBigIntegerError {}
