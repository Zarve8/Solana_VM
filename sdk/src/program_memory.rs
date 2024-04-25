//! Basic low-level memory operations.
//!
//! Within the SBF environment, these are implemented as syscalls and executed by
//! the runtime in native code.

/// Like C `memcpy`.
///
/// # Arguments
///
/// - `dst` - Destination
/// - `src` - Source
/// - `n` - Number of bytes to copy
///
/// # Errors
///
/// When executed within a SBF program, the memory regions spanning `n` bytes
/// from from the start of `dst` and `src` must be mapped program memory. If not,
/// the program will abort.
///
/// The memory regions spanning `n` bytes from `dst` and `src` from the start
/// of `dst` and `src` must not overlap. If they do, then the program will abort
/// or, if run outside of the SBF VM, will panic.
///
/// # Safety
///
/// __This function is incorrectly missing an `unsafe` declaration.__
///
/// This function does not verify that `n` is less than or equal to the
/// lengths of the `dst` and `src` slices passed to it &mdash; it will copy
/// bytes to and from beyond the slices.
///
/// Specifying an `n` greater than either the length of `dst` or `src` will
/// likely introduce undefined behavior.
#[inline]
pub fn sol_memcpy(dst: &mut [u8], src: &[u8], n: usize) {
    crate::custom::sol_memcpy(dst.as_mut_ptr(), src.as_ptr(), n);
}

/// Like C `memmove`.
///
/// # Arguments
///
/// - `dst` - Destination
/// - `src` - Source
/// - `n` - Number of bytes to copy
///
/// # Errors
///
/// When executed within a SBF program, the memory regions spanning `n` bytes
/// from from `dst` and `src` must be mapped program memory. If not, the program
/// will abort.
///
/// # Safety
///
/// The same safety rules apply as in [`ptr::copy`].
///
/// [`ptr::copy`]: https://doc.rust-lang.org/std/ptr/fn.copy.html
#[inline]
pub unsafe fn sol_memmove(dst: *mut u8, src: *mut u8, n: usize) {
    crate::custom::sol_memmove(dst, src, n);
}

/// Like C `memcmp`.
///
/// # Arguments
///
/// - `s1` - Slice to be compared
/// - `s2` - Slice to be compared
/// - `n` - Number of bytes to compare
///
/// # Errors
///
/// When executed within a SBF program, the memory regions spanning `n` bytes
/// from from the start of `dst` and `src` must be mapped program memory. If not,
/// the program will abort.
///
/// # Safety
///
/// __This function is incorrectly missing an `unsafe` declaration.__
///
/// It does not verify that `n` is less than or equal to the lengths of the
/// `dst` and `src` slices passed to it &mdash; it will read bytes beyond the
/// slices.
///
/// Specifying an `n` greater than either the length of `dst` or `src` will
/// likely introduce undefined behavior.
#[inline]
pub fn sol_memcmp(s1: &[u8], s2: &[u8], n: usize) -> i32 {
    let mut result = 0;
    crate::custom::sol_memcmp(s1.as_ptr(), s2.as_ptr(), n, &mut result as *mut i32);
    result
}

/// Like C `memset`.
///
/// # Arguments
///
/// - `s` - Slice to be set
/// - `c` - Repeated byte to set
/// - `n` - Number of bytes to set
///
/// # Errors
///
/// When executed within a SBF program, the memory region spanning `n` bytes
/// from from the start of `s` must be mapped program memory. If not, the program
/// will abort.
///
/// # Safety
///
/// __This function is incorrectly missing an `unsafe` declaration.__
///
/// This function does not verify that `n` is less than or equal to the length
/// of the `s` slice passed to it &mdash; it will write bytes beyond the
/// slice.
///
/// Specifying an `n` greater than the length of `s` will likely introduce
/// undefined behavior.
#[inline]
pub fn sol_memset(s: &mut [u8], c: u8, n: usize) {
    crate::custom::sol_memset(s.as_mut_ptr(), c, n);
}
