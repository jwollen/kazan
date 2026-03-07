#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_win32_keyed_mutex";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

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

    unsafe impl<'a> Extends<SubmitInfo<'a>> for Win32KeyedMutexAcquireReleaseInfoNV<'a> {}
    unsafe impl<'a> Extends<SubmitInfo2<'a>> for Win32KeyedMutexAcquireReleaseInfoNV<'a> {}

    impl Default for Win32KeyedMutexAcquireReleaseInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                acquire_count: Default::default(),
                p_acquire_syncs: core::ptr::null(),
                p_acquire_keys: core::ptr::null(),
                p_acquire_timeout_milliseconds: core::ptr::null(),
                release_count: Default::default(),
                p_release_syncs: core::ptr::null(),
                p_release_keys: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> Win32KeyedMutexAcquireReleaseInfoNV<'a> {
        #[inline]
        pub fn acquire_syncs(mut self, acquire_syncs: &'a [DeviceMemory]) -> Self {
            self.acquire_count = acquire_syncs.len().try_into().unwrap();
            self.p_acquire_syncs = acquire_syncs.as_ptr();
            self
        }

        #[inline]
        pub fn acquire_keys(mut self, acquire_keys: &'a [u64]) -> Self {
            self.acquire_count = acquire_keys.len().try_into().unwrap();
            self.p_acquire_keys = acquire_keys.as_ptr();
            self
        }

        #[inline]
        pub fn acquire_timeout_milliseconds(
            mut self,
            acquire_timeout_milliseconds: &'a [u32],
        ) -> Self {
            self.acquire_count = acquire_timeout_milliseconds.len().try_into().unwrap();
            self.p_acquire_timeout_milliseconds = acquire_timeout_milliseconds.as_ptr();
            self
        }

        #[inline]
        pub fn release_syncs(mut self, release_syncs: &'a [DeviceMemory]) -> Self {
            self.release_count = release_syncs.len().try_into().unwrap();
            self.p_release_syncs = release_syncs.as_ptr();
            self
        }

        #[inline]
        pub fn release_keys(mut self, release_keys: &'a [u64]) -> Self {
            self.release_count = release_keys.len().try_into().unwrap();
            self.p_release_keys = release_keys.as_ptr();
            self
        }
    }
}
