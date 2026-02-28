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
    pub struct PhysicalDeviceShaderFmaFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_fma_float16: Bool32,
        pub shader_fma_float32: Bool32,
        pub shader_fma_float64: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceShaderFmaFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR,
                p_next: core::ptr::null_mut(),
                shader_fma_float16: Default::default(),
                shader_fma_float32: Default::default(),
                shader_fma_float64: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderFmaFeaturesKHR<'a> {
        pub fn shader_fma_float16(mut self, shader_fma_float16: Bool32) -> Self {
            self.shader_fma_float16 = shader_fma_float16;
            self
        }
        pub fn shader_fma_float32(mut self, shader_fma_float32: Bool32) -> Self {
            self.shader_fma_float32 = shader_fma_float32;
            self
        }
        pub fn shader_fma_float64(mut self, shader_fma_float64: Bool32) -> Self {
            self.shader_fma_float64 = shader_fma_float64;
            self
        }
    }
}
