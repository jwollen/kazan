#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineLibraryCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub library_count: u32,
        pub p_libraries: *const Pipeline,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineLibraryCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_LIBRARY_CREATE_INFO_KHR;
    }
    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>> for PipelineLibraryCreateInfoKHR<'a> {}
    impl Default for PipelineLibraryCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                library_count: Default::default(),
                p_libraries: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineLibraryCreateInfoKHR<'a> {
        pub fn libraries(mut self, libraries: &'a [Pipeline]) -> Self {
            self.library_count = libraries.len().try_into().unwrap();
            self.p_libraries = libraries.as_ptr();
            self
        }
    }
}
