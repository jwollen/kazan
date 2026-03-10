//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_pipeline_opacity_micromap.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_pipeline_opacity_micromap";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

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

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDevicePipelineOpacityMicromapFeaturesARM<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDevicePipelineOpacityMicromapFeaturesARM<'_> {}

    impl Default for PhysicalDevicePipelineOpacityMicromapFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePipelineOpacityMicromapFeaturesARM =
        PhysicalDevicePipelineOpacityMicromapFeaturesARM<'static>;
    impl PhysicalDevicePipelineOpacityMicromapFeaturesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePipelineOpacityMicromapFeaturesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
}
