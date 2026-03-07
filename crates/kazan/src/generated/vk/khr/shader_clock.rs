#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_shader_clock";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderClockFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderClockFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_subgroup_clock: Bool32,
        pub shader_device_clock: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderClockFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderClockFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_subgroup_clock", &self.shader_subgroup_clock)
                .field("shader_device_clock", &self.shader_device_clock)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderClockFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceShaderClockFeaturesKHR<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderClockFeaturesKHR<'a> {}

    impl Default for PhysicalDeviceShaderClockFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_subgroup_clock: Default::default(),
                shader_device_clock: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderClockFeaturesKHR<'a> {
        #[inline]
        pub fn shader_subgroup_clock(mut self, shader_subgroup_clock: bool) -> Self {
            self.shader_subgroup_clock = shader_subgroup_clock.into();
            self
        }

        #[inline]
        pub fn shader_device_clock(mut self, shader_device_clock: bool) -> Self {
            self.shader_device_clock = shader_device_clock.into();
            self
        }
    }
}
