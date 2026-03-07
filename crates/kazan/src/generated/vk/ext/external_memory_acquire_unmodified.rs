//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_external_memory_acquire_unmodified.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_external_memory_acquire_unmodified";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryAcquireUnmodifiedEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalMemoryAcquireUnmodifiedEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub acquire_unmodified_memory: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalMemoryAcquireUnmodifiedEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalMemoryAcquireUnmodifiedEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("acquire_unmodified_memory", &self.acquire_unmodified_memory)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalMemoryAcquireUnmodifiedEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT;
    }

    unsafe impl<'a> Extends<BufferMemoryBarrier<'a>> for ExternalMemoryAcquireUnmodifiedEXT<'a> {}
    unsafe impl<'a> Extends<BufferMemoryBarrier2<'a>> for ExternalMemoryAcquireUnmodifiedEXT<'a> {}
    unsafe impl<'a> Extends<ImageMemoryBarrier<'a>> for ExternalMemoryAcquireUnmodifiedEXT<'a> {}
    unsafe impl<'a> Extends<ImageMemoryBarrier2<'a>> for ExternalMemoryAcquireUnmodifiedEXT<'a> {}

    impl Default for ExternalMemoryAcquireUnmodifiedEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                acquire_unmodified_memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalMemoryAcquireUnmodifiedEXT<'a> {
        #[inline]
        pub fn acquire_unmodified_memory(mut self, acquire_unmodified_memory: bool) -> Self {
            self.acquire_unmodified_memory = acquire_unmodified_memory.into();
            self
        }
    }
}
