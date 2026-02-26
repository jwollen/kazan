#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct ImportScreenBufferInfoQNX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *mut _screen_buffer,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ScreenBufferPropertiesQNX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ScreenBufferFormatPropertiesQNX<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ExternalFormatQNX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format: u64,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub screen_buffer_import: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkGetScreenBufferPropertiesQNX = unsafe extern "system" fn(
    device: Device,
    buffer: *const _screen_buffer,
    p_properties: *mut ScreenBufferPropertiesQNX<'_>,
) -> Result;
