#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SampleLocationEXT {
    pub x: f32,
    pub y: f32,
}
impl SampleLocationEXT {
    pub fn x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }
    pub fn y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }
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
impl<'a> SampleLocationsInfoEXT<'a> {
    pub fn sample_locations_per_pixel(
        mut self,
        sample_locations_per_pixel: SampleCountFlagBits,
    ) -> Self {
        self.sample_locations_per_pixel = sample_locations_per_pixel;
        self
    }
    pub fn sample_location_grid_size(mut self, sample_location_grid_size: Extent2D) -> Self {
        self.sample_location_grid_size = sample_location_grid_size;
        self
    }
    pub fn sample_locations(mut self, sample_locations: &'a [SampleLocationEXT]) -> Self {
        self.sample_locations_count = sample_locations.len().try_into().unwrap();
        self.p_sample_locations = sample_locations.as_ptr();
        self
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
impl<'a> AttachmentSampleLocationsEXT<'a> {
    pub fn attachment_index(mut self, attachment_index: u32) -> Self {
        self.attachment_index = attachment_index;
        self
    }
    pub fn sample_locations_info(
        mut self,
        sample_locations_info: SampleLocationsInfoEXT<'a>,
    ) -> Self {
        self.sample_locations_info = sample_locations_info;
        self
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
impl<'a> SubpassSampleLocationsEXT<'a> {
    pub fn subpass_index(mut self, subpass_index: u32) -> Self {
        self.subpass_index = subpass_index;
        self
    }
    pub fn sample_locations_info(
        mut self,
        sample_locations_info: SampleLocationsInfoEXT<'a>,
    ) -> Self {
        self.sample_locations_info = sample_locations_info;
        self
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
impl<'a> RenderPassSampleLocationsBeginInfoEXT<'a> {
    pub fn attachment_initial_sample_locations(
        mut self,
        attachment_initial_sample_locations: &'a [AttachmentSampleLocationsEXT<'a>],
    ) -> Self {
        self.attachment_initial_sample_locations_count = attachment_initial_sample_locations
            .len()
            .try_into()
            .unwrap();
        self.p_attachment_initial_sample_locations = attachment_initial_sample_locations.as_ptr();
        self
    }
    pub fn post_subpass_sample_locations(
        mut self,
        post_subpass_sample_locations: &'a [SubpassSampleLocationsEXT<'a>],
    ) -> Self {
        self.post_subpass_sample_locations_count =
            post_subpass_sample_locations.len().try_into().unwrap();
        self.p_post_subpass_sample_locations = post_subpass_sample_locations.as_ptr();
        self
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
impl<'a> PipelineSampleLocationsStateCreateInfoEXT<'a> {
    pub fn sample_locations_enable(mut self, sample_locations_enable: Bool32) -> Self {
        self.sample_locations_enable = sample_locations_enable;
        self
    }
    pub fn sample_locations_info(
        mut self,
        sample_locations_info: SampleLocationsInfoEXT<'a>,
    ) -> Self {
        self.sample_locations_info = sample_locations_info;
        self
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
impl<'a> PhysicalDeviceSampleLocationsPropertiesEXT<'a> {
    pub fn sample_location_sample_counts(
        mut self,
        sample_location_sample_counts: SampleCountFlags,
    ) -> Self {
        self.sample_location_sample_counts = sample_location_sample_counts;
        self
    }
    pub fn max_sample_location_grid_size(
        mut self,
        max_sample_location_grid_size: Extent2D,
    ) -> Self {
        self.max_sample_location_grid_size = max_sample_location_grid_size;
        self
    }
    pub fn sample_location_coordinate_range(
        mut self,
        sample_location_coordinate_range: [f32; 2],
    ) -> Self {
        self.sample_location_coordinate_range = sample_location_coordinate_range;
        self
    }
    pub fn sample_location_sub_pixel_bits(mut self, sample_location_sub_pixel_bits: u32) -> Self {
        self.sample_location_sub_pixel_bits = sample_location_sub_pixel_bits;
        self
    }
    pub fn variable_sample_locations(mut self, variable_sample_locations: Bool32) -> Self {
        self.variable_sample_locations = variable_sample_locations;
        self
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
impl<'a> MultisamplePropertiesEXT<'a> {
    pub fn max_sample_location_grid_size(
        mut self,
        max_sample_location_grid_size: Extent2D,
    ) -> Self {
        self.max_sample_location_grid_size = max_sample_location_grid_size;
        self
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
