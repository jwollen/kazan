#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub transform_feedback: Bool32,
    pub geometry_streams: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceTransformFeedbackFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            transform_feedback: Default::default(),
            geometry_streams: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceTransformFeedbackFeaturesEXT<'a> {
    pub fn transform_feedback(mut self, transform_feedback: Bool32) -> Self {
        self.transform_feedback = transform_feedback;
        self
    }
    pub fn geometry_streams(mut self, geometry_streams: Bool32) -> Self {
        self.geometry_streams = geometry_streams;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTransformFeedbackPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_transform_feedback_streams: u32,
    pub max_transform_feedback_buffers: u32,
    pub max_transform_feedback_buffer_size: DeviceSize,
    pub max_transform_feedback_stream_data_size: u32,
    pub max_transform_feedback_buffer_data_size: u32,
    pub max_transform_feedback_buffer_data_stride: u32,
    pub transform_feedback_queries: Bool32,
    pub transform_feedback_streams_lines_triangles: Bool32,
    pub transform_feedback_rasterization_stream_select: Bool32,
    pub transform_feedback_draw: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceTransformFeedbackPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            max_transform_feedback_streams: Default::default(),
            max_transform_feedback_buffers: Default::default(),
            max_transform_feedback_buffer_size: Default::default(),
            max_transform_feedback_stream_data_size: Default::default(),
            max_transform_feedback_buffer_data_size: Default::default(),
            max_transform_feedback_buffer_data_stride: Default::default(),
            transform_feedback_queries: Default::default(),
            transform_feedback_streams_lines_triangles: Default::default(),
            transform_feedback_rasterization_stream_select: Default::default(),
            transform_feedback_draw: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceTransformFeedbackPropertiesEXT<'a> {
    pub fn max_transform_feedback_streams(mut self, max_transform_feedback_streams: u32) -> Self {
        self.max_transform_feedback_streams = max_transform_feedback_streams;
        self
    }
    pub fn max_transform_feedback_buffers(mut self, max_transform_feedback_buffers: u32) -> Self {
        self.max_transform_feedback_buffers = max_transform_feedback_buffers;
        self
    }
    pub fn max_transform_feedback_buffer_size(
        mut self,
        max_transform_feedback_buffer_size: DeviceSize,
    ) -> Self {
        self.max_transform_feedback_buffer_size = max_transform_feedback_buffer_size;
        self
    }
    pub fn max_transform_feedback_stream_data_size(
        mut self,
        max_transform_feedback_stream_data_size: u32,
    ) -> Self {
        self.max_transform_feedback_stream_data_size = max_transform_feedback_stream_data_size;
        self
    }
    pub fn max_transform_feedback_buffer_data_size(
        mut self,
        max_transform_feedback_buffer_data_size: u32,
    ) -> Self {
        self.max_transform_feedback_buffer_data_size = max_transform_feedback_buffer_data_size;
        self
    }
    pub fn max_transform_feedback_buffer_data_stride(
        mut self,
        max_transform_feedback_buffer_data_stride: u32,
    ) -> Self {
        self.max_transform_feedback_buffer_data_stride = max_transform_feedback_buffer_data_stride;
        self
    }
    pub fn transform_feedback_queries(mut self, transform_feedback_queries: Bool32) -> Self {
        self.transform_feedback_queries = transform_feedback_queries;
        self
    }
    pub fn transform_feedback_streams_lines_triangles(
        mut self,
        transform_feedback_streams_lines_triangles: Bool32,
    ) -> Self {
        self.transform_feedback_streams_lines_triangles =
            transform_feedback_streams_lines_triangles;
        self
    }
    pub fn transform_feedback_rasterization_stream_select(
        mut self,
        transform_feedback_rasterization_stream_select: Bool32,
    ) -> Self {
        self.transform_feedback_rasterization_stream_select =
            transform_feedback_rasterization_stream_select;
        self
    }
    pub fn transform_feedback_draw(mut self, transform_feedback_draw: Bool32) -> Self {
        self.transform_feedback_draw = transform_feedback_draw;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationStateStreamCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationStateStreamCreateFlagsEXT,
    pub rasterization_stream: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineRasterizationStateStreamCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            rasterization_stream: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineRasterizationStateStreamCreateInfoEXT<'a> {
    pub fn flags(mut self, flags: PipelineRasterizationStateStreamCreateFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
    pub fn rasterization_stream(mut self, rasterization_stream: u32) -> Self {
        self.rasterization_stream = rasterization_stream;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineRasterizationStateStreamCreateFlagsEXT: Flags {
    }
}
pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
    p_sizes: *const DeviceSize,
);
pub type PFN_vkCmdBeginTransformFeedbackEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    p_counter_buffers: *const Buffer,
    p_counter_buffer_offsets: *const DeviceSize,
);
pub type PFN_vkCmdEndTransformFeedbackEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    p_counter_buffers: *const Buffer,
    p_counter_buffer_offsets: *const DeviceSize,
);
pub type PFN_vkCmdBeginQueryIndexedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
    index: u32,
);
pub type PFN_vkCmdEndQueryIndexedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    index: u32,
);
pub type PFN_vkCmdDrawIndirectByteCountEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    instance_count: u32,
    first_instance: u32,
    counter_buffer: Buffer,
    counter_buffer_offset: DeviceSize,
    counter_offset: u32,
    vertex_stride: u32,
);
