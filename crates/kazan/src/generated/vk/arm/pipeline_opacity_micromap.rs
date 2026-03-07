//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_pipeline_opacity_micromap.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_pipeline_opacity_micromap";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePipelineOpacityMicromapFeaturesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePipelineOpacityMicromapFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_opacity_micromap: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePipelineOpacityMicromapFeaturesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePipelineOpacityMicromapFeaturesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pipeline_opacity_micromap", &self.pipeline_opacity_micromap)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelineOpacityMicromapFeaturesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePipelineOpacityMicromapFeaturesARM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDevicePipelineOpacityMicromapFeaturesARM<'a>
    {
    }

    impl Default for PhysicalDevicePipelineOpacityMicromapFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                pipeline_opacity_micromap: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePipelineOpacityMicromapFeaturesARM<'a> {
        #[inline]
        pub fn pipeline_opacity_micromap(mut self, pipeline_opacity_micromap: bool) -> Self {
            self.pipeline_opacity_micromap = pipeline_opacity_micromap.into();
            self
        }
    }
}
