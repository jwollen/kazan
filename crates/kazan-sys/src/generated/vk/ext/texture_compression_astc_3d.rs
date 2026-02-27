#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub texture_compression_astc_3d: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            texture_compression_astc_3d: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'a> {
    pub fn texture_compression_astc_3d(mut self, texture_compression_astc_3d: Bool32) -> Self {
        self.texture_compression_astc_3d = texture_compression_astc_3d;
        self
    }
}
