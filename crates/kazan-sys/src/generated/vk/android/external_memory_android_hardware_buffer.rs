#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type AHardwareBuffer = *const c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportAndroidHardwareBufferInfoANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *mut AHardwareBuffer,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportAndroidHardwareBufferInfoANDROID<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
            p_next: core::ptr::null(),
            buffer: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AndroidHardwareBufferUsageANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub android_hardware_buffer_usage: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AndroidHardwareBufferUsageANDROID<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID,
            p_next: core::ptr::null_mut(),
            android_hardware_buffer_usage: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AndroidHardwareBufferPropertiesANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AndroidHardwareBufferPropertiesANDROID<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID,
            p_next: core::ptr::null_mut(),
            allocation_size: Default::default(),
            memory_type_bits: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryGetAndroidHardwareBufferInfoANDROID<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
            p_next: core::ptr::null(),
            memory: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for AndroidHardwareBufferFormatPropertiesANDROID<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID,
            p_next: core::ptr::null_mut(),
            format: Default::default(),
            external_format: Default::default(),
            format_features: Default::default(),
            sampler_ycbcr_conversion_components: Default::default(),
            suggested_ycbcr_model: Default::default(),
            suggested_ycbcr_range: Default::default(),
            suggested_x_chroma_offset: Default::default(),
            suggested_y_chroma_offset: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalFormatANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalFormatANDROID<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_FORMAT_ANDROID,
            p_next: core::ptr::null_mut(),
            external_format: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for AndroidHardwareBufferFormatProperties2ANDROID<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID,
            p_next: core::ptr::null_mut(),
            format: Default::default(),
            external_format: Default::default(),
            format_features: Default::default(),
            sampler_ycbcr_conversion_components: Default::default(),
            suggested_ycbcr_model: Default::default(),
            suggested_ycbcr_range: Default::default(),
            suggested_x_chroma_offset: Default::default(),
            suggested_y_chroma_offset: Default::default(),
            _marker: PhantomData,
        }
    }
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
