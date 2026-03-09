pub mod vk;

#[cfg(feature = "ffi")]
pub use vk::ffi;

mod generated;
pub use generated::external::*;

mod chain;
pub use chain::*;

mod extensions;
pub use extensions::{ExtensionSet, UnknownExtensionError};

pub(crate) mod macros;

mod loading;

#[cfg(feature = "window")]
pub mod window;

pub mod util;

#[cfg(feature = "std")]
pub use util::read_spv;

use core::{
    ffi::{CStr, c_char},
    fmt,
    mem::MaybeUninit,
    ptr,
};

pub trait ExtendUninit<T> {
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>];

    unsafe fn set_len(&mut self, len: usize);
}

impl<T> ExtendUninit<T> for Vec<T> {
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>] {
        self.reserve(capacity.saturating_sub(self.capacity()));
        self.spare_capacity_mut()
    }

    unsafe fn set_len(&mut self, len: usize) {
        unsafe {
            self.set_len(self.len() + len);
        }
    }
}

impl<T> ExtendUninit<T> for &mut Vec<T> {
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>] {
        unsafe { ExtendUninit::reserve(*self, capacity) }
    }

    unsafe fn set_len(&mut self, len: usize) {
        unsafe { ExtendUninit::set_len(*self, len) }
    }
}

pub trait RawPtr<T> {
    fn to_raw_ptr(self) -> *const T;
}

impl<T> RawPtr<T> for Option<&T> {
    fn to_raw_ptr(self) -> *const T {
        match self {
            Some(inner) => inner,
            None => ptr::null(),
        }
    }
}

impl<T> RawPtr<T> for Option<&[T]> {
    fn to_raw_ptr(self) -> *const T {
        match self {
            Some(inner) => inner.as_ptr(),
            None => ptr::null(),
        }
    }
}

impl RawPtr<c_char> for Option<&CStr> {
    fn to_raw_ptr(self) -> *const c_char {
        match self {
            Some(inner) => inner.as_ptr(),
            None => ptr::null(),
        }
    }
}

pub trait RawMutPtr<T> {
    unsafe fn to_raw_mut_ptr(self) -> *mut T;
}

impl<T> RawMutPtr<T> for Option<&mut T> {
    unsafe fn to_raw_mut_ptr(self) -> *mut T {
        match self {
            Some(inner) => inner,
            None => ptr::null_mut(),
        }
    }
}

impl<T> RawMutPtr<T> for Option<&mut [T]> {
    unsafe fn to_raw_mut_ptr(self) -> *mut T {
        match self {
            Some(inner) => inner.as_mut_ptr(),
            None => ptr::null_mut(),
        }
    }
}

pub trait Handle: Sized {
    const TYPE: vk::ObjectType;
    fn as_raw(self) -> u64;
    fn from_raw(_: u64) -> Self;

    /// Returns whether the handle is a `NULL` value.
    ///
    /// # Example
    ///
    /// ```
    /// # use kazan::vk::{Handle, Instance};
    /// let instance = Instance::null();
    /// assert!(instance.is_null());
    /// ```
    fn is_null(self) -> bool {
        self.as_raw() == 0
    }
}

pub use loading::{Entry, MissingEntryPointError, StaticFn};

/// Helper for Debug-formatting bitflag types. Prints known flags by name,
/// separated by `|`, and appends any remaining unknown bits as hex.
pub fn debug_flags<F: Into<u64> + Copy>(
    f: &mut fmt::Formatter<'_>,
    known: &[(F, &str)],
    value: F,
) -> fmt::Result {
    let mut first = true;
    let mut remaining: u64 = value.into();
    for &(bit, name) in known {
        let bit: u64 = bit.into();
        if bit != 0 && remaining & bit == bit {
            if !first {
                f.write_str(" | ")?;
            }
            f.write_str(name)?;
            first = false;
            remaining &= !bit;
        }
    }
    if remaining != 0 {
        if !first {
            f.write_str(" | ")?;
        }
        write!(f, "{:#x}", remaining)?;
    } else if first {
        f.write_str("(empty)")?;
    }
    Ok(())
}

/// Converts a possibly-null `*const c_char` to `Option<&CStr>`.
///
/// # Safety
/// If non-null, the pointer must point to a valid nul-terminated C string.
#[inline]
pub(crate) unsafe fn as_c_str<'a>(ptr: *const c_char) -> Option<&'a CStr> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(ptr) })
    }
}

#[inline]
pub(crate) fn wrap_c_str_slice_until_nul(
    str: &[core::ffi::c_char],
) -> core::result::Result<&core::ffi::CStr, core::ffi::FromBytesUntilNulError> {
    // SAFETY: The cast from c_char to u8 is ok because a c_char is always one byte.
    let bytes = unsafe { core::slice::from_raw_parts(str.as_ptr().cast(), str.len()) };
    core::ffi::CStr::from_bytes_until_nul(bytes)
}

#[derive(Debug)]
pub struct CStrTooLargeForStaticArray {
    pub static_array_size: usize,
    pub c_str_size: usize,
}

#[cfg(feature = "std")]
impl std::error::Error for CStrTooLargeForStaticArray {}
impl core::fmt::Display for CStrTooLargeForStaticArray {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "static `c_char` target array of length `{}` is too small to write a `CStr` (with `NUL`-terminator) of length `{}`",
            self.static_array_size, self.c_str_size
        )
    }
}

#[inline]
pub(crate) fn write_c_str_slice_with_nul(
    target: &mut [core::ffi::c_char],
    str: &core::ffi::CStr,
) -> core::result::Result<(), CStrTooLargeForStaticArray> {
    let bytes = str.to_bytes_with_nul();
    // SAFETY: The cast from c_char to u8 is ok because a c_char is always one byte.
    let bytes = unsafe { core::slice::from_raw_parts(bytes.as_ptr().cast(), bytes.len()) };
    let static_array_size = target.len();
    target
        .get_mut(..bytes.len())
        .ok_or(CStrTooLargeForStaticArray {
            static_array_size,
            c_str_size: bytes.len(),
        })?
        .copy_from_slice(bytes);
    Ok(())
}

pub type Result<T> = core::result::Result<T, vk::Result>;
