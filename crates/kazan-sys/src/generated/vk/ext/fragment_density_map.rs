#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map: Bool32,
    pub fragment_density_map_dynamic: Bool32,
    pub fragment_density_map_non_subsampled_images: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentDensityMapFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            fragment_density_map: Default::default(),
            fragment_density_map_dynamic: Default::default(),
            fragment_density_map_non_subsampled_images: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_fragment_density_texel_size: Extent2D,
    pub max_fragment_density_texel_size: Extent2D,
    pub fragment_density_invocations: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentDensityMapPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            min_fragment_density_texel_size: Default::default(),
            max_fragment_density_texel_size: Default::default(),
            fragment_density_invocations: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassFragmentDensityMapCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_density_map_attachment: AttachmentReference,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderPassFragmentDensityMapCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            fragment_density_map_attachment: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingFragmentDensityMapAttachmentInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderingFragmentDensityMapAttachmentInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT,
            p_next: core::ptr::null(),
            image_view: Default::default(),
            image_layout: Default::default(),
            _marker: PhantomData,
        }
    }
}
