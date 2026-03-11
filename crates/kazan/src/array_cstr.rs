use core::ffi::{CStr, c_char};
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;

use crate::CStrTooLargeForStaticArray;

/// A fixed-size, NUL-terminated C string stored inline in an array of `N` bytes.
///
/// This is the Rust-side representation of Vulkan's fixed-size `char` arrays
/// (e.g. `char deviceName[VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]`). The name mirrors
/// `arrayvec::ArrayString` in spirit: an array-backed string type.
///
/// # Invariants
///
/// The array always contains at least one NUL byte. This is guaranteed by:
/// - [`Default`] producing an all-zero array (NUL at index 0).
/// - [`write_c_str`](Self::write_c_str) copying from a `&CStr` which includes a NUL.
/// - Vulkan spec requiring returned `char[N]` fields to be NUL-terminated.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ArrayCStr<const N: usize>([c_char; N]);

impl<const N: usize> ArrayCStr<N> {
    /// Returns the contents as a `&CStr`, scanning for the first NUL byte.
    ///
    /// # Panics
    ///
    /// Panics if no NUL byte is found, which indicates a broken invariant
    /// (e.g. a non-conforming Vulkan driver).
    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        // SAFETY: c_char and u8 are both one byte.
        let bytes = unsafe { core::slice::from_raw_parts(self.0.as_ptr().cast::<u8>(), N) };
        CStr::from_bytes_until_nul(bytes).expect("ArrayCStr: no NUL byte found")
    }

    /// Writes a `CStr` into this buffer, including its NUL terminator.
    ///
    /// Returns an error if the string (with NUL) does not fit in `N` bytes.
    #[inline]
    pub fn write_c_str(
        &mut self,
        s: &CStr,
    ) -> core::result::Result<(), CStrTooLargeForStaticArray> {
        let bytes = s.to_bytes_with_nul();
        // SAFETY: c_char and u8 are both one byte.
        let src =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr().cast::<c_char>(), bytes.len()) };
        self.0
            .get_mut(..src.len())
            .ok_or(CStrTooLargeForStaticArray {
                static_array_size: N,
                c_str_size: src.len(),
            })?
            .copy_from_slice(src);
        Ok(())
    }
}

impl<const N: usize> Default for ArrayCStr<N> {
    #[inline]
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Deref for ArrayCStr<N> {
    type Target = CStr;

    #[inline]
    fn deref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl<const N: usize> AsRef<CStr> for ArrayCStr<N> {
    #[inline]
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl<const N: usize> fmt::Debug for ArrayCStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_c_str(), f)
    }
}

impl<const N: usize> fmt::Display for ArrayCStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &byte in self.as_c_str().to_bytes() {
            let ch = byte as char;
            fmt::Write::write_char(f, ch)?;
        }
        Ok(())
    }
}

impl<const N: usize> PartialEq for ArrayCStr<N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_c_str() == other.as_c_str()
    }
}

impl<const N: usize> Eq for ArrayCStr<N> {}

impl<const N: usize> Hash for ArrayCStr<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_c_str().hash(state);
    }
}
