//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_pipeline_library.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_pipeline_library";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineLibraryCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineLibraryCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub library_count: u32,
        pub p_libraries: *const Pipeline,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineLibraryCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineLibraryCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("library_count", &self.library_count)
                .field("p_libraries", &self.p_libraries)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineLibraryCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_LIBRARY_CREATE_INFO_KHR;
    }

    unsafe impl Extends<GraphicsPipelineCreateInfo<'_>> for PipelineLibraryCreateInfoKHR<'_> {}

    impl Default for PipelineLibraryCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                library_count: Default::default(),
                p_libraries: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineLibraryCreateInfoKHR<'a> {
        #[inline]
        pub fn libraries(mut self, libraries: &'a [Pipeline]) -> Self {
            self.library_count = libraries.len().try_into().unwrap();
            self.p_libraries = libraries.as_ptr() as _;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPipelineLibraryCreateInfoKHR = PipelineLibraryCreateInfoKHR<'static>;
    impl PipelineLibraryCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineLibraryCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
