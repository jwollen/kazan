#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_shader_fma";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderFmaFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderFmaFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_fma_float16: Bool32,
        pub shader_fma_float32: Bool32,
        pub shader_fma_float64: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceShaderFmaFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderFmaFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_fma_float16", &self.shader_fma_float16)
                .field("shader_fma_float32", &self.shader_fma_float32)
                .field("shader_fma_float64", &self.shader_fma_float64)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderFmaFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceShaderFmaFeaturesKHR<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderFmaFeaturesKHR<'a> {}

    impl Default for PhysicalDeviceShaderFmaFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_fma_float16: Default::default(),
                shader_fma_float32: Default::default(),
                shader_fma_float64: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderFmaFeaturesKHR<'a> {
        pub fn shader_fma_float16(mut self, shader_fma_float16: bool) -> Self {
            self.shader_fma_float16 = shader_fma_float16.into();
            self
        }

        pub fn shader_fma_float32(mut self, shader_fma_float32: bool) -> Self {
            self.shader_fma_float32 = shader_fma_float32.into();
            self
        }

        pub fn shader_fma_float64(mut self, shader_fma_float64: bool) -> Self {
            self.shader_fma_float64 = shader_fma_float64.into();
            self
        }
    }
}
