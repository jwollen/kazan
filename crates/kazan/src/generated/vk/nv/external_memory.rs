#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExternalMemoryImageCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalMemoryHandleTypeFlagsNV,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for ExternalMemoryImageCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
                p_next: core::ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExternalMemoryImageCreateInfoNV<'a> {
        pub fn handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlagsNV) -> Self {
            self.handle_types = handle_types;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportMemoryAllocateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalMemoryHandleTypeFlagsNV,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for ExportMemoryAllocateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::EXPORT_MEMORY_ALLOCATE_INFO_NV,
                p_next: core::ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExportMemoryAllocateInfoNV<'a> {
        pub fn handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlagsNV) -> Self {
            self.handle_types = handle_types;
            self
        }
    }
}
