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
    pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_image_int64_atomics: Bool32,
        pub sparse_image_int64_atomics: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
        pub fn shader_image_int64_atomics(mut self, shader_image_int64_atomics: Bool32) -> Self {
            self.shader_image_int64_atomics = shader_image_int64_atomics;
            self
        }
        pub fn sparse_image_int64_atomics(mut self, sparse_image_int64_atomics: Bool32) -> Self {
            self.sparse_image_int64_atomics = sparse_image_int64_atomics;
            self
        }
    }
}
