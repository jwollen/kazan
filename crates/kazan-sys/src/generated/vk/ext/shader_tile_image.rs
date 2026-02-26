#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderTileImageFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_tile_image_color_read_access: Bool32,
    pub shader_tile_image_depth_read_access: Bool32,
    pub shader_tile_image_stencil_read_access: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderTileImageFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            shader_tile_image_color_read_access: Default::default(),
            shader_tile_image_depth_read_access: Default::default(),
            shader_tile_image_stencil_read_access: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderTileImagePropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_tile_image_coherent_read_accelerated: Bool32,
    pub shader_tile_image_read_sample_from_pixel_rate_invocation: Bool32,
    pub shader_tile_image_read_from_helper_invocation: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderTileImagePropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            shader_tile_image_coherent_read_accelerated: Default::default(),
            shader_tile_image_read_sample_from_pixel_rate_invocation: Default::default(),
            shader_tile_image_read_from_helper_invocation: Default::default(),
            _marker: PhantomData,
        }
    }
}
