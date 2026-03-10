//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_shader_float8.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_shader_float8";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderFloat8FeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderFloat8FeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_float8: Bool32,
        pub shader_float8_cooperative_matrix: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderFloat8FeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderFloat8FeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_float8", &self.shader_float8)
                .field(
                    "shader_float8_cooperative_matrix",
                    &self.shader_float8_cooperative_matrix,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderFloat8FeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceShaderFloat8FeaturesEXT<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceShaderFloat8FeaturesEXT<'_> {}

    impl Default for PhysicalDeviceShaderFloat8FeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_float8: Default::default(),
                shader_float8_cooperative_matrix: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderFloat8FeaturesEXT<'a> {
        #[inline]
        pub fn shader_float8(mut self, shader_float8: bool) -> Self {
            self.shader_float8 = shader_float8.into();
            self
        }

        #[inline]
        pub fn shader_float8_cooperative_matrix(
            mut self,
            shader_float8_cooperative_matrix: bool,
        ) -> Self {
            self.shader_float8_cooperative_matrix = shader_float8_cooperative_matrix.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceShaderFloat8FeaturesEXT =
        PhysicalDeviceShaderFloat8FeaturesEXT<'static>;
    impl PhysicalDeviceShaderFloat8FeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceShaderFloat8FeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
