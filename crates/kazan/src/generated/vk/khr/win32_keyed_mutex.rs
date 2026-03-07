//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_win32_keyed_mutex.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_win32_keyed_mutex";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWin32KeyedMutexAcquireReleaseInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct Win32KeyedMutexAcquireReleaseInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub acquire_count: u32,
        pub p_acquire_syncs: *const DeviceMemory,
        pub p_acquire_keys: *const u64,
        pub p_acquire_timeouts: *const u32,
        pub release_count: u32,
        pub p_release_syncs: *const DeviceMemory,
        pub p_release_keys: *const u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for Win32KeyedMutexAcquireReleaseInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Win32KeyedMutexAcquireReleaseInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("acquire_count", &self.acquire_count)
                .field("p_acquire_syncs", &self.p_acquire_syncs)
                .field("p_acquire_keys", &self.p_acquire_keys)
                .field("p_acquire_timeouts", &self.p_acquire_timeouts)
                .field("release_count", &self.release_count)
                .field("p_release_syncs", &self.p_release_syncs)
                .field("p_release_keys", &self.p_release_keys)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for Win32KeyedMutexAcquireReleaseInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR;
    }

    unsafe impl<'a> Extends<SubmitInfo<'a>> for Win32KeyedMutexAcquireReleaseInfoKHR<'a> {}
    unsafe impl<'a> Extends<SubmitInfo2<'a>> for Win32KeyedMutexAcquireReleaseInfoKHR<'a> {}

    impl Default for Win32KeyedMutexAcquireReleaseInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                acquire_count: Default::default(),
                p_acquire_syncs: ptr::null(),
                p_acquire_keys: ptr::null(),
                p_acquire_timeouts: ptr::null(),
                release_count: Default::default(),
                p_release_syncs: ptr::null(),
                p_release_keys: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> Win32KeyedMutexAcquireReleaseInfoKHR<'a> {
        #[inline]
        pub fn acquires(
            mut self,
            acquire_syncs: &'a [DeviceMemory],
            acquire_keys: &'a [u64],
            acquire_timeouts: &'a [u32],
        ) -> Self {
            self.acquire_count = acquire_syncs.len().try_into().unwrap();
            assert_eq!(acquire_keys.len(), self.acquire_count as usize);
            assert_eq!(acquire_timeouts.len(), self.acquire_count as usize);
            self.p_acquire_syncs = acquire_syncs.as_ptr();
            self.p_acquire_keys = acquire_keys.as_ptr();
            self.p_acquire_timeouts = acquire_timeouts.as_ptr();
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
            self.p_release_syncs = release_syncs.as_ptr();
            self.p_release_keys = release_keys.as_ptr();
            self
        }
    }
}
