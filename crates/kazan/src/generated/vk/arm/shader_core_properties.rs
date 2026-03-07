#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_shader_core_properties";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderCorePropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderCorePropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pixel_rate: u32,
        pub texel_rate: u32,
        pub fma_rate: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderCorePropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderCorePropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pixel_rate", &self.pixel_rate)
                .field("texel_rate", &self.texel_rate)
                .field("fma_rate", &self.fma_rate)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderCorePropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceShaderCorePropertiesARM<'a>
    {
    }

    impl Default for PhysicalDeviceShaderCorePropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                pixel_rate: Default::default(),
                texel_rate: Default::default(),
                fma_rate: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderCorePropertiesARM<'a> {
        pub fn pixel_rate(mut self, pixel_rate: u32) -> Self {
            self.pixel_rate = pixel_rate;
            self
        }

        pub fn texel_rate(mut self, texel_rate: u32) -> Self {
            self.texel_rate = texel_rate;
            self
        }

        pub fn fma_rate(mut self, fma_rate: u32) -> Self {
            self.fma_rate = fma_rate;
            self
        }
    }
}
