//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_pipeline_library_group_handles.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_pipeline_library_group_handles";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_library_group_handles: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "pipeline_library_group_handles",
                    &self.pipeline_library_group_handles,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR<'_>
    {
    }

    impl Default for PhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                pipeline_library_group_handles: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR<'a> {
        #[inline]
        pub fn pipeline_library_group_handles(
            mut self,
            pipeline_library_group_handles: bool,
        ) -> Self {
            self.pipeline_library_group_handles = pipeline_library_group_handles.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR =
        PhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR<'static>;
    impl PhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
