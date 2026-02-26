#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct DrmFormatModifierPropertiesListEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifier_properties: *mut DrmFormatModifierPropertiesEXT,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DrmFormatModifierPropertiesEXT {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: FormatFeatureFlags,
}
#[repr(C)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier: u64,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImageDrmFormatModifierListCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifiers: *const u64,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub p_plane_layouts: *const SubresourceLayout,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImageDrmFormatModifierPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub drm_format_modifier: u64,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DrmFormatModifierPropertiesList2EXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifier_properties: *mut DrmFormatModifierProperties2EXT,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DrmFormatModifierProperties2EXT {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: FormatFeatureFlags2,
}
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_properties: *mut ImageDrmFormatModifierPropertiesEXT<'_>,
) -> Result;
