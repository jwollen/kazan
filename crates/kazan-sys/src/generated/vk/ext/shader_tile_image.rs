#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceShaderTileImageFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_tile_image_color_read_access: Bool32,
    pub shader_tile_image_depth_read_access: Bool32,
    pub shader_tile_image_stencil_read_access: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceShaderTileImagePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_tile_image_coherent_read_accelerated: Bool32,
    pub shader_tile_image_read_sample_from_pixel_rate_invocation: Bool32,
    pub shader_tile_image_read_from_helper_invocation: Bool32,
}
