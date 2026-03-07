//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_external_memory.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_external_memory";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryImageCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalMemoryImageCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalMemoryHandleTypeFlagsNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalMemoryImageCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalMemoryImageCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_types", &self.handle_types)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalMemoryImageCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for ExternalMemoryImageCreateInfoNV<'a> {}

    impl Default for ExternalMemoryImageCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalMemoryImageCreateInfoNV<'a> {
        #[inline]
        pub fn handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlagsNV) -> Self {
            self.handle_types = handle_types;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMemoryAllocateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExportMemoryAllocateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalMemoryHandleTypeFlagsNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportMemoryAllocateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportMemoryAllocateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_types", &self.handle_types)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportMemoryAllocateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_MEMORY_ALLOCATE_INFO_NV;
    }

    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ExportMemoryAllocateInfoNV<'a> {}

    impl Default for ExportMemoryAllocateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMemoryAllocateInfoNV<'a> {
        #[inline]
        pub fn handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlagsNV) -> Self {
            self.handle_types = handle_types;
            self
        }
    }
}
