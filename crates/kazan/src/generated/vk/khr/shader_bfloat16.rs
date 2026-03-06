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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderBfloat16FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_b_float16_type: Bool32,
        pub shader_b_float16_dot_product: Bool32,
        pub shader_b_float16_cooperative_matrix: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderBfloat16FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderBfloat16FeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderBfloat16FeaturesKHR<'a> {}
    impl Default for PhysicalDeviceShaderBfloat16FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_b_float16_type: Default::default(),
                shader_b_float16_dot_product: Default::default(),
                shader_b_float16_cooperative_matrix: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderBfloat16FeaturesKHR<'a> {
        pub fn shader_b_float16_type(mut self, shader_b_float16_type: bool) -> Self {
            self.shader_b_float16_type = shader_b_float16_type.into();
            self
        }
        pub fn shader_b_float16_dot_product(mut self, shader_b_float16_dot_product: bool) -> Self {
            self.shader_b_float16_dot_product = shader_b_float16_dot_product.into();
            self
        }
        pub fn shader_b_float16_cooperative_matrix(
            mut self,
            shader_b_float16_cooperative_matrix: bool,
        ) -> Self {
            self.shader_b_float16_cooperative_matrix = shader_b_float16_cooperative_matrix.into();
            self
        }
    }
}
