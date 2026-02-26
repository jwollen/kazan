#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceTilePropertiesFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_properties: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct TilePropertiesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_size: Extent3D,
    pub apron_size: Extent2D,
    pub origin: Offset2D,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkGetFramebufferTilePropertiesQCOM = unsafe extern "system" fn(
    device: Device,
    framebuffer: Framebuffer,
    p_properties_count: *mut u32,
    p_properties: *mut TilePropertiesQCOM<'_>,
) -> Result;
pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = unsafe extern "system" fn(
    device: Device,
    p_rendering_info: *const RenderingInfo<'_>,
    p_properties: *mut TilePropertiesQCOM<'_>,
) -> Result;
