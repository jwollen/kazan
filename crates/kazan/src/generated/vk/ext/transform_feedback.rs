#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceTransformFeedbackFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceTransformFeedbackFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub transform_feedback: Bool32,
        pub geometry_streams: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTransformFeedbackFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceTransformFeedbackFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceTransformFeedbackFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceTransformFeedbackFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                transform_feedback: Default::default(),
                geometry_streams: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceTransformFeedbackFeaturesEXT<'a> {
        pub fn transform_feedback(mut self, transform_feedback: bool) -> Self {
            self.transform_feedback = transform_feedback.into();
            self
        }
        pub fn geometry_streams(mut self, geometry_streams: bool) -> Self {
            self.geometry_streams = geometry_streams.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceTransformFeedbackPropertiesEXT.html>
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTransformFeedbackPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceTransformFeedbackPropertiesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceTransformFeedbackPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
        pub fn max_transform_feedback_streams(
            mut self,
            max_transform_feedback_streams: u32,
        ) -> Self {
            self.max_transform_feedback_streams = max_transform_feedback_streams;
            self
        }
        pub fn max_transform_feedback_buffers(
            mut self,
            max_transform_feedback_buffers: u32,
        ) -> Self {
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
            self.max_transform_feedback_buffer_data_stride =
                max_transform_feedback_buffer_data_stride;
            self
        }
        pub fn transform_feedback_queries(mut self, transform_feedback_queries: bool) -> Self {
            self.transform_feedback_queries = transform_feedback_queries.into();
            self
        }
        pub fn transform_feedback_streams_lines_triangles(
            mut self,
            transform_feedback_streams_lines_triangles: bool,
        ) -> Self {
            self.transform_feedback_streams_lines_triangles =
                transform_feedback_streams_lines_triangles.into();
            self
        }
        pub fn transform_feedback_rasterization_stream_select(
            mut self,
            transform_feedback_rasterization_stream_select: bool,
        ) -> Self {
            self.transform_feedback_rasterization_stream_select =
                transform_feedback_rasterization_stream_select.into();
            self
        }
        pub fn transform_feedback_draw(mut self, transform_feedback_draw: bool) -> Self {
            self.transform_feedback_draw = transform_feedback_draw.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRasterizationStateStreamCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineRasterizationStateStreamCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineRasterizationStateStreamCreateFlagsEXT,
        pub rasterization_stream: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineRasterizationStateStreamCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT;
    }
    unsafe impl<'a> Extends<PipelineRasterizationStateCreateInfo<'a>>
        for PipelineRasterizationStateStreamCreateInfoEXT<'a>
    {
    }
    impl Default for PipelineRasterizationStateStreamCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRasterizationStateStreamCreateFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineRasterizationStateStreamCreateFlagsEXT(Flags);
    vk_bitflags_wrapped!(PipelineRasterizationStateStreamCreateFlagsEXT, Flags);
    impl fmt::Debug for PipelineRasterizationStateStreamCreateFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindTransformFeedbackBuffersEXT.html>
    pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
        p_sizes: *const DeviceSize,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginTransformFeedbackEXT.html>
    pub type PFN_vkCmdBeginTransformFeedbackEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const Buffer,
        p_counter_buffer_offsets: *const DeviceSize,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndTransformFeedbackEXT.html>
    pub type PFN_vkCmdEndTransformFeedbackEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const Buffer,
        p_counter_buffer_offsets: *const DeviceSize,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginQueryIndexedEXT.html>
    pub type PFN_vkCmdBeginQueryIndexedEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
        index: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndQueryIndexedEXT.html>
    pub type PFN_vkCmdEndQueryIndexedEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        index: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndirectByteCountEXT.html>
    pub type PFN_vkCmdDrawIndirectByteCountEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    );
}
pub struct DeviceFn {
    cmd_bind_transform_feedback_buffers_ext: PFN_vkCmdBindTransformFeedbackBuffersEXT,
    cmd_begin_transform_feedback_ext: PFN_vkCmdBeginTransformFeedbackEXT,
    cmd_end_transform_feedback_ext: PFN_vkCmdEndTransformFeedbackEXT,
    cmd_begin_query_indexed_ext: PFN_vkCmdBeginQueryIndexedEXT,
    cmd_end_query_indexed_ext: PFN_vkCmdEndQueryIndexedEXT,
    cmd_draw_indirect_byte_count_ext: PFN_vkCmdDrawIndirectByteCountEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_bind_transform_feedback_buffers_ext: transmute(
                    load(c"vkCmdBindTransformFeedbackBuffersEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_begin_transform_feedback_ext: transmute(
                    load(c"vkCmdBeginTransformFeedbackEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_transform_feedback_ext: transmute(
                    load(c"vkCmdEndTransformFeedbackEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_begin_query_indexed_ext: transmute(
                    load(c"vkCmdBeginQueryIndexedEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_query_indexed_ext: transmute(
                    load(c"vkCmdEndQueryIndexedEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_indirect_byte_count_ext: transmute(
                    load(c"vkCmdDrawIndirectByteCountEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindTransformFeedbackBuffersEXT.html>
    pub unsafe fn cmd_bind_transform_feedback_buffers_ext(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
    ) {
        unsafe {
            (self.cmd_bind_transform_feedback_buffers_ext)(
                command_buffer,
                first_binding,
                buffers.len().try_into().unwrap(),
                buffers.as_ptr() as _,
                offsets.as_ptr() as _,
                sizes.to_raw_ptr(),
            )
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginTransformFeedbackEXT.html>
    pub unsafe fn cmd_begin_transform_feedback_ext(
        &self,
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffers: &[Buffer],
        counter_buffer_offsets: Option<&[DeviceSize]>,
    ) {
        unsafe {
            (self.cmd_begin_transform_feedback_ext)(
                command_buffer,
                first_counter_buffer,
                counter_buffers.len().try_into().unwrap(),
                counter_buffers.as_ptr() as _,
                counter_buffer_offsets.to_raw_ptr(),
            )
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndTransformFeedbackEXT.html>
    pub unsafe fn cmd_end_transform_feedback_ext(
        &self,
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffers: &[Buffer],
        counter_buffer_offsets: Option<&[DeviceSize]>,
    ) {
        unsafe {
            (self.cmd_end_transform_feedback_ext)(
                command_buffer,
                first_counter_buffer,
                counter_buffers.len().try_into().unwrap(),
                counter_buffers.as_ptr() as _,
                counter_buffer_offsets.to_raw_ptr(),
            )
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginQueryIndexedEXT.html>
    pub unsafe fn cmd_begin_query_indexed_ext(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
        index: u32,
    ) {
        unsafe {
            (self.cmd_begin_query_indexed_ext)(command_buffer, query_pool, query, flags, index)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndQueryIndexedEXT.html>
    pub unsafe fn cmd_end_query_indexed_ext(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        index: u32,
    ) {
        unsafe { (self.cmd_end_query_indexed_ext)(command_buffer, query_pool, query, index) }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndirectByteCountEXT.html>
    pub unsafe fn cmd_draw_indirect_byte_count_ext(
        &self,
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_indirect_byte_count_ext)(
                command_buffer,
                instance_count,
                first_instance,
                counter_buffer,
                counter_buffer_offset,
                counter_offset,
                vertex_stride,
            )
        }
    }
}
