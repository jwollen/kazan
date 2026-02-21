#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map: Bool32,
    pub fragment_density_map_dynamic: Bool32,
    pub fragment_density_map_non_subsampled_images: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_fragment_density_texel_size: Extent2D,
    pub max_fragment_density_texel_size: Extent2D,
    pub fragment_density_invocations: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassFragmentDensityMapCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_density_map_attachment: AttachmentReference,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingFragmentDensityMapAttachmentInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
}
