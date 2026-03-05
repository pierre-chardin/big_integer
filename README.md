# Big Integer

A library for arbitrary-precision integer calculations.

# Dependencies

## GMP Library

### Introduction

GMP is a multiple precision arithmetic library. It must br installed as a dynamic library.

This program was tested with GMP `6.3.0`.

See: https://gmplib.org/

### Installation on macOS

To install GMP on macOS, use `Homebrew` with following commands:

```
brew update
brew install gmp
```

# TODO

## Associated functions ##

* Operators `>`, `>=`, `<`, `<=` and associated methods.
* Operators `|`, `&`, `^` and associated methods.
* Operators `<<`, `>>` and associated methods.
* `to_be_bytes()`,  `to_le_bytes()`, `to_ne_bytes()`.

## Methods ##

* fn div_rem_assign(a: &mut BigInteger, b: &BigInteger) -> remainder: BigInteger (returns remainder)
* Operators `&=`, `|=`, `^=` and associated methods.
* Operators `<<=`, `>>=` and associated methods.
* `pow(exponent: u32)`.
* `rotate_left(n:u32)` and `rotate_righ(n:u32)`. Check that these are not from traits.
* Trait ToOwned`.
* Trait `Hash`.
* `div_euclid()`, `div_ceil()`.
* Trait `Ord` (min, max).
* `midpoint(v: BigUnsiged)`.
* `is_multiple_of(value);`.
* `is_power_of_two()`.

## Other ##

* Check document generation.
* Build and test on linux 32 and 64 bits.
* Build and test on Windows 64 bits.

## Const ##

How can I create a const of big unsigned? Seems similar to String versus str.


