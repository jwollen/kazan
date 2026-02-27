#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
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
impl<'a> DrmFormatModifierPropertiesListEXT<'a> {
    pub fn drm_format_modifier_properties(
        mut self,
        drm_format_modifier_properties: &'a mut [DrmFormatModifierPropertiesEXT],
    ) -> Self {
        self.drm_format_modifier_count = drm_format_modifier_properties.len().try_into().unwrap();
        self.p_drm_format_modifier_properties = drm_format_modifier_properties.as_mut_ptr();
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DrmFormatModifierPropertiesEXT {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: FormatFeatureFlags,
}
impl DrmFormatModifierPropertiesEXT {
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.drm_format_modifier = drm_format_modifier;
        self
    }
    pub fn drm_format_modifier_plane_count(mut self, drm_format_modifier_plane_count: u32) -> Self {
        self.drm_format_modifier_plane_count = drm_format_modifier_plane_count;
        self
    }
    pub fn drm_format_modifier_tiling_features(
        mut self,
        drm_format_modifier_tiling_features: FormatFeatureFlags,
    ) -> Self {
        self.drm_format_modifier_tiling_features = drm_format_modifier_tiling_features;
        self
    }
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
impl<'a> PhysicalDeviceImageDrmFormatModifierInfoEXT<'a> {
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.drm_format_modifier = drm_format_modifier;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: SharingMode) -> Self {
        self.sharing_mode = sharing_mode;
        self
    }
    pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
        self.queue_family_index_count = queue_family_indices.len().try_into().unwrap();
        self.p_queue_family_indices = queue_family_indices.as_ptr();
        self
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
impl<'a> ImageDrmFormatModifierListCreateInfoEXT<'a> {
    pub fn drm_format_modifiers(mut self, drm_format_modifiers: &'a [u64]) -> Self {
        self.drm_format_modifier_count = drm_format_modifiers.len().try_into().unwrap();
        self.p_drm_format_modifiers = drm_format_modifiers.as_ptr();
        self
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
impl<'a> ImageDrmFormatModifierExplicitCreateInfoEXT<'a> {
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.drm_format_modifier = drm_format_modifier;
        self
    }
    pub fn plane_layouts(mut self, plane_layouts: &'a [SubresourceLayout]) -> Self {
        self.drm_format_modifier_plane_count = plane_layouts.len().try_into().unwrap();
        self.p_plane_layouts = plane_layouts.as_ptr();
        self
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
impl<'a> ImageDrmFormatModifierPropertiesEXT<'a> {
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.drm_format_modifier = drm_format_modifier;
        self
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
impl<'a> DrmFormatModifierPropertiesList2EXT<'a> {
    pub fn drm_format_modifier_properties(
        mut self,
        drm_format_modifier_properties: &'a mut [DrmFormatModifierProperties2EXT],
    ) -> Self {
        self.drm_format_modifier_count = drm_format_modifier_properties.len().try_into().unwrap();
        self.p_drm_format_modifier_properties = drm_format_modifier_properties.as_mut_ptr();
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DrmFormatModifierProperties2EXT {
    pub drm_format_modifier: u64,
    pub drm_format_modifier_plane_count: u32,
    pub drm_format_modifier_tiling_features: FormatFeatureFlags2,
}
impl DrmFormatModifierProperties2EXT {
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.drm_format_modifier = drm_format_modifier;
        self
    }
    pub fn drm_format_modifier_plane_count(mut self, drm_format_modifier_plane_count: u32) -> Self {
        self.drm_format_modifier_plane_count = drm_format_modifier_plane_count;
        self
    }
    pub fn drm_format_modifier_tiling_features(
        mut self,
        drm_format_modifier_tiling_features: FormatFeatureFlags2,
    ) -> Self {
        self.drm_format_modifier_tiling_features = drm_format_modifier_tiling_features;
        self
    }
}
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_properties: *mut ImageDrmFormatModifierPropertiesEXT<'_>,
) -> Result;
