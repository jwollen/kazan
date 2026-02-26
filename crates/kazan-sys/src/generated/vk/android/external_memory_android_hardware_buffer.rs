#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type AHardwareBuffer = *const c_void;
#[repr(C)]
pub struct ImportAndroidHardwareBufferInfoANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *mut AHardwareBuffer,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct AndroidHardwareBufferUsageANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub android_hardware_buffer_usage: u64,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct AndroidHardwareBufferPropertiesANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
    pub external_format: u64,
    pub format_features: FormatFeatureFlags,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_x_chroma_offset: ChromaLocation,
    pub suggested_y_chroma_offset: ChromaLocation,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ExternalFormatANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format: u64,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct AndroidHardwareBufferFormatProperties2ANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
    pub external_format: u64,
    pub format_features: FormatFeatureFlags2,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_x_chroma_offset: ChromaLocation,
    pub suggested_y_chroma_offset: ChromaLocation,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = unsafe extern "system" fn(
    device: Device,
    buffer: *const AHardwareBuffer,
    p_properties: *mut AndroidHardwareBufferPropertiesANDROID<'_>,
) -> Result;
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = unsafe extern "system" fn(
    device: Device,
    p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
    p_buffer: *mut *mut AHardwareBuffer,
) -> Result;
