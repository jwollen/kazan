#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalFormatResolveFeaturesANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format_resolve: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExternalFormatResolveFeaturesANDROID<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID,
            p_next: core::ptr::null_mut(),
            external_format_resolve: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalFormatResolvePropertiesANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub null_color_attachment_with_external_format_resolve: Bool32,
    pub external_format_resolve_chroma_offset_x: ChromaLocation,
    pub external_format_resolve_chroma_offset_y: ChromaLocation,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExternalFormatResolvePropertiesANDROID<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID,
            p_next: core::ptr::null_mut(),
            null_color_attachment_with_external_format_resolve: Default::default(),
            external_format_resolve_chroma_offset_x: Default::default(),
            external_format_resolve_chroma_offset_y: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AndroidHardwareBufferFormatResolvePropertiesANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub color_attachment_format: Format,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AndroidHardwareBufferFormatResolvePropertiesANDROID<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID,
            p_next: core::ptr::null_mut(),
            color_attachment_format: Default::default(),
            _marker: PhantomData,
        }
    }
}
