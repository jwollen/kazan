//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_win32_keyed_mutex.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_win32_keyed_mutex";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWin32KeyedMutexAcquireReleaseInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct Win32KeyedMutexAcquireReleaseInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub acquire_count: u32,
        pub p_acquire_syncs: *const DeviceMemory,
        pub p_acquire_keys: *const u64,
        pub p_acquire_timeout_milliseconds: *const u32,
        pub release_count: u32,
        pub p_release_syncs: *const DeviceMemory,
        pub p_release_keys: *const u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for Win32KeyedMutexAcquireReleaseInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Win32KeyedMutexAcquireReleaseInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("acquire_count", &self.acquire_count)
                .field("p_acquire_syncs", &self.p_acquire_syncs)
                .field("p_acquire_keys", &self.p_acquire_keys)
                .field(
                    "p_acquire_timeout_milliseconds",
                    &self.p_acquire_timeout_milliseconds,
                )
                .field("release_count", &self.release_count)
                .field("p_release_syncs", &self.p_release_syncs)
                .field("p_release_keys", &self.p_release_keys)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for Win32KeyedMutexAcquireReleaseInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV;
    }

    unsafe impl Extends<SubmitInfo<'_>> for Win32KeyedMutexAcquireReleaseInfoNV<'_> {}
    unsafe impl Extends<SubmitInfo2<'_>> for Win32KeyedMutexAcquireReleaseInfoNV<'_> {}

    impl Default for Win32KeyedMutexAcquireReleaseInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                acquire_count: Default::default(),
                p_acquire_syncs: ptr::null(),
                p_acquire_keys: ptr::null(),
                p_acquire_timeout_milliseconds: ptr::null(),
                release_count: Default::default(),
                p_release_syncs: ptr::null(),
                p_release_keys: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> Win32KeyedMutexAcquireReleaseInfoNV<'a> {
        #[inline]
        pub fn acquires(
            mut self,
            acquire_syncs: &'a [DeviceMemory],
            acquire_keys: &'a [u64],
            acquire_timeout_milliseconds: &'a [u32],
        ) -> Self {
            self.acquire_count = acquire_syncs.len().try_into().unwrap();
            assert_eq!(acquire_keys.len(), self.acquire_count as usize);
            assert_eq!(
                acquire_timeout_milliseconds.len(),
                self.acquire_count as usize
            );
            self.p_acquire_syncs = acquire_syncs.as_ptr() as _;
            self.p_acquire_keys = acquire_keys.as_ptr() as _;
            self.p_acquire_timeout_milliseconds = acquire_timeout_milliseconds.as_ptr() as _;
            self
        }

        #[inline]
        pub fn releases(
            mut self,
            release_syncs: &'a [DeviceMemory],
            release_keys: &'a [u64],
        ) -> Self {
            self.release_count = release_syncs.len().try_into().unwrap();
            assert_eq!(release_keys.len(), self.release_count as usize);
            self.p_release_syncs = release_syncs.as_ptr() as _;
            self.p_release_keys = release_keys.as_ptr() as _;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkWin32KeyedMutexAcquireReleaseInfoNV = Win32KeyedMutexAcquireReleaseInfoNV<'static>;
    impl Win32KeyedMutexAcquireReleaseInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkWin32KeyedMutexAcquireReleaseInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}
