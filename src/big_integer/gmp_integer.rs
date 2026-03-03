//! This module provides bindings useful when using GMP integers.
//! For details on GMP, see https://gmplib.org/.
//!

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

impl MpzStruct {
    /// Creates a GMP integer equal to 0.
    ///
    pub fn new() -> MpzStruct {
        unsafe {
            let mut v = mem::MaybeUninit::<MpzStruct>::uninit();
            __gmpz_init(v.as_mut_ptr());
            v.assume_init()
        }
    }

    // TODO: Implement from_u64 and from_i64. Note: On windows, c_ulong=u32 and c_long=i32.

    ///  Converts an `u32` value to a GMP integer.
    ///
    #[inline]
    pub fn from_u32(value: u32) -> MpzStruct {
        from_c_ulong(value as c_ulong)
    }

    ///  Converts an `i32` value to a GMP integer.
    ///
    #[inline]
    pub fn from_i32(value: i32) -> MpzStruct {
        from_c_long(value as c_long)
    }

    ///  Converts an `u64` value to a GMP integer.
    ///
    pub fn from_u64(value: u64) -> MpzStruct {
        // Compiler should optimize code if c_ulong is u64.

        #[allow(clippy::unnecessary_cast)] // On some platforms c_ulong is not an u64.
        if value <= c_ulong::MAX as u64 {
            return from_c_ulong(value as c_ulong);
        }

        MpzStruct::from_u128(value as u128)
    }

    ///  Converts an `i64` value to a GMP integer.
    ///
    pub fn from_i64(value: i64) -> MpzStruct {
        // Compiler should optimize code if c_ulong is u64.

        #[allow(clippy::unnecessary_cast)] // On some platforms c_ulong is not an u64.
        if value > c_long::MAX as i64 {
            return MpzStruct::from_u128(value as u128);
        }

        #[allow(clippy::unnecessary_cast)] // On some platforms c_ulong is not an u64.
        if value < c_long::MIN as i64 {
            let mut bint = MpzStruct::from_u128((-value) as u128);
            unsafe { __gmpz_neg(&mut bint, &bint) };
            return bint;
        }

        from_c_ulong(value as c_ulong)
    }

    ///  Converts an `u128` value to a GMP integer.
    ///
    pub fn from_u128(mut value: u128) -> MpzStruct {
        let mut array: Vec<c_ulong> = Vec::new();
        loop {
            array.push(value as c_ulong);
            value >>= c_ulong::BITS;
            if value == 0 {
                break;
            }
        }

        from_c_long_array(&array)
    }

    ///  Converts an `i128` value to a GMP integer.
    ///
    pub fn from_i128(value: i128) -> MpzStruct {
        if value >= 0 {
            return MpzStruct::from_u128(value as u128);
        }

        let mut bint = MpzStruct::from_u128((-value) as u128);
        unsafe { __gmpz_neg(&mut bint, &bint) };
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
    pub fn from_str_radix(src: &str, radix: u32) -> Option<MpzStruct> {
        from_str_radix_internal(src, to_gmp_radix(radix))
    }

    /// Converts a GMP integer to a lowercase string for the given `radix`.
    ///
    /// # Panics
    ///
    /// Panics if given a `radix` smaller than 2 or larger than 36.
    ///
    #[inline]
    pub fn to_string_lowercase_radix(self: &Self, radix: u32) -> String {
        to_string_radix_internal(self, to_gmp_radix(radix))
    }

    /// Converts a GMP integer to an uppercase string for the given `radix`.
    ///
    /// # Panics
    ///
    /// Panics if given a `radix` smaller than 2 or larger than 36.
    ///
    #[inline]
    pub fn to_string_uppercase_radix(self: &Self, radix: u32) -> String {
        to_string_radix_internal(self, to_gmp_radix(radix)).to_uppercase()
    }

    /// Converts a GMP integer to a string for the given `radix`. Any character case may be used.
    ///
    /// # Panics
    ///
    /// Panics if given a `radix` smaller than 2 or larger than 36.
    ///
    #[inline]
    pub fn to_string_radix(self: &Self, radix: u32) -> String {
        to_string_radix_internal(self, to_gmp_radix(radix))
    }
}

/// Frees all allocated memory.
///
impl Drop for MpzStruct {
    fn drop(&mut self) {
        unsafe {
            __gmpz_clear(self);
        }
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

/// Converts an `c_ulong` value to a GMP integer.
///
fn from_c_ulong(value: c_ulong) -> MpzStruct {
    unsafe {
        let mut v = mem::MaybeUninit::<MpzStruct>::uninit();
        __gmpz_init_set_ui(v.as_mut_ptr(), value);
        v.assume_init()
    }
}

/// Converts an `c_long` value to a GMP integer.
///
fn from_c_long(value: c_long) -> MpzStruct {
    unsafe {
        let mut v = mem::MaybeUninit::<MpzStruct>::uninit();
        __gmpz_init_set_si(v.as_mut_ptr(), value);
        v.assume_init()
    }
}

/// Converts an array of `c_ulong` to a positive GMP integer.
fn from_c_long_array(value: &[c_ulong]) -> MpzStruct {
    let mut bint: MpzStruct;
    unsafe {
        let mut v = mem::MaybeUninit::<MpzStruct>::uninit();
        __gmpz_init(v.as_mut_ptr());
        bint = v.assume_init();
    }

    unsafe {
        __gmpz_import(
            &mut bint,
            value.len(),
            -1,
            size_of::<c_ulong>(),
            0,
            0,
            value.as_ptr() as *const c_void,
        )
    }

    bint
}

/// Converts a GMP integer  to a string for the given `radix`. Does not check if `radix` is valid.
/// Return string is always lowercase.
///
fn to_string_radix_internal(value: &MpzStruct, radix: c_int) -> String {
    let str: String;
    unsafe {
        // Build the "C" string.
        let str_c = __gmpz_get_str(std::ptr::null_mut(), radix, value);

        // Convert the "C" string to a RUST string.
        str = CStr::from_ptr(str_c).to_str().unwrap().to_owned();

        // Free the memory allocated by GMP.
        libc::free(str_c as *mut c_void);
    }
    str
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
    // fn __gmpz_cmp(op1: *const MpzStruct, op2: *const MpzStruct) -> c_int;
    // fn __gmpz_cmp_ui(op1: *const MpzStruct, op2: c_ulong) -> c_int;
    // fn __gmpz_add(rop: *mut MpzStruct, op1: *const MpzStruct, op2: *const MpzStruct);
    fn __gmpz_add_ui(rop: *mut MpzStruct, op1: *const MpzStruct, op2: c_ulong);
    // fn __gmpz_sub(rop: *mut MpzStruct, op1: *const MpzStruct, op2: *const MpzStruct);
    // fn __gmpz_sub_ui(rop: *mut MpzStruct, op1: *const MpzStruct, op2: c_ulong);
    // fn __gmpz_ui_sub(rop: *mut MpzStruct, op1: c_ulong, op2: *const MpzStruct);
    // fn __gmpz_mul(rop: *mut MpzStruct, op1: *const MpzStruct, op2: *const MpzStruct);
    fn __gmpz_mul_ui(rop: *mut MpzStruct, op1: *const MpzStruct, op2: c_ulong);
    fn __gmpz_mul_si(rop: *mut MpzStruct, op1: *const MpzStruct, op2: c_long);
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
