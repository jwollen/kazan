#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportScreenBufferInfoQNX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *mut _screen_buffer,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportScreenBufferInfoQNX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_SCREEN_BUFFER_INFO_QNX,
            p_next: core::ptr::null(),
            buffer: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScreenBufferPropertiesQNX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ScreenBufferPropertiesQNX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SCREEN_BUFFER_PROPERTIES_QNX,
            p_next: core::ptr::null_mut(),
            allocation_size: Default::default(),
            memory_type_bits: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for ScreenBufferFormatPropertiesQNX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SCREEN_BUFFER_FORMAT_PROPERTIES_QNX,
            p_next: core::ptr::null_mut(),
            format: Default::default(),
            external_format: Default::default(),
            screen_usage: Default::default(),
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
pub struct ExternalFormatQNX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalFormatQNX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_FORMAT_QNX,
            p_next: core::ptr::null_mut(),
            external_format: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub screen_buffer_import: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX,
            p_next: core::ptr::null_mut(),
            screen_buffer_import: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetScreenBufferPropertiesQNX = unsafe extern "system" fn(
    device: Device,
    buffer: *const _screen_buffer,
    p_properties: *mut ScreenBufferPropertiesQNX<'_>,
) -> Result;
