//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_shader_ocp_microscaling_types.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_shader_ocp_microscaling_types";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_float4: Bool32,
        pub shader_float6: Bool32,
        pub shader_float8_unsigned_e8m0: Bool32,
        pub shader_mx_int8: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_float4", &self.shader_float4)
                .field("shader_float6", &self.shader_float6)
                .field(
                    "shader_float8_unsigned_e8m0",
                    &self.shader_float8_unsigned_e8m0,
                )
                .field("shader_mx_int8", &self.shader_mx_int8)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_OCP_MICROSCALING_TYPES_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'_>
    {
    }

    impl Default for PhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_float4: Default::default(),
                shader_float6: Default::default(),
                shader_float8_unsigned_e8m0: Default::default(),
                shader_mx_int8: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'a> {
        #[inline]
        pub fn shader_float4(mut self, shader_float4: bool) -> Self {
            self.shader_float4 = shader_float4.into();
            self
        }

        #[inline]
        pub fn shader_float6(mut self, shader_float6: bool) -> Self {
            self.shader_float6 = shader_float6.into();
            self
        }

        #[inline]
        pub fn shader_float8_unsigned_e8m0(mut self, shader_float8_unsigned_e8m0: bool) -> Self {
            self.shader_float8_unsigned_e8m0 = shader_float8_unsigned_e8m0.into();
            self
        }

        #[inline]
        pub fn shader_mx_int8(mut self, shader_mx_int8: bool) -> Self {
            self.shader_mx_int8 = shader_mx_int8.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT =
        PhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'static>;
    impl PhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
