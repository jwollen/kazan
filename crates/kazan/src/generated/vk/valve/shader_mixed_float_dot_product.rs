//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_VALVE_shader_mixed_float_dot_product.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_VALVE_shader_mixed_float_dot_product";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_mixed_float_dot_product_float16_acc_float32: Bool32,
        pub shader_mixed_float_dot_product_float16_acc_float16: Bool32,
        pub shader_mixed_float_dot_product_b_float16_acc: Bool32,
        pub shader_mixed_float_dot_product_float8_acc_float32: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shader_mixed_float_dot_product_float16_acc_float32",
                    &self.shader_mixed_float_dot_product_float16_acc_float32,
                )
                .field(
                    "shader_mixed_float_dot_product_float16_acc_float16",
                    &self.shader_mixed_float_dot_product_float16_acc_float16,
                )
                .field(
                    "shader_mixed_float_dot_product_b_float16_acc",
                    &self.shader_mixed_float_dot_product_b_float16_acc,
                )
                .field(
                    "shader_mixed_float_dot_product_float8_acc_float32",
                    &self.shader_mixed_float_dot_product_float8_acc_float32,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_MIXED_FLOAT_DOT_PRODUCT_FEATURES_VALVE;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'_>
    {
    }

    impl Default for PhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_mixed_float_dot_product_float16_acc_float32: Default::default(),
                shader_mixed_float_dot_product_float16_acc_float16: Default::default(),
                shader_mixed_float_dot_product_b_float16_acc: Default::default(),
                shader_mixed_float_dot_product_float8_acc_float32: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'a> {
        #[inline]
        pub fn shader_mixed_float_dot_product_float16_acc_float32(
            mut self,
            shader_mixed_float_dot_product_float16_acc_float32: bool,
        ) -> Self {
            self.shader_mixed_float_dot_product_float16_acc_float32 =
                shader_mixed_float_dot_product_float16_acc_float32.into();
            self
        }

        #[inline]
        pub fn shader_mixed_float_dot_product_float16_acc_float16(
            mut self,
            shader_mixed_float_dot_product_float16_acc_float16: bool,
        ) -> Self {
            self.shader_mixed_float_dot_product_float16_acc_float16 =
                shader_mixed_float_dot_product_float16_acc_float16.into();
            self
        }

        #[inline]
        pub fn shader_mixed_float_dot_product_b_float16_acc(
            mut self,
            shader_mixed_float_dot_product_b_float16_acc: bool,
        ) -> Self {
            self.shader_mixed_float_dot_product_b_float16_acc =
                shader_mixed_float_dot_product_b_float16_acc.into();
            self
        }

        #[inline]
        pub fn shader_mixed_float_dot_product_float8_acc_float32(
            mut self,
            shader_mixed_float_dot_product_float8_acc_float32: bool,
        ) -> Self {
            self.shader_mixed_float_dot_product_float8_acc_float32 =
                shader_mixed_float_dot_product_float8_acc_float32.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE =
        PhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'static>;
    impl PhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE {
            unsafe { core::mem::transmute(self) }
        }
    }
}
