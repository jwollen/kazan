#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceExternalFormatResolveFeaturesANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format_resolve: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceExternalFormatResolvePropertiesANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub null_color_attachment_with_external_format_resolve: Bool32,
    pub external_format_resolve_chroma_offset_x: ChromaLocation,
    pub external_format_resolve_chroma_offset_y: ChromaLocation,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct AndroidHardwareBufferFormatResolvePropertiesANDROID<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub color_attachment_format: Format,
    pub _marker: PhantomData<&'a ()>,
}
