//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_shader_atomic_float2.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_shader_atomic_float2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_buffer_float16_atomics: Bool32,
        pub shader_buffer_float16_atomic_add: Bool32,
        pub shader_buffer_float16_atomic_min_max: Bool32,
        pub shader_buffer_float32_atomic_min_max: Bool32,
        pub shader_buffer_float64_atomic_min_max: Bool32,
        pub shader_shared_float16_atomics: Bool32,
        pub shader_shared_float16_atomic_add: Bool32,
        pub shader_shared_float16_atomic_min_max: Bool32,
        pub shader_shared_float32_atomic_min_max: Bool32,
        pub shader_shared_float64_atomic_min_max: Bool32,
        pub shader_image_float32_atomic_min_max: Bool32,
        pub sparse_image_float32_atomic_min_max: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderAtomicFloat2FeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shader_buffer_float16_atomics",
                    &self.shader_buffer_float16_atomics,
                )
                .field(
                    "shader_buffer_float16_atomic_add",
                    &self.shader_buffer_float16_atomic_add,
                )
                .field(
                    "shader_buffer_float16_atomic_min_max",
                    &self.shader_buffer_float16_atomic_min_max,
                )
                .field(
                    "shader_buffer_float32_atomic_min_max",
                    &self.shader_buffer_float32_atomic_min_max,
                )
                .field(
                    "shader_buffer_float64_atomic_min_max",
                    &self.shader_buffer_float64_atomic_min_max,
                )
                .field(
                    "shader_shared_float16_atomics",
                    &self.shader_shared_float16_atomics,
                )
                .field(
                    "shader_shared_float16_atomic_add",
                    &self.shader_shared_float16_atomic_add,
                )
                .field(
                    "shader_shared_float16_atomic_min_max",
                    &self.shader_shared_float16_atomic_min_max,
                )
                .field(
                    "shader_shared_float32_atomic_min_max",
                    &self.shader_shared_float32_atomic_min_max,
                )
                .field(
                    "shader_shared_float64_atomic_min_max",
                    &self.shader_shared_float64_atomic_min_max,
                )
                .field(
                    "shader_image_float32_atomic_min_max",
                    &self.shader_image_float32_atomic_min_max,
                )
                .field(
                    "sparse_image_float32_atomic_min_max",
                    &self.sparse_image_float32_atomic_min_max,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'a> {}

    impl Default for PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_buffer_float16_atomics: Default::default(),
                shader_buffer_float16_atomic_add: Default::default(),
                shader_buffer_float16_atomic_min_max: Default::default(),
                shader_buffer_float32_atomic_min_max: Default::default(),
                shader_buffer_float64_atomic_min_max: Default::default(),
                shader_shared_float16_atomics: Default::default(),
                shader_shared_float16_atomic_add: Default::default(),
                shader_shared_float16_atomic_min_max: Default::default(),
                shader_shared_float32_atomic_min_max: Default::default(),
                shader_shared_float64_atomic_min_max: Default::default(),
                shader_image_float32_atomic_min_max: Default::default(),
                sparse_image_float32_atomic_min_max: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'a> {
        #[inline]
        pub fn shader_buffer_float16_atomics(
            mut self,
            shader_buffer_float16_atomics: bool,
        ) -> Self {
            self.shader_buffer_float16_atomics = shader_buffer_float16_atomics.into();
            self
        }

        #[inline]
        pub fn shader_buffer_float16_atomic_add(
            mut self,
            shader_buffer_float16_atomic_add: bool,
        ) -> Self {
            self.shader_buffer_float16_atomic_add = shader_buffer_float16_atomic_add.into();
            self
        }

        #[inline]
        pub fn shader_buffer_float16_atomic_min_max(
            mut self,
            shader_buffer_float16_atomic_min_max: bool,
        ) -> Self {
            self.shader_buffer_float16_atomic_min_max = shader_buffer_float16_atomic_min_max.into();
            self
        }

        #[inline]
        pub fn shader_buffer_float32_atomic_min_max(
            mut self,
            shader_buffer_float32_atomic_min_max: bool,
        ) -> Self {
            self.shader_buffer_float32_atomic_min_max = shader_buffer_float32_atomic_min_max.into();
            self
        }

        #[inline]
        pub fn shader_buffer_float64_atomic_min_max(
            mut self,
            shader_buffer_float64_atomic_min_max: bool,
        ) -> Self {
            self.shader_buffer_float64_atomic_min_max = shader_buffer_float64_atomic_min_max.into();
            self
        }

        #[inline]
        pub fn shader_shared_float16_atomics(
            mut self,
            shader_shared_float16_atomics: bool,
        ) -> Self {
            self.shader_shared_float16_atomics = shader_shared_float16_atomics.into();
            self
        }

        #[inline]
        pub fn shader_shared_float16_atomic_add(
            mut self,
            shader_shared_float16_atomic_add: bool,
        ) -> Self {
            self.shader_shared_float16_atomic_add = shader_shared_float16_atomic_add.into();
            self
        }

        #[inline]
        pub fn shader_shared_float16_atomic_min_max(
            mut self,
            shader_shared_float16_atomic_min_max: bool,
        ) -> Self {
            self.shader_shared_float16_atomic_min_max = shader_shared_float16_atomic_min_max.into();
            self
        }

        #[inline]
        pub fn shader_shared_float32_atomic_min_max(
            mut self,
            shader_shared_float32_atomic_min_max: bool,
        ) -> Self {
            self.shader_shared_float32_atomic_min_max = shader_shared_float32_atomic_min_max.into();
            self
        }

        #[inline]
        pub fn shader_shared_float64_atomic_min_max(
            mut self,
            shader_shared_float64_atomic_min_max: bool,
        ) -> Self {
            self.shader_shared_float64_atomic_min_max = shader_shared_float64_atomic_min_max.into();
            self
        }

        #[inline]
        pub fn shader_image_float32_atomic_min_max(
            mut self,
            shader_image_float32_atomic_min_max: bool,
        ) -> Self {
            self.shader_image_float32_atomic_min_max = shader_image_float32_atomic_min_max.into();
            self
        }

        #[inline]
        pub fn sparse_image_float32_atomic_min_max(
            mut self,
            sparse_image_float32_atomic_min_max: bool,
        ) -> Self {
            self.sparse_image_float32_atomic_min_max = sparse_image_float32_atomic_min_max.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT =
        PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'static>;
    impl PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
