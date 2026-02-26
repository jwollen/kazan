#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct SampleLocationEXT {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
pub struct SampleLocationsInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_per_pixel: SampleCountFlagBits,
    pub sample_location_grid_size: Extent2D,
    pub sample_locations_count: u32,
    pub p_sample_locations: *const SampleLocationEXT,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct AttachmentSampleLocationsEXT<'a> {
    pub attachment_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SubpassSampleLocationsEXT<'a> {
    pub subpass_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct RenderPassSampleLocationsBeginInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_initial_sample_locations_count: u32,
    pub p_attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT<'a>,
    pub post_subpass_sample_locations_count: u32,
    pub p_post_subpass_sample_locations: *const SubpassSampleLocationsEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineSampleLocationsStateCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_enable: Bool32,
    pub sample_locations_info: SampleLocationsInfoEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sample_location_sample_counts: SampleCountFlags,
    pub max_sample_location_grid_size: Extent2D,
    pub sample_location_coordinate_range: [f32; 2],
    pub sample_location_sub_pixel_bits: u32,
    pub variable_sample_locations: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MultisamplePropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_sample_location_grid_size: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_sample_locations_info: *const SampleLocationsInfoEXT<'_>,
);
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    samples: SampleCountFlagBits,
    p_multisample_properties: *mut MultisamplePropertiesEXT<'_>,
);
