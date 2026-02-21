#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportScreenBufferInfoQNX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *mut _screen_buffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScreenBufferPropertiesQNX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScreenBufferFormatPropertiesQNX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
    pub external_format: u64,
    pub screen_usage: u64,
    pub format_features: FormatFeatureFlags,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_x_chroma_offset: ChromaLocation,
    pub suggested_y_chroma_offset: ChromaLocation,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalFormatQNX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub screen_buffer_import: Bool32,
}
pub type PFN_vkGetScreenBufferPropertiesQNX = unsafe extern "system" fn(
    device: Device,
    buffer: *const _screen_buffer,
    p_properties: *mut ScreenBufferPropertiesQNX,
) -> Result;
