#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTilePropertiesFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_properties: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceTilePropertiesFeaturesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM,
            p_next: core::ptr::null_mut(),
            tile_properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TilePropertiesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_size: Extent3D,
    pub apron_size: Extent2D,
    pub origin: Offset2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TilePropertiesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TILE_PROPERTIES_QCOM,
            p_next: core::ptr::null_mut(),
            tile_size: Default::default(),
            apron_size: Default::default(),
            origin: Default::default(),
            _marker: PhantomData,
        }
    }
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
