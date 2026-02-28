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
    pub struct PhysicalDeviceShaderFloat8FeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_float8: Bool32,
        pub shader_float8_cooperative_matrix: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceShaderFloat8FeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT,
                p_next: core::ptr::null_mut(),
                shader_float8: Default::default(),
                shader_float8_cooperative_matrix: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderFloat8FeaturesEXT<'a> {
        pub fn shader_float8(mut self, shader_float8: Bool32) -> Self {
            self.shader_float8 = shader_float8;
            self
        }
        pub fn shader_float8_cooperative_matrix(
            mut self,
            shader_float8_cooperative_matrix: Bool32,
        ) -> Self {
            self.shader_float8_cooperative_matrix = shader_float8_cooperative_matrix;
            self
        }
    }
}
