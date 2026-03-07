#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_present_metering";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSetPresentConfigNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SetPresentConfigNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub num_frames_per_batch: u32,
        pub present_config_feedback: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SetPresentConfigNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SetPresentConfigNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("num_frames_per_batch", &self.num_frames_per_batch)
                .field("present_config_feedback", &self.present_config_feedback)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SetPresentConfigNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SET_PRESENT_CONFIG_NV;
    }

    unsafe impl<'a> Extends<PresentInfoKHR<'a>> for SetPresentConfigNV<'a> {}

    impl Default for SetPresentConfigNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                num_frames_per_batch: Default::default(),
                present_config_feedback: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SetPresentConfigNV<'a> {
        pub fn num_frames_per_batch(mut self, num_frames_per_batch: u32) -> Self {
            self.num_frames_per_batch = num_frames_per_batch;
            self
        }

        pub fn present_config_feedback(mut self, present_config_feedback: u32) -> Self {
            self.present_config_feedback = present_config_feedback;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePresentMeteringFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePresentMeteringFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_metering: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePresentMeteringFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePresentMeteringFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_metering", &self.present_metering)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePresentMeteringFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePresentMeteringFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevicePresentMeteringFeaturesNV<'a> {}

    impl Default for PhysicalDevicePresentMeteringFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                present_metering: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePresentMeteringFeaturesNV<'a> {
        pub fn present_metering(mut self, present_metering: bool) -> Self {
            self.present_metering = present_metering.into();
            self
        }
    }
}
