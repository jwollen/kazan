#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type OH_NativeBuffer = *const c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeBufferUsageOHOS {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ohos_native_buffer_usage: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeBufferPropertiesOHOS {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeBufferFormatPropertiesOHOS {
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
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportNativeBufferInfoOHOS {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *mut OH_NativeBuffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryGetNativeBufferInfoOHOS {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalFormatOHOS {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format: u64,
}
pub type PFN_vkGetNativeBufferPropertiesOHOS = unsafe extern "system" fn(
    device: Device,
    buffer: *const OH_NativeBuffer,
    p_properties: *mut NativeBufferPropertiesOHOS,
) -> Result;
pub type PFN_vkGetMemoryNativeBufferOHOS = unsafe extern "system" fn(
    device: Device,
    p_info: *const MemoryGetNativeBufferInfoOHOS,
    p_buffer: *mut *mut OH_NativeBuffer,
) -> Result;
