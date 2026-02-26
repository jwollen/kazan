#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SampleLocationEXT {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SampleLocationsInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_per_pixel: SampleCountFlagBits,
    pub sample_location_grid_size: Extent2D,
    pub sample_locations_count: u32,
    pub p_sample_locations: *const SampleLocationEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SampleLocationsInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SAMPLE_LOCATIONS_INFO_EXT,
            p_next: core::ptr::null(),
            sample_locations_per_pixel: Default::default(),
            sample_location_grid_size: Default::default(),
            sample_locations_count: Default::default(),
            p_sample_locations: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentSampleLocationsEXT<'a> {
    pub attachment_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AttachmentSampleLocationsEXT<'_> {
    fn default() -> Self {
        Self {
            attachment_index: Default::default(),
            sample_locations_info: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassSampleLocationsEXT<'a> {
    pub subpass_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SubpassSampleLocationsEXT<'_> {
    fn default() -> Self {
        Self {
            subpass_index: Default::default(),
            sample_locations_info: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassSampleLocationsBeginInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_initial_sample_locations_count: u32,
    pub p_attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT<'a>,
    pub post_subpass_sample_locations_count: u32,
    pub p_post_subpass_sample_locations: *const SubpassSampleLocationsEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderPassSampleLocationsBeginInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
            p_next: core::ptr::null(),
            attachment_initial_sample_locations_count: Default::default(),
            p_attachment_initial_sample_locations: core::ptr::null(),
            post_subpass_sample_locations_count: Default::default(),
            p_post_subpass_sample_locations: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineSampleLocationsStateCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_enable: Bool32,
    pub sample_locations_info: SampleLocationsInfoEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineSampleLocationsStateCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            sample_locations_enable: Default::default(),
            sample_locations_info: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for PhysicalDeviceSampleLocationsPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            sample_location_sample_counts: Default::default(),
            max_sample_location_grid_size: Default::default(),
            sample_location_coordinate_range: [Default::default(); _],
            sample_location_sub_pixel_bits: Default::default(),
            variable_sample_locations: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MultisamplePropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_sample_location_grid_size: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MultisamplePropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MULTISAMPLE_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            max_sample_location_grid_size: Default::default(),
            _marker: PhantomData,
        }
    }
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
