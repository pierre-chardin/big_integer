//! This module provides bindings useful when using GMP integers.
//! For details on GMP, see https://gmplib.org/.
//!

//TODO: Check visibility. This module should be internal to big_integer.

use libc::{c_char, c_int, c_long, c_ulong, size_t};
use std::ffi::{CStr, CString, c_void};
use std::mem;

/// Encapsulates a GMP integer.
///
#[repr(C)]
pub(super) struct MpzStruct {
    _mp_alloc: c_int,
    _mp_size: c_int,
    _mp_d: *mut c_void,
}

/// An unsigned integer value. Depending on platform, may be u32 or u64.
pub type UWord = c_ulong;
/// A signed integer value. Depending on platform, may be i32 or i64.
pub(super) type SWord = c_long;

/// Byte order to use for byte slices.
///
pub(super) enum ByteOrder {
    LittleEndian,
    NativeEndian,
    BigEndian,
}

#[inline]
fn byte_order_to_c_int(e: ByteOrder) -> c_int {
    match e {
        ByteOrder::LittleEndian => -1,
        ByteOrder::NativeEndian => DEFAULT_BYTE_ORDER,
        ByteOrder::BigEndian => 1,
    }
}

#[cfg(target_endian = "big")]
const DEFAULT_BYTE_ORDER: c_int = 1;

#[cfg(target_endian = "little")]
const DEFAULT_BYTE_ORDER: c_int = -1;

impl MpzStruct {
    /// Creates a GMP integer equal to 0.
    ///
    pub(super) fn new() -> MpzStruct {
        unsafe {
            let mut v = mem::MaybeUninit::<MpzStruct>::uninit();
            __gmpz_init(v.as_mut_ptr());
            v.assume_init()
        }
    }

    ///  Converts an unsigned integer value to a GMP integer.
    ///
    #[inline]
    pub(super) fn from_u_word(value: UWord) -> MpzStruct {
        unsafe {
            let mut v = mem::MaybeUninit::<MpzStruct>::uninit();
            __gmpz_init_set_ui(v.as_mut_ptr(), value);
            v.assume_init()
        }
    }

    ///  Converts a signed integer value to a GMP integer.
    ///
    #[inline]
    pub(super) fn from_s_word(value: SWord) -> MpzStruct {
        unsafe {
            let mut v = mem::MaybeUninit::<MpzStruct>::uninit();
            __gmpz_init_set_si(v.as_mut_ptr(), value);
            v.assume_init()
        }
    }

    /// Converts a byte slice to a GMP integer. Items are ordered with the most significant bytes
    /// first. The byte slice `positive_value`is always considered representing a positive integer.
    /// To get a negative GMP integer, set `sign_is_minus` to `true`.
    ///
    pub(super) fn from_bytes(
        sign_is_minus: bool,
        positive_value: &[u8],
        byte_order: ByteOrder,
    ) -> MpzStruct {
        let mut bint: MpzStruct;
        unsafe {
            let mut v = mem::MaybeUninit::<MpzStruct>::uninit();
            __gmpz_init(v.as_mut_ptr());
            bint = v.assume_init();
        }

        unsafe {
            __gmpz_import(
                &mut bint,
                positive_value.len(),
                byte_order_to_c_int(byte_order),
                size_of::<u8>(),
                0,
                0,
                positive_value.as_ptr() as *const c_void,
            );
            if sign_is_minus {
                __gmpz_neg(&mut bint, &bint);
            }
        }

        bint
    }

    /// Converts a string to a GMP integer for the given `radix`.  White space is allowed in
    /// the string, and is simply ignored. Returns `None` if string format is not valid.
    ///
    /// # Panics
    ///
    /// Panics if given a `radix` smaller than 2 or larger than 36.
    ///
    #[inline]
    pub(super) fn from_str_radix(src: &str, radix: u32) -> Option<MpzStruct> {
        from_str_radix_internal(src, to_gmp_radix(radix))
    }

    /// Returns the GMP integer converted to a lowercase string for the given `radix` and a boolean
    /// that is `true` if and only if the integer is nonnegative.
    ///
    /// If `radix`equal or less than 10, one should use instead [`to_string_radix()`].
    ///
    /// # Panics
    ///
    /// Panics if given a `radix` smaller than 2 or larger than 36.
    ///
    #[inline]
    pub(super) fn to_string_lowercase_radix(&self, radix: u32) -> (String, bool) {
        to_string_radix_internal(self, to_gmp_radix(radix))
    }

    /// Returns the GMP integer converted to an uppercase string for the given `radix` and a boolean
    /// that is `true` if and only if the integer is nonnegative.
    ///
    /// # Panics
    ///
    /// Panics if given a `radix` smaller than 2 or larger than 36.
    ///
    #[inline]
    pub(super) fn to_string_uppercase_radix(&self, radix: u32) -> (String, bool) {
        let (str, sign) = to_string_radix_internal(self, to_gmp_radix(radix));
        (str.to_uppercase(), sign)
    }

    /// Returns the GMP integer converted to a string (any character case) for the given `radix`
    /// and a boolean that is `true` if and only if the integer is nonnegative.
    ///
    /// # Panics
    ///
    /// Panics if given a `radix` smaller than 2 or larger than 36.
    ///
    #[inline]
    pub(super) fn to_string_radix(&self, radix: u32) -> (String, bool) {
        to_string_radix_internal(self, to_gmp_radix(radix))
    }

    ///  Returns the GMP integer * -1.
    ///
    #[inline]
    pub(super) fn neg(&self) -> Self {
        let mut result = MpzStruct::new();
        unsafe {
            __gmpz_neg(&mut result, self);
        }
        result
    }

    ///  Returns the GMP integer + `op`.
    ///
    #[inline]
    pub(super) fn add(&self, op: &Self) -> Self {
        let mut result = MpzStruct::new();
        unsafe {
            __gmpz_add(&mut result, self, op);
        }
        result
    }

    ///  Adds `op` to the GMP integer.
    ///
    #[inline]
    pub(super) fn add_assign(&mut self, op: &Self) {
        unsafe {
            __gmpz_add(self, self, op);
        }
    }

    ///  Returns the GMP integer * `op`.
    ///
    #[inline]
    pub(super) fn mul(&self, op: &Self) -> Self {
        let mut result = MpzStruct::new();
        unsafe {
            __gmpz_mul(&mut result, self, op);
        }
        result
    }

    ///  Multiplies the GMP integer by op.
    ///
    #[inline]
    pub(super) fn mul_assign(&mut self, op: &Self) {
        unsafe {
            __gmpz_mul(self, self, op);
        }
    }
}

/// Drop trait.
/// Frees all allocated memory.
///
impl Drop for MpzStruct {
    fn drop(&mut self) {
        unsafe {
            __gmpz_clear(self);
        }
    }
}

/// Clone trait.
///
impl Clone for MpzStruct {
    fn clone(&self) -> Self {
        unsafe {
            let mut v = mem::MaybeUninit::<MpzStruct>::uninit();
            __gmpz_init_set(v.as_mut_ptr(), self);
            v.assume_init()
        }
    }
}

/// Trait for equality operator.
///
impl PartialEq for MpzStruct {
    fn eq(&self, other: &Self) -> bool {
        unsafe { __gmpz_cmp(self, other) == 0 }
    }
}

/// Returns the radix safely cast to GMP.
/// Panics if radix is invalid.
///
#[inline]
fn to_gmp_radix(radix: u32) -> c_int {
    if (radix < 2) || (radix > 36) {
        panic!("Radix must be between 2 and 36 (inclusive)");
    }
    radix as c_int
}

/// Converts a GMP integer to a string for the given `radix`. Does not check if `radix` is valid.
///
fn from_str_radix_internal(src: &str, radix: c_int) -> Option<MpzStruct> {
    // GMP does not accept a leading `+` but accepts a leading `-`.
    let src = src.trim_start().trim_start_matches('+');

    let src_c_str = CString::new(src).unwrap(); // Variable must NOT be dropped before the C function is done with the pointer.
    unsafe {
        let mut v = mem::MaybeUninit::<MpzStruct>::uninit();
        if __gmpz_init_set_str(v.as_mut_ptr(), src_c_str.as_ptr(), radix as c_int) != 0 {
            // The variable is initialized even if an error occurs so call mpz_clear().
            __gmpz_clear(v.as_mut_ptr());
            // Invalid format.
            return None;
        }
        Some(v.assume_init())
    }
}

/// Returns the GMP integer converted to a lowercase string for the given `radix` and a boolean
/// that is `true` if and only if the integer is nonnegative. Does not check if `radix` is valid.
///
fn to_string_radix_internal(value: &MpzStruct, radix: c_int) -> (String, bool) {
    let str: String;
    let is_nonnegative: bool;
    unsafe {
        // Build the "C" string.
        let str_c = __gmpz_get_str(std::ptr::null_mut(), radix, value);

        let start_str_c;
        if *str_c == b'-' as c_char {
            // Remove leading '-'.
            start_str_c = str_c.add(1);
            is_nonnegative = false
        } else {
            start_str_c = str_c;
            is_nonnegative = true;
        }

        // Convert the "C" string to a RUST string.
        str = CStr::from_ptr(start_str_c).to_str().unwrap().to_owned();

        // Free the memory allocated by GMP.
        libc::free(str_c as *mut c_void);
    }
    (str, is_nonnegative)
}

// "C" bindings to GMP library ////////////////////////////////////////////////

#[link(name = "gmp")]
unsafe extern "C" {
    fn __gmpz_init(x: *mut MpzStruct);
    fn __gmpz_init_set(rop: *mut MpzStruct, op: *const MpzStruct);
    fn __gmpz_init_set_ui(rop: *mut MpzStruct, op: c_ulong);
    fn __gmpz_init_set_si(rop: *mut MpzStruct, op: c_long);
    fn __gmpz_init_set_str(rop: *mut MpzStruct, s: *const c_char, base: c_int) -> c_int;
    fn __gmpz_clear(x: *mut MpzStruct);

    // fn __gmpz_realloc2(x: *mut MpzStruct, n: c_ulong);
    // fn __gmpz_size(x: *mut MpzStruct) -> c_int;
    // fn __gmpz_set(rop: *mut MpzStruct, op: *const MpzStruct);
    // fn __gmpz_set_str(rop: *mut MpzStruct, s: *const c_char, base: c_int) -> c_int;
    fn __gmpz_get_str(s: *mut c_char, base: c_int, op: *const MpzStruct) -> *mut c_char;
    // fn __gmpz_get_ui(op: *const MpzStruct) -> c_ulong;
    // fn __gmpz_fits_ulong_p(op: *const MpzStruct) -> c_int;
    // fn __gmpz_get_si(op: *const MpzStruct) -> c_ulong;
    // fn __gmpz_get_d(op: *const MpzStruct) -> c_double;
    // fn __gmpz_fits_slong_p(op: *const MpzStruct) -> c_long;
    // fn __gmpz_sizeinbase(op: *const MpzStruct, base: c_int) -> size_t;
    fn __gmpz_cmp(op1: *const MpzStruct, op2: *const MpzStruct) -> c_int;
    // fn __gmpz_cmp_ui(op1: *const MpzStruct, op2: c_ulong) -> c_int;
    fn __gmpz_add(rop: *mut MpzStruct, op1: *const MpzStruct, op2: *const MpzStruct);
    // fn __gmpz_add_ui(rop: *mut MpzStruct, op1: *const MpzStruct, op2: c_ulong);
    fn __gmpz_sub(rop: *mut MpzStruct, op1: *const MpzStruct, op2: *const MpzStruct);
    // fn __gmpz_sub_ui(rop: *mut MpzStruct, op1: *const MpzStruct, op2: c_ulong);
    // fn __gmpz_ui_sub(rop: *mut MpzStruct, op1: c_ulong, op2: *const MpzStruct);
    fn __gmpz_mul(rop: *mut MpzStruct, op1: *const MpzStruct, op2: *const MpzStruct);
    // fn __gmpz_mul_ui(rop: *mut MpzStruct, op1: *const MpzStruct, op2: c_ulong);
    // fn __gmpz_mul_si(rop: *mut MpzStruct, op1: *const MpzStruct, op2: c_long);
    // fn __gmpz_mul_2exp(rop: *mut MpzStruct, op1: *const MpzStruct, op2: c_ulong);
    // fn __gmpz_addmul(rop: *mut MpzStruct, op1: *const MpzStruct, op2: c_long);
    // fn __gmpz_addmul_ui(rop: *mut MpzStruct, op1: *const MpzStruct, op2: c_ulong);
    fn __gmpz_neg(rop: *mut MpzStruct, op: *const MpzStruct);
    // fn __gmpz_abs(rop: *mut MpzStruct, op: *const MpzStruct);
    // fn __gmpz_tdiv_q(q: *mut MpzStruct, n: *const MpzStruct, d: *const MpzStruct);
    // fn __gmpz_tdiv_r(r: *mut MpzStruct, n: *const MpzStruct, d: *const MpzStruct);
    // fn __gmpz_tdiv_q_ui(q: *mut MpzStruct, n: *const MpzStruct, d: c_ulong);
    // fn __gmpz_tdiv_r_ui(r: *mut MpzStruct, n: *const MpzStruct, d: c_ulong);
    // fn __gmpz_fdiv_r(r: *mut MpzStruct, n: *const MpzStruct, d: *const MpzStruct);
    // fn __gmpz_fdiv_q_2exp(q: *mut MpzStruct, n: *const MpzStruct, b: c_ulong);
    // fn __gmpz_mod(r: *mut MpzStruct, n: *const MpzStruct, d: *const MpzStruct);
    // fn __gmpz_divisible_p(n: *const MpzStruct, d: *const MpzStruct) -> c_int;
    // fn __gmpz_and(rop: *mut MpzStruct, op1: *const MpzStruct, op2: *const MpzStruct);
    // fn __gmpz_ior(rop: *mut MpzStruct, op1: *const MpzStruct, op2: *const MpzStruct);
    // fn __gmpz_xor(rop: *mut MpzStruct, op1: *const MpzStruct, op2: *const MpzStruct);
    // fn __gmpz_com(rop: *mut MpzStruct, op: *const MpzStruct);
    // fn __gmpz_popcount(op: *const MpzStruct) -> c_ulong;
    // fn __gmpz_pow_ui(rop: *mut MpzStruct, base: *const MpzStruct, exp: c_ulong);
    // fn __gmpz_ui_pow_ui(rop: *mut MpzStruct, base: c_ulong, exp: c_ulong);
    // fn __gmpz_powm(
    //     rop: *mut MpzStruct,
    //     base: *const MpzStruct,
    //     exp: *const MpzStruct,
    //     modulo: *const MpzStruct,
    // );
    // fn __gmpz_powm_sec(
    //     rop: *mut MpzStruct,
    //     base: *const MpzStruct,
    //     exp: *const MpzStruct,
    //     modulo: *const MpzStruct,
    // );
    // fn __gmpz_hamdist(op1: *const MpzStruct, op2: *const MpzStruct) -> c_ulong;
    // fn __gmpz_setbit(rop: *mut MpzStruct, bit_index: c_ulong);
    // fn __gmpz_clrbit(rop: *mut MpzStruct, bit_index: c_ulong);
    // fn __gmpz_combit(rop: *mut MpzStruct, bit_index: c_ulong);
    // fn __gmpz_tstbit(rop: *const MpzStruct, bit_index: c_ulong) -> c_int;
    // fn __gmpz_probab_prime_p(n: *const MpzStruct, reps: c_int) -> c_int;
    // fn __gmpz_nextprime(rop: *mut MpzStruct, op: *const MpzStruct);
    // fn __gmpz_gcd(rop: *mut MpzStruct, op1: *const MpzStruct, op2: *const MpzStruct);
    // fn __gmpz_gcdext(
    //     g: *mut MpzStruct,
    //     s: *mut MpzStruct,
    //     t: *mut MpzStruct,
    //     a: *const MpzStruct,
    //     b: *const MpzStruct,
    // );
    // fn __gmpz_lcm(rop: *mut MpzStruct, op1: *const MpzStruct, op2: *const MpzStruct);
    // fn __gmpz_invert(rop: *mut MpzStruct, op1: *const MpzStruct, op2: *const MpzStruct) -> c_int;
    fn __gmpz_import(
        rop: *mut MpzStruct,
        count: size_t,
        order: c_int,
        size: size_t,
        endian: c_int,
        nails: size_t,
        op: *const c_void,
    );
    // fn __gmpz_export(
    //     rop: *mut c_void,
    //     countp: *mut size_t,
    //     order: c_int,
    //     size: size_t,
    //     endian: c_int,
    //     nails: size_t,
    //     op: *const MpzStruct,
    // );
    // fn __gmpz_root(rop: *mut MpzStruct, op: *const MpzStruct, n: c_ulong) -> c_int;
    // fn __gmpz_sqrt(rop: *mut MpzStruct, op: *const MpzStruct);
    // fn __gmpz_millerrabin(n: *const MpzStruct, reps: c_int) -> c_int;
}
