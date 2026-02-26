#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct XYColorEXT {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdrMetadataEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_primary_red: XYColorEXT,
    pub display_primary_green: XYColorEXT,
    pub display_primary_blue: XYColorEXT,
    pub white_point: XYColorEXT,
    pub max_luminance: f32,
    pub min_luminance: f32,
    pub max_content_light_level: f32,
    pub max_frame_average_light_level: f32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for HdrMetadataEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::HDR_METADATA_EXT,
            p_next: core::ptr::null(),
            display_primary_red: Default::default(),
            display_primary_green: Default::default(),
            display_primary_blue: Default::default(),
            white_point: Default::default(),
            max_luminance: Default::default(),
            min_luminance: Default::default(),
            max_content_light_level: Default::default(),
            max_frame_average_light_level: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkSetHdrMetadataEXT = unsafe extern "system" fn(
    device: Device,
    swapchain_count: u32,
    p_swapchains: *const SwapchainKHR,
    p_metadata: *const HdrMetadataEXT<'_>,
);
