#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type OH_NativeBuffer = *const c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeBufferUsageOHOS<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ohos_native_buffer_usage: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for NativeBufferUsageOHOS<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::NATIVE_BUFFER_USAGE_OHOS,
            p_next: core::ptr::null_mut(),
            ohos_native_buffer_usage: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeBufferPropertiesOHOS<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for NativeBufferPropertiesOHOS<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::NATIVE_BUFFER_PROPERTIES_OHOS,
            p_next: core::ptr::null_mut(),
            allocation_size: Default::default(),
            memory_type_bits: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeBufferFormatPropertiesOHOS<'a> {
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
impl Default for NativeBufferFormatPropertiesOHOS<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS,
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
pub struct ImportNativeBufferInfoOHOS<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *mut OH_NativeBuffer,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportNativeBufferInfoOHOS<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_NATIVE_BUFFER_INFO_OHOS,
            p_next: core::ptr::null(),
            buffer: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryGetNativeBufferInfoOHOS<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryGetNativeBufferInfoOHOS<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_GET_NATIVE_BUFFER_INFO_OHOS,
            p_next: core::ptr::null(),
            memory: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalFormatOHOS<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalFormatOHOS<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_FORMAT_OHOS,
            p_next: core::ptr::null_mut(),
            external_format: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetNativeBufferPropertiesOHOS = unsafe extern "system" fn(
    device: Device,
    buffer: *const OH_NativeBuffer,
    p_properties: *mut NativeBufferPropertiesOHOS<'_>,
) -> Result;
pub type PFN_vkGetMemoryNativeBufferOHOS = unsafe extern "system" fn(
    device: Device,
    p_info: *const MemoryGetNativeBufferInfoOHOS<'_>,
    p_buffer: *mut *mut OH_NativeBuffer,
) -> Result;
