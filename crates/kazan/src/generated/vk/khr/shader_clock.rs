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
    pub struct PhysicalDeviceShaderClockFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_subgroup_clock: Bool32,
        pub shader_device_clock: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceShaderClockFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR,
                p_next: core::ptr::null_mut(),
                shader_subgroup_clock: Default::default(),
                shader_device_clock: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderClockFeaturesKHR<'a> {
        pub fn shader_subgroup_clock(mut self, shader_subgroup_clock: Bool32) -> Self {
            self.shader_subgroup_clock = shader_subgroup_clock;
            self
        }
        pub fn shader_device_clock(mut self, shader_device_clock: Bool32) -> Self {
            self.shader_device_clock = shader_device_clock;
            self
        }
    }
}
