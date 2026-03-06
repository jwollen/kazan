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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCubicClampFeaturesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceCubicClampFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cubic_range_clamp: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCubicClampFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceCubicClampFeaturesQCOM<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceCubicClampFeaturesQCOM<'a> {}
    impl Default for PhysicalDeviceCubicClampFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                cubic_range_clamp: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceCubicClampFeaturesQCOM<'a> {
        pub fn cubic_range_clamp(mut self, cubic_range_clamp: bool) -> Self {
            self.cubic_range_clamp = cubic_range_clamp.into();
            self
        }
    }
}
