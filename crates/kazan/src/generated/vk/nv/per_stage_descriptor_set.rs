//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_per_stage_descriptor_set.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_per_stage_descriptor_set";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePerStageDescriptorSetFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePerStageDescriptorSetFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub per_stage_descriptor_set: Bool32,
        pub dynamic_pipeline_layout: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePerStageDescriptorSetFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePerStageDescriptorSetFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("per_stage_descriptor_set", &self.per_stage_descriptor_set)
                .field("dynamic_pipeline_layout", &self.dynamic_pipeline_layout)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePerStageDescriptorSetFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDevicePerStageDescriptorSetFeaturesNV<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDevicePerStageDescriptorSetFeaturesNV<'_> {}

    impl Default for PhysicalDevicePerStageDescriptorSetFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                per_stage_descriptor_set: Default::default(),
                dynamic_pipeline_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePerStageDescriptorSetFeaturesNV<'a> {
        #[inline]
        pub fn per_stage_descriptor_set(mut self, per_stage_descriptor_set: bool) -> Self {
            self.per_stage_descriptor_set = per_stage_descriptor_set.into();
            self
        }

        #[inline]
        pub fn dynamic_pipeline_layout(mut self, dynamic_pipeline_layout: bool) -> Self {
            self.dynamic_pipeline_layout = dynamic_pipeline_layout.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePerStageDescriptorSetFeaturesNV =
        PhysicalDevicePerStageDescriptorSetFeaturesNV<'static>;
    impl PhysicalDevicePerStageDescriptorSetFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePerStageDescriptorSetFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}
