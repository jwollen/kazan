#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
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
impl Default for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            shader_image_int64_atomics: Default::default(),
            sparse_image_int64_atomics: Default::default(),
            _marker: PhantomData,
        }
    }
}
