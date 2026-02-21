#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTilePropertiesFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_properties: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TilePropertiesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_size: Extent3D,
    pub apron_size: Extent2D,
    pub origin: Offset2D,
}
pub type PFN_vkGetFramebufferTilePropertiesQCOM = unsafe extern "system" fn(
    device: Device,
    framebuffer: Framebuffer,
    p_properties_count: *mut u32,
    p_properties: *mut TilePropertiesQCOM,
) -> Result;
pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = unsafe extern "system" fn(
    device: Device,
    p_rendering_info: *const RenderingInfo,
    p_properties: *mut TilePropertiesQCOM,
) -> Result;
