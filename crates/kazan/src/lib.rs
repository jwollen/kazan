//! Vulkan bindings for Rust, generated from the official
//! [Vulkan-Headers](https://github.com/KhronosGroup/Vulkan-Headers) `vk.xml` registry.
//!
//! # Modules
//!
//! - [`vk`] -- all Vulkan types, constants, function pointer types, and Vulkan-specific traits
//!   ([`Handle`](vk::Handle), [`ApiVersion`](vk::ApiVersion), [`TaggedStructure`](vk::TaggedStructure),
//!   [`Extends`](vk::Extends), etc.), re-exported flat.
//!   - [`vk::vk1_0`], [`vk::vk1_1`], ... -- items grouped by API version.
//!   - [`vk::khr`], [`vk::ext`], [`vk::nv`], ... -- items grouped by vendor / individual extension.
//! - [`util`] -- SPIR-V decoding ([`read_spv`]).
//! - [`window`] -- platform surface creation via `raw-window-handle` (requires `window` feature).
//!
//! # Function tables
//!
//! Each API version and extension defines its own `InstanceFn` / `DeviceFn` struct. Load them
//! with the [`LoadInstanceFn`] and [`LoadDeviceFn`] traits. [`Entry`] bootstraps the loading
//! chain.
//!
//! # Quick start
//!
//! ```ignore
//! use kazan::{Entry, LoadDeviceFn as _, LoadInstanceFn as _, vk::{self, vk1_0}};
//!
//! let entry = unsafe { Entry::load()? };
//! let instance = entry.vk1_0.create_instance(&create_info, None)?;
//! let instance_fn = unsafe { vk1_0::InstanceFn::load(&entry, instance)? };
//! let device = unsafe { instance_fn.create_device(physical_device, &device_create_info, None)? };
//! let device_fn = unsafe { vk1_0::DeviceFn::load(&instance_fn, device)? };
//! ```

pub mod vk;

#[cfg(feature = "ffi")]
pub use vk::ffi;

#[allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::derivable_impls,
    clippy::get_first,
    clippy::let_and_return,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::missing_safety_doc,
    clippy::missing_transmute_annotations,
    clippy::needless_borrow,
    clippy::needless_option_as_deref,
    clippy::too_many_arguments,
    clippy::uninlined_format_args,
    clippy::useless_conversion,
    clippy::write_with_newline,
    dead_code,
    unused_imports,
    rustdoc::invalid_html_tags
)]
mod generated;
pub use generated::external::*;

mod array_cstr;
pub use array_cstr::ArrayCStr;

mod extensions;
pub use extensions::UnknownExtensionError;
pub use generated::extensions::{DeviceExtensionSet, InstanceExtensionSet};

pub(crate) mod macros;

mod loading;

/// Platform surface creation via `raw-window-handle`.
#[cfg(feature = "window")]
pub mod window;

/// Utility functions (SPIR-V decoding).
pub mod util;

#[cfg(feature = "std")]
pub use util::read_spv;

use core::{
    ffi::{CStr, c_char},
    mem::MaybeUninit,
    ptr,
};

/// Output buffer for Vulkan two-call enumerate commands.
///
/// The Vulkan enumerate pattern calls a command once to get the count, then again to fill a
/// buffer. Implementors of this trait provide the backing storage.
pub trait EnumerateInto<T> {
    /// Reserves capacity and returns the spare capacity as uninitialised memory.
    ///
    /// # Safety
    /// Caller must initialise the returned slice elements before calling [`set_len`](Self::set_len).
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>];

    /// Sets the length of the container after elements have been written.
    ///
    /// # Safety
    /// The first `len` elements beyond the current length must be initialised.
    unsafe fn set_len(&mut self, len: usize);
}

impl<T> EnumerateInto<T> for Vec<T> {
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

impl<T> EnumerateInto<T> for &mut Vec<T> {
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>] {
        unsafe { EnumerateInto::reserve(*self, capacity) }
    }

    unsafe fn set_len(&mut self, len: usize) {
        unsafe { EnumerateInto::set_len(*self, len) }
    }
}

/// Converts an optional reference to a raw const pointer, returning null for `None`.
pub trait RawPtr<T> {
    /// Returns a raw const pointer, or null if the value is absent.
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

/// Converts an optional mutable reference to a raw mutable pointer, returning null for `None`.
pub trait RawMutPtr<T> {
    /// Returns a raw mutable pointer, or null if the value is absent.
    ///
    /// # Safety
    /// The caller must ensure the pointer is not used after the referent is dropped.
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

/// An array parameter where the length is meaningful even without data.
///
/// Some Vulkan commands accept a count with a NULL pointer, meaning
/// "N entries with default/null values." This enum lets callers express both:
///
/// ```ignore
/// // "Bind 4 slots using default values"
/// cmd(SliceOrLen::Len(4));
/// // "Bind these specific entries"
/// cmd(SliceOrLen::Slice(&entries));
/// ```
pub enum SliceOrLen<'a, T> {
    /// A slice of values; the length is derived from the slice length.
    Slice(&'a [T]),
    /// Just a length with no data (NULL pointer -- N default/null values).
    Len(usize),
}

impl<T> SliceOrLen<'_, T> {
    /// Returns the number of elements (slice length or bare count).
    pub fn len(&self) -> usize {
        match self {
            SliceOrLen::Slice(s) => s.len(),
            SliceOrLen::Len(n) => *n,
        }
    }

    /// Returns `true` if the length is zero.
    pub fn is_empty(&self) -> bool {
        match self {
            SliceOrLen::Slice(s) => s.is_empty(),
            SliceOrLen::Len(n) => *n == 0,
        }
    }
}

impl<T> RawPtr<T> for SliceOrLen<'_, T> {
    fn to_raw_ptr(self) -> *const T {
        match self {
            SliceOrLen::Slice(s) => s.as_ptr(),
            SliceOrLen::Len(_) => ptr::null(),
        }
    }
}

impl<T> RawPtr<T> for Option<SliceOrLen<'_, T>> {
    fn to_raw_ptr(self) -> *const T {
        match self {
            Some(soc) => soc.to_raw_ptr(),
            None => ptr::null(),
        }
    }
}

pub use loading::{Entry, LoadDeviceFn, LoadInstanceFn, MissingEntryPointError, StaticFn};

/// Error returned when a `CStr` is too large for a fixed-size `c_char` array.
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

/// Convenience alias for `Result<T, vk::Result>`.
pub type Result<T> = core::result::Result<T, vk::Result>;

/// Set a bitfield region within an integer, with a range check on the value.
///
/// `OFFSET` is the bit position of the field's least-significant bit.
/// `WIDTH` is the number of bits in the field.
///
/// # Panics
///
/// Panics if `value` does not fit in `WIDTH` bits.
#[inline(always)]
pub(crate) fn set_bitfield<const OFFSET: u32, const WIDTH: u32>(field: &mut u32, value: u32) {
    assert!(
        value >> WIDTH == 0,
        "bitfield value {value:#x} does not fit in {WIDTH} bits",
    );
    const { assert!(OFFSET + WIDTH <= u32::BITS) };
    let mask = ((1u32 << WIDTH) - 1) << OFFSET;
    *field = (*field & !mask) | (value << OFFSET);
}

/// Set a single-bit bitfield from a `bool`.
#[inline(always)]
pub(crate) fn set_bitfield_bool<const OFFSET: u32>(field: &mut u32, value: bool) {
    const { assert!(OFFSET < u32::BITS) };
    let mask = 1u32 << OFFSET;
    *field = (*field & !mask) | ((value as u32) << OFFSET);
}

#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/long_lived_root_struct_borrow.rs");
}
