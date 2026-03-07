#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_shader_atomic_float";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderAtomicFloatFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_buffer_float32_atomics: Bool32,
        pub shader_buffer_float32_atomic_add: Bool32,
        pub shader_buffer_float64_atomics: Bool32,
        pub shader_buffer_float64_atomic_add: Bool32,
        pub shader_shared_float32_atomics: Bool32,
        pub shader_shared_float32_atomic_add: Bool32,
        pub shader_shared_float64_atomics: Bool32,
        pub shader_shared_float64_atomic_add: Bool32,
        pub shader_image_float32_atomics: Bool32,
        pub shader_image_float32_atomic_add: Bool32,
        pub sparse_image_float32_atomics: Bool32,
        pub sparse_image_float32_atomic_add: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderAtomicFloatFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderAtomicFloatFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shader_buffer_float32_atomics",
                    &self.shader_buffer_float32_atomics,
                )
                .field(
                    "shader_buffer_float32_atomic_add",
                    &self.shader_buffer_float32_atomic_add,
                )
                .field(
                    "shader_buffer_float64_atomics",
                    &self.shader_buffer_float64_atomics,
                )
                .field(
                    "shader_buffer_float64_atomic_add",
                    &self.shader_buffer_float64_atomic_add,
                )
                .field(
                    "shader_shared_float32_atomics",
                    &self.shader_shared_float32_atomics,
                )
                .field(
                    "shader_shared_float32_atomic_add",
                    &self.shader_shared_float32_atomic_add,
                )
                .field(
                    "shader_shared_float64_atomics",
                    &self.shader_shared_float64_atomics,
                )
                .field(
                    "shader_shared_float64_atomic_add",
                    &self.shader_shared_float64_atomic_add,
                )
                .field(
                    "shader_image_float32_atomics",
                    &self.shader_image_float32_atomics,
                )
                .field(
                    "shader_image_float32_atomic_add",
                    &self.shader_image_float32_atomic_add,
                )
                .field(
                    "sparse_image_float32_atomics",
                    &self.sparse_image_float32_atomics,
                )
                .field(
                    "sparse_image_float32_atomic_add",
                    &self.sparse_image_float32_atomic_add,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderAtomicFloatFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderAtomicFloatFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderAtomicFloatFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceShaderAtomicFloatFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_buffer_float32_atomics: Default::default(),
                shader_buffer_float32_atomic_add: Default::default(),
                shader_buffer_float64_atomics: Default::default(),
                shader_buffer_float64_atomic_add: Default::default(),
                shader_shared_float32_atomics: Default::default(),
                shader_shared_float32_atomic_add: Default::default(),
                shader_shared_float64_atomics: Default::default(),
                shader_shared_float64_atomic_add: Default::default(),
                shader_image_float32_atomics: Default::default(),
                shader_image_float32_atomic_add: Default::default(),
                sparse_image_float32_atomics: Default::default(),
                sparse_image_float32_atomic_add: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderAtomicFloatFeaturesEXT<'a> {
        pub fn shader_buffer_float32_atomics(
            mut self,
            shader_buffer_float32_atomics: bool,
        ) -> Self {
            self.shader_buffer_float32_atomics = shader_buffer_float32_atomics.into();
            self
        }

        pub fn shader_buffer_float32_atomic_add(
            mut self,
            shader_buffer_float32_atomic_add: bool,
        ) -> Self {
            self.shader_buffer_float32_atomic_add = shader_buffer_float32_atomic_add.into();
            self
        }

        pub fn shader_buffer_float64_atomics(
            mut self,
            shader_buffer_float64_atomics: bool,
        ) -> Self {
            self.shader_buffer_float64_atomics = shader_buffer_float64_atomics.into();
            self
        }

        pub fn shader_buffer_float64_atomic_add(
            mut self,
            shader_buffer_float64_atomic_add: bool,
        ) -> Self {
            self.shader_buffer_float64_atomic_add = shader_buffer_float64_atomic_add.into();
            self
        }

        pub fn shader_shared_float32_atomics(
            mut self,
            shader_shared_float32_atomics: bool,
        ) -> Self {
            self.shader_shared_float32_atomics = shader_shared_float32_atomics.into();
            self
        }

        pub fn shader_shared_float32_atomic_add(
            mut self,
            shader_shared_float32_atomic_add: bool,
        ) -> Self {
            self.shader_shared_float32_atomic_add = shader_shared_float32_atomic_add.into();
            self
        }

        pub fn shader_shared_float64_atomics(
            mut self,
            shader_shared_float64_atomics: bool,
        ) -> Self {
            self.shader_shared_float64_atomics = shader_shared_float64_atomics.into();
            self
        }

        pub fn shader_shared_float64_atomic_add(
            mut self,
            shader_shared_float64_atomic_add: bool,
        ) -> Self {
            self.shader_shared_float64_atomic_add = shader_shared_float64_atomic_add.into();
            self
        }

        pub fn shader_image_float32_atomics(mut self, shader_image_float32_atomics: bool) -> Self {
            self.shader_image_float32_atomics = shader_image_float32_atomics.into();
            self
        }

        pub fn shader_image_float32_atomic_add(
            mut self,
            shader_image_float32_atomic_add: bool,
        ) -> Self {
            self.shader_image_float32_atomic_add = shader_image_float32_atomic_add.into();
            self
        }

        pub fn sparse_image_float32_atomics(mut self, sparse_image_float32_atomics: bool) -> Self {
            self.sparse_image_float32_atomics = sparse_image_float32_atomics.into();
            self
        }

        pub fn sparse_image_float32_atomic_add(
            mut self,
            sparse_image_float32_atomic_add: bool,
        ) -> Self {
            self.sparse_image_float32_atomic_add = sparse_image_float32_atomic_add.into();
            self
        }
    }
}
