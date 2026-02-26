#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrmFormatModifierPropertiesListEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifier_properties: *mut DrmFormatModifierPropertiesEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DrmFormatModifierPropertiesListEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT,
            p_next: core::ptr::null_mut(),
            drm_format_modifier_count: Default::default(),
            p_drm_format_modifier_properties: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DrmFormatModifierPropertiesEXT {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: FormatFeatureFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier: u64,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceImageDrmFormatModifierInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT,
            p_next: core::ptr::null(),
            drm_format_modifier: Default::default(),
            sharing_mode: Default::default(),
            queue_family_index_count: Default::default(),
            p_queue_family_indices: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageDrmFormatModifierListCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifiers: *const u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageDrmFormatModifierListCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            drm_format_modifier_count: Default::default(),
            p_drm_format_modifiers: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub p_plane_layouts: *const SubresourceLayout,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageDrmFormatModifierExplicitCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            drm_format_modifier: Default::default(),
            drm_format_modifier_plane_count: Default::default(),
            p_plane_layouts: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageDrmFormatModifierPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub drm_format_modifier: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageDrmFormatModifierPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            drm_format_modifier: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrmFormatModifierPropertiesList2EXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub drm_format_modifier_count: u32,
    pub p_drm_format_modifier_properties: *mut DrmFormatModifierProperties2EXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DrmFormatModifierPropertiesList2EXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT,
            p_next: core::ptr::null_mut(),
            drm_format_modifier_count: Default::default(),
            p_drm_format_modifier_properties: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
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
