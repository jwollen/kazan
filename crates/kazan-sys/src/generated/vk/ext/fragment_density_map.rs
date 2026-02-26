#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map: Bool32,
    pub fragment_density_map_dynamic: Bool32,
    pub fragment_density_map_non_subsampled_images: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_fragment_density_texel_size: Extent2D,
    pub max_fragment_density_texel_size: Extent2D,
    pub fragment_density_invocations: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct RenderPassFragmentDensityMapCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_density_map_attachment: AttachmentReference,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct RenderingFragmentDensityMapAttachmentInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub _marker: PhantomData<&'a ()>,
}
