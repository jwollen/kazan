#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SampleLocationEXT {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SampleLocationsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_per_pixel: SampleCountFlagBits,
    pub sample_location_grid_size: Extent2D,
    pub sample_locations_count: u32,
    pub p_sample_locations: *const SampleLocationEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentSampleLocationsEXT {
    pub attachment_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassSampleLocationsEXT {
    pub subpass_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassSampleLocationsBeginInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_initial_sample_locations_count: u32,
    pub p_attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT,
    pub post_subpass_sample_locations_count: u32,
    pub p_post_subpass_sample_locations: *const SubpassSampleLocationsEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineSampleLocationsStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_enable: Bool32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sample_location_sample_counts: SampleCountFlags,
    pub max_sample_location_grid_size: Extent2D,
    pub sample_location_coordinate_range: [f32; 2],
    pub sample_location_sub_pixel_bits: u32,
    pub variable_sample_locations: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MultisamplePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_sample_location_grid_size: Extent2D,
}
pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_sample_locations_info: *const SampleLocationsInfoEXT,
);
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    samples: SampleCountFlagBits,
    p_multisample_properties: *mut MultisamplePropertiesEXT,
);
