//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_shader_image_atomic_int64.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_shader_image_atomic_int64";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_image_int64_atomics: Bool32,
        pub sparse_image_int64_atomics: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderImageAtomicInt64FeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shader_image_int64_atomics",
                    &self.shader_image_int64_atomics,
                )
                .field(
                    "sparse_image_int64_atomics",
                    &self.sparse_image_int64_atomics,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_image_int64_atomics: Default::default(),
                sparse_image_int64_atomics: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'a> {
        #[inline]
        pub fn shader_image_int64_atomics(mut self, shader_image_int64_atomics: bool) -> Self {
            self.shader_image_int64_atomics = shader_image_int64_atomics.into();
            self
        }

        #[inline]
        pub fn sparse_image_int64_atomics(mut self, sparse_image_int64_atomics: bool) -> Self {
            self.sparse_image_int64_atomics = sparse_image_int64_atomics.into();
            self
        }
    }
}
