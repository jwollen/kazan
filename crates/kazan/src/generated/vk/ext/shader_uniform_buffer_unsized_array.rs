//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_shader_uniform_buffer_unsized_array.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_shader_uniform_buffer_unsized_array";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_uniform_buffer_unsized_array: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shader_uniform_buffer_unsized_array",
                    &self.shader_uniform_buffer_unsized_array,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_uniform_buffer_unsized_array: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'a> {
        #[inline]
        pub fn shader_uniform_buffer_unsized_array(
            mut self,
            shader_uniform_buffer_unsized_array: bool,
        ) -> Self {
            self.shader_uniform_buffer_unsized_array = shader_uniform_buffer_unsized_array.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT =
        PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'static>;
    impl PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
