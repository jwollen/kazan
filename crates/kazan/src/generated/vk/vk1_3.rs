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
    pub type Flags64 = u64;
    handle_nondispatchable!(PrivateDataSlot, PRIVATE_DATA_SLOT, doc = "");
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DevicePrivateDataCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub private_data_slot_request_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DevicePrivateDataCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_PRIVATE_DATA_CREATE_INFO;
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for DevicePrivateDataCreateInfo<'a> {}
    impl Default for DevicePrivateDataCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                private_data_slot_request_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DevicePrivateDataCreateInfo<'a> {
        pub fn private_data_slot_request_count(
            mut self,
            private_data_slot_request_count: u32,
        ) -> Self {
            self.private_data_slot_request_count = private_data_slot_request_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PrivateDataSlotCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PrivateDataSlotCreateFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PrivateDataSlotCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRIVATE_DATA_SLOT_CREATE_INFO;
    }
    impl Default for PrivateDataSlotCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PrivateDataSlotCreateInfo<'a> {
        pub fn flags(mut self, flags: PrivateDataSlotCreateFlags) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePrivateDataFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub private_data: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePrivateDataFeatures<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDevicePrivateDataFeatures<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevicePrivateDataFeatures<'a> {}
    impl Default for PhysicalDevicePrivateDataFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                private_data: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePrivateDataFeatures<'a> {
        pub fn private_data(mut self, private_data: Bool32) -> Self {
            self.private_data = private_data;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceBufferMemoryRequirements<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_create_info: *const BufferCreateInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceBufferMemoryRequirements<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_BUFFER_MEMORY_REQUIREMENTS;
    }
    impl Default for DeviceBufferMemoryRequirements<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_create_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceBufferMemoryRequirements<'a> {
        pub fn create_info(mut self, create_info: &'a BufferCreateInfo<'a>) -> Self {
            self.p_create_info = create_info;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceImageMemoryRequirements<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_create_info: *const ImageCreateInfo<'a>,
        pub plane_aspect: ImageAspectFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceImageMemoryRequirements<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_IMAGE_MEMORY_REQUIREMENTS;
    }
    impl Default for DeviceImageMemoryRequirements<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_create_info: core::ptr::null(),
                plane_aspect: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceImageMemoryRequirements<'a> {
        pub fn create_info(mut self, create_info: &'a ImageCreateInfo<'a>) -> Self {
            self.p_create_info = create_info;
            self
        }
        pub fn plane_aspect(mut self, plane_aspect: ImageAspectFlagBits) -> Self {
            self.plane_aspect = plane_aspect;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceInlineUniformBlockFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub inline_uniform_block: Bool32,
        pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceInlineUniformBlockFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceInlineUniformBlockFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceInlineUniformBlockFeatures<'a> {}
    impl Default for PhysicalDeviceInlineUniformBlockFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                inline_uniform_block: Default::default(),
                descriptor_binding_inline_uniform_block_update_after_bind: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceInlineUniformBlockFeatures<'a> {
        pub fn inline_uniform_block(mut self, inline_uniform_block: Bool32) -> Self {
            self.inline_uniform_block = inline_uniform_block;
            self
        }
        pub fn descriptor_binding_inline_uniform_block_update_after_bind(
            mut self,
            descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_inline_uniform_block_update_after_bind =
                descriptor_binding_inline_uniform_block_update_after_bind;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceInlineUniformBlockProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_inline_uniform_block_size: u32,
        pub max_per_stage_descriptor_inline_uniform_blocks: u32,
        pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
        pub max_descriptor_set_inline_uniform_blocks: u32,
        pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceInlineUniformBlockProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceInlineUniformBlockProperties<'a>
    {
    }
    impl Default for PhysicalDeviceInlineUniformBlockProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_inline_uniform_block_size: Default::default(),
                max_per_stage_descriptor_inline_uniform_blocks: Default::default(),
                max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: Default::default(
                ),
                max_descriptor_set_inline_uniform_blocks: Default::default(),
                max_descriptor_set_update_after_bind_inline_uniform_blocks: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceInlineUniformBlockProperties<'a> {
        pub fn max_inline_uniform_block_size(mut self, max_inline_uniform_block_size: u32) -> Self {
            self.max_inline_uniform_block_size = max_inline_uniform_block_size;
            self
        }
        pub fn max_per_stage_descriptor_inline_uniform_blocks(
            mut self,
            max_per_stage_descriptor_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_per_stage_descriptor_inline_uniform_blocks =
                max_per_stage_descriptor_inline_uniform_blocks;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(
            mut self,
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks =
                max_per_stage_descriptor_update_after_bind_inline_uniform_blocks;
            self
        }
        pub fn max_descriptor_set_inline_uniform_blocks(
            mut self,
            max_descriptor_set_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_descriptor_set_inline_uniform_blocks =
                max_descriptor_set_inline_uniform_blocks;
            self
        }
        pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks(
            mut self,
            max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_inline_uniform_blocks =
                max_descriptor_set_update_after_bind_inline_uniform_blocks;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct WriteDescriptorSetInlineUniformBlock<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub data_size: u32,
        pub p_data: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for WriteDescriptorSetInlineUniformBlock<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK;
    }
    unsafe impl<'a> Extends<WriteDescriptorSet<'a>> for WriteDescriptorSetInlineUniformBlock<'a> {}
    impl Default for WriteDescriptorSetInlineUniformBlock<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                data_size: Default::default(),
                p_data: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> WriteDescriptorSetInlineUniformBlock<'a> {
        pub fn data(mut self, data: &'a [u8]) -> Self {
            self.data_size = data.len().try_into().unwrap();
            self.p_data = data.as_ptr() as _;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorPoolInlineUniformBlockCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub max_inline_uniform_block_bindings: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DescriptorPoolInlineUniformBlockCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO;
    }
    unsafe impl<'a> Extends<DescriptorPoolCreateInfo<'a>>
        for DescriptorPoolInlineUniformBlockCreateInfo<'a>
    {
    }
    impl Default for DescriptorPoolInlineUniformBlockCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                max_inline_uniform_block_bindings: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DescriptorPoolInlineUniformBlockCreateInfo<'a> {
        pub fn max_inline_uniform_block_bindings(
            mut self,
            max_inline_uniform_block_bindings: u32,
        ) -> Self {
            self.max_inline_uniform_block_bindings = max_inline_uniform_block_bindings;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMaintenance4Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub maintenance4: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance4Features<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceMaintenance4Features<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMaintenance4Features<'a> {}
    impl Default for PhysicalDeviceMaintenance4Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                maintenance4: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMaintenance4Features<'a> {
        pub fn maintenance4(mut self, maintenance4: Bool32) -> Self {
            self.maintenance4 = maintenance4;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMaintenance4Properties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_buffer_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance4Properties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMaintenance4Properties<'a>
    {
    }
    impl Default for PhysicalDeviceMaintenance4Properties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_buffer_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMaintenance4Properties<'a> {
        pub fn max_buffer_size(mut self, max_buffer_size: DeviceSize) -> Self {
            self.max_buffer_size = max_buffer_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceTextureCompressionASTCHDRFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub texture_compression_astc_hdr: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTextureCompressionASTCHDRFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceTextureCompressionASTCHDRFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceTextureCompressionASTCHDRFeatures<'a>
    {
    }
    impl Default for PhysicalDeviceTextureCompressionASTCHDRFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                texture_compression_astc_hdr: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceTextureCompressionASTCHDRFeatures<'a> {
        pub fn texture_compression_astc_hdr(
            mut self,
            texture_compression_astc_hdr: Bool32,
        ) -> Self {
            self.texture_compression_astc_hdr = texture_compression_astc_hdr;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct PipelineCreationFeedback {
        pub flags: PipelineCreationFeedbackFlags,
        pub duration: u64,
    }
    impl PipelineCreationFeedback {
        pub fn flags(mut self, flags: PipelineCreationFeedbackFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn duration(mut self, duration: u64) -> Self {
            self.duration = duration;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineCreationFeedbackCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_pipeline_creation_feedback: *mut PipelineCreationFeedback,
        pub pipeline_stage_creation_feedback_count: u32,
        pub p_pipeline_stage_creation_feedbacks: *mut PipelineCreationFeedback,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineCreationFeedbackCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_CREATION_FEEDBACK_CREATE_INFO;
    }
    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>> for PipelineCreationFeedbackCreateInfo<'a> {}
    unsafe impl<'a> Extends<ComputePipelineCreateInfo<'a>> for PipelineCreationFeedbackCreateInfo<'a> {}
    unsafe impl<'a> Extends<RayTracingPipelineCreateInfoNV<'a>>
        for PipelineCreationFeedbackCreateInfo<'a>
    {
    }
    unsafe impl<'a> Extends<RayTracingPipelineCreateInfoKHR<'a>>
        for PipelineCreationFeedbackCreateInfo<'a>
    {
    }
    unsafe impl<'a> Extends<ExecutionGraphPipelineCreateInfoAMDX<'a>>
        for PipelineCreationFeedbackCreateInfo<'a>
    {
    }
    unsafe impl<'a> Extends<DataGraphPipelineCreateInfoARM<'a>>
        for PipelineCreationFeedbackCreateInfo<'a>
    {
    }
    impl Default for PipelineCreationFeedbackCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_pipeline_creation_feedback: core::ptr::null_mut(),
                pipeline_stage_creation_feedback_count: Default::default(),
                p_pipeline_stage_creation_feedbacks: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineCreationFeedbackCreateInfo<'a> {
        pub fn pipeline_creation_feedback(
            mut self,
            pipeline_creation_feedback: &'a mut PipelineCreationFeedback,
        ) -> Self {
            self.p_pipeline_creation_feedback = pipeline_creation_feedback;
            self
        }
        pub fn pipeline_stage_creation_feedbacks(
            mut self,
            pipeline_stage_creation_feedbacks: &'a mut [PipelineCreationFeedback],
        ) -> Self {
            self.pipeline_stage_creation_feedback_count =
                pipeline_stage_creation_feedbacks.len().try_into().unwrap();
            self.p_pipeline_stage_creation_feedbacks =
                pipeline_stage_creation_feedbacks.as_mut_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_demote_to_helper_invocation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a>
    {
    }
    impl Default for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_demote_to_helper_invocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a> {
        pub fn shader_demote_to_helper_invocation(
            mut self,
            shader_demote_to_helper_invocation: Bool32,
        ) -> Self {
            self.shader_demote_to_helper_invocation = shader_demote_to_helper_invocation;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceTexelBufferAlignmentProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
        pub storage_texel_buffer_offset_single_texel_alignment: Bool32,
        pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
        pub uniform_texel_buffer_offset_single_texel_alignment: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTexelBufferAlignmentProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceTexelBufferAlignmentProperties<'a>
    {
    }
    impl Default for PhysicalDeviceTexelBufferAlignmentProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                storage_texel_buffer_offset_alignment_bytes: Default::default(),
                storage_texel_buffer_offset_single_texel_alignment: Default::default(),
                uniform_texel_buffer_offset_alignment_bytes: Default::default(),
                uniform_texel_buffer_offset_single_texel_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceTexelBufferAlignmentProperties<'a> {
        pub fn storage_texel_buffer_offset_alignment_bytes(
            mut self,
            storage_texel_buffer_offset_alignment_bytes: DeviceSize,
        ) -> Self {
            self.storage_texel_buffer_offset_alignment_bytes =
                storage_texel_buffer_offset_alignment_bytes;
            self
        }
        pub fn storage_texel_buffer_offset_single_texel_alignment(
            mut self,
            storage_texel_buffer_offset_single_texel_alignment: Bool32,
        ) -> Self {
            self.storage_texel_buffer_offset_single_texel_alignment =
                storage_texel_buffer_offset_single_texel_alignment;
            self
        }
        pub fn uniform_texel_buffer_offset_alignment_bytes(
            mut self,
            uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
        ) -> Self {
            self.uniform_texel_buffer_offset_alignment_bytes =
                uniform_texel_buffer_offset_alignment_bytes;
            self
        }
        pub fn uniform_texel_buffer_offset_single_texel_alignment(
            mut self,
            uniform_texel_buffer_offset_single_texel_alignment: Bool32,
        ) -> Self {
            self.uniform_texel_buffer_offset_single_texel_alignment =
                uniform_texel_buffer_offset_single_texel_alignment;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceSubgroupSizeControlFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub subgroup_size_control: Bool32,
        pub compute_full_subgroups: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSubgroupSizeControlFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceSubgroupSizeControlFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceSubgroupSizeControlFeatures<'a> {}
    impl Default for PhysicalDeviceSubgroupSizeControlFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                subgroup_size_control: Default::default(),
                compute_full_subgroups: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceSubgroupSizeControlFeatures<'a> {
        pub fn subgroup_size_control(mut self, subgroup_size_control: Bool32) -> Self {
            self.subgroup_size_control = subgroup_size_control;
            self
        }
        pub fn compute_full_subgroups(mut self, compute_full_subgroups: Bool32) -> Self {
            self.compute_full_subgroups = compute_full_subgroups;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceSubgroupSizeControlProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_subgroup_size: u32,
        pub max_subgroup_size: u32,
        pub max_compute_workgroup_subgroups: u32,
        pub required_subgroup_size_stages: ShaderStageFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSubgroupSizeControlProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceSubgroupSizeControlProperties<'a>
    {
    }
    impl Default for PhysicalDeviceSubgroupSizeControlProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                min_subgroup_size: Default::default(),
                max_subgroup_size: Default::default(),
                max_compute_workgroup_subgroups: Default::default(),
                required_subgroup_size_stages: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceSubgroupSizeControlProperties<'a> {
        pub fn min_subgroup_size(mut self, min_subgroup_size: u32) -> Self {
            self.min_subgroup_size = min_subgroup_size;
            self
        }
        pub fn max_subgroup_size(mut self, max_subgroup_size: u32) -> Self {
            self.max_subgroup_size = max_subgroup_size;
            self
        }
        pub fn max_compute_workgroup_subgroups(
            mut self,
            max_compute_workgroup_subgroups: u32,
        ) -> Self {
            self.max_compute_workgroup_subgroups = max_compute_workgroup_subgroups;
            self
        }
        pub fn required_subgroup_size_stages(
            mut self,
            required_subgroup_size_stages: ShaderStageFlags,
        ) -> Self {
            self.required_subgroup_size_stages = required_subgroup_size_stages;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub required_subgroup_size: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
    }
    unsafe impl<'a> Extends<PipelineShaderStageCreateInfo<'a>>
        for PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a>
    {
    }
    unsafe impl<'a> Extends<ShaderCreateInfoEXT<'a>>
        for PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a>
    {
    }
    impl Default for PipelineShaderStageRequiredSubgroupSizeCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                required_subgroup_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a> {
        pub fn required_subgroup_size(mut self, required_subgroup_size: u32) -> Self {
            self.required_subgroup_size = required_subgroup_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePipelineCreationCacheControlFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_creation_cache_control: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelineCreationCacheControlFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePipelineCreationCacheControlFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDevicePipelineCreationCacheControlFeatures<'a>
    {
    }
    impl Default for PhysicalDevicePipelineCreationCacheControlFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                pipeline_creation_cache_control: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePipelineCreationCacheControlFeatures<'a> {
        pub fn pipeline_creation_cache_control(
            mut self,
            pipeline_creation_cache_control: Bool32,
        ) -> Self {
            self.pipeline_creation_cache_control = pipeline_creation_cache_control;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVulkan13Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub robust_image_access: Bool32,
        pub inline_uniform_block: Bool32,
        pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
        pub pipeline_creation_cache_control: Bool32,
        pub private_data: Bool32,
        pub shader_demote_to_helper_invocation: Bool32,
        pub shader_terminate_invocation: Bool32,
        pub subgroup_size_control: Bool32,
        pub compute_full_subgroups: Bool32,
        pub synchronization2: Bool32,
        pub texture_compression_astc_hdr: Bool32,
        pub shader_zero_initialize_workgroup_memory: Bool32,
        pub dynamic_rendering: Bool32,
        pub shader_integer_dot_product: Bool32,
        pub maintenance4: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVulkan13Features<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceVulkan13Features<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceVulkan13Features<'a> {}
    impl Default for PhysicalDeviceVulkan13Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                robust_image_access: Default::default(),
                inline_uniform_block: Default::default(),
                descriptor_binding_inline_uniform_block_update_after_bind: Default::default(),
                pipeline_creation_cache_control: Default::default(),
                private_data: Default::default(),
                shader_demote_to_helper_invocation: Default::default(),
                shader_terminate_invocation: Default::default(),
                subgroup_size_control: Default::default(),
                compute_full_subgroups: Default::default(),
                synchronization2: Default::default(),
                texture_compression_astc_hdr: Default::default(),
                shader_zero_initialize_workgroup_memory: Default::default(),
                dynamic_rendering: Default::default(),
                shader_integer_dot_product: Default::default(),
                maintenance4: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVulkan13Features<'a> {
        pub fn robust_image_access(mut self, robust_image_access: Bool32) -> Self {
            self.robust_image_access = robust_image_access;
            self
        }
        pub fn inline_uniform_block(mut self, inline_uniform_block: Bool32) -> Self {
            self.inline_uniform_block = inline_uniform_block;
            self
        }
        pub fn descriptor_binding_inline_uniform_block_update_after_bind(
            mut self,
            descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_inline_uniform_block_update_after_bind =
                descriptor_binding_inline_uniform_block_update_after_bind;
            self
        }
        pub fn pipeline_creation_cache_control(
            mut self,
            pipeline_creation_cache_control: Bool32,
        ) -> Self {
            self.pipeline_creation_cache_control = pipeline_creation_cache_control;
            self
        }
        pub fn private_data(mut self, private_data: Bool32) -> Self {
            self.private_data = private_data;
            self
        }
        pub fn shader_demote_to_helper_invocation(
            mut self,
            shader_demote_to_helper_invocation: Bool32,
        ) -> Self {
            self.shader_demote_to_helper_invocation = shader_demote_to_helper_invocation;
            self
        }
        pub fn shader_terminate_invocation(mut self, shader_terminate_invocation: Bool32) -> Self {
            self.shader_terminate_invocation = shader_terminate_invocation;
            self
        }
        pub fn subgroup_size_control(mut self, subgroup_size_control: Bool32) -> Self {
            self.subgroup_size_control = subgroup_size_control;
            self
        }
        pub fn compute_full_subgroups(mut self, compute_full_subgroups: Bool32) -> Self {
            self.compute_full_subgroups = compute_full_subgroups;
            self
        }
        pub fn synchronization2(mut self, synchronization2: Bool32) -> Self {
            self.synchronization2 = synchronization2;
            self
        }
        pub fn texture_compression_astc_hdr(
            mut self,
            texture_compression_astc_hdr: Bool32,
        ) -> Self {
            self.texture_compression_astc_hdr = texture_compression_astc_hdr;
            self
        }
        pub fn shader_zero_initialize_workgroup_memory(
            mut self,
            shader_zero_initialize_workgroup_memory: Bool32,
        ) -> Self {
            self.shader_zero_initialize_workgroup_memory = shader_zero_initialize_workgroup_memory;
            self
        }
        pub fn dynamic_rendering(mut self, dynamic_rendering: Bool32) -> Self {
            self.dynamic_rendering = dynamic_rendering;
            self
        }
        pub fn shader_integer_dot_product(mut self, shader_integer_dot_product: Bool32) -> Self {
            self.shader_integer_dot_product = shader_integer_dot_product;
            self
        }
        pub fn maintenance4(mut self, maintenance4: Bool32) -> Self {
            self.maintenance4 = maintenance4;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVulkan13Properties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_subgroup_size: u32,
        pub max_subgroup_size: u32,
        pub max_compute_workgroup_subgroups: u32,
        pub required_subgroup_size_stages: ShaderStageFlags,
        pub max_inline_uniform_block_size: u32,
        pub max_per_stage_descriptor_inline_uniform_blocks: u32,
        pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
        pub max_descriptor_set_inline_uniform_blocks: u32,
        pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
        pub max_inline_uniform_total_size: u32,
        pub integer_dot_product8_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product8_bit_signed_accelerated: Bool32,
        pub integer_dot_product8_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product4x8_bit_packed_unsigned_accelerated: Bool32,
        pub integer_dot_product4x8_bit_packed_signed_accelerated: Bool32,
        pub integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product16_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product16_bit_signed_accelerated: Bool32,
        pub integer_dot_product16_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product32_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product32_bit_signed_accelerated: Bool32,
        pub integer_dot_product32_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product64_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product64_bit_signed_accelerated: Bool32,
        pub integer_dot_product64_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated:
            Bool32,
        pub integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Bool32,
        pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
        pub storage_texel_buffer_offset_single_texel_alignment: Bool32,
        pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
        pub uniform_texel_buffer_offset_single_texel_alignment: Bool32,
        pub max_buffer_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVulkan13Properties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>> for PhysicalDeviceVulkan13Properties<'a> {}
    impl Default for PhysicalDeviceVulkan13Properties<'_> {
        fn default() -> Self {
            Self {
s_type: Self::STRUCTURE_TYPE
,
p_next: core::ptr::null_mut(),
min_subgroup_size: Default::default(),
max_subgroup_size: Default::default(),
max_compute_workgroup_subgroups: Default::default(),
required_subgroup_size_stages: Default::default(),
max_inline_uniform_block_size: Default::default(),
max_per_stage_descriptor_inline_uniform_blocks: Default::default(),
max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: Default::default(),
max_descriptor_set_inline_uniform_blocks: Default::default(),
max_descriptor_set_update_after_bind_inline_uniform_blocks: Default::default(),
max_inline_uniform_total_size: Default::default(),
integer_dot_product8_bit_unsigned_accelerated: Default::default(),
integer_dot_product8_bit_signed_accelerated: Default::default(),
integer_dot_product8_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product4x8_bit_packed_unsigned_accelerated: Default::default(),
integer_dot_product4x8_bit_packed_signed_accelerated: Default::default(),
integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Default::default(),
integer_dot_product16_bit_unsigned_accelerated: Default::default(),
integer_dot_product16_bit_signed_accelerated: Default::default(),
integer_dot_product16_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product32_bit_unsigned_accelerated: Default::default(),
integer_dot_product32_bit_signed_accelerated: Default::default(),
integer_dot_product32_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product64_bit_unsigned_accelerated: Default::default(),
integer_dot_product64_bit_signed_accelerated: Default::default(),
integer_dot_product64_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Default::default(),
storage_texel_buffer_offset_alignment_bytes: Default::default(),
storage_texel_buffer_offset_single_texel_alignment: Default::default(),
uniform_texel_buffer_offset_alignment_bytes: Default::default(),
uniform_texel_buffer_offset_single_texel_alignment: Default::default(),
max_buffer_size: Default::default(),
_marker: PhantomData
}
        }
    }
    impl<'a> PhysicalDeviceVulkan13Properties<'a> {
        pub fn min_subgroup_size(mut self, min_subgroup_size: u32) -> Self {
            self.min_subgroup_size = min_subgroup_size;
            self
        }
        pub fn max_subgroup_size(mut self, max_subgroup_size: u32) -> Self {
            self.max_subgroup_size = max_subgroup_size;
            self
        }
        pub fn max_compute_workgroup_subgroups(
            mut self,
            max_compute_workgroup_subgroups: u32,
        ) -> Self {
            self.max_compute_workgroup_subgroups = max_compute_workgroup_subgroups;
            self
        }
        pub fn required_subgroup_size_stages(
            mut self,
            required_subgroup_size_stages: ShaderStageFlags,
        ) -> Self {
            self.required_subgroup_size_stages = required_subgroup_size_stages;
            self
        }
        pub fn max_inline_uniform_block_size(mut self, max_inline_uniform_block_size: u32) -> Self {
            self.max_inline_uniform_block_size = max_inline_uniform_block_size;
            self
        }
        pub fn max_per_stage_descriptor_inline_uniform_blocks(
            mut self,
            max_per_stage_descriptor_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_per_stage_descriptor_inline_uniform_blocks =
                max_per_stage_descriptor_inline_uniform_blocks;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(
            mut self,
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks =
                max_per_stage_descriptor_update_after_bind_inline_uniform_blocks;
            self
        }
        pub fn max_descriptor_set_inline_uniform_blocks(
            mut self,
            max_descriptor_set_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_descriptor_set_inline_uniform_blocks =
                max_descriptor_set_inline_uniform_blocks;
            self
        }
        pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks(
            mut self,
            max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_inline_uniform_blocks =
                max_descriptor_set_update_after_bind_inline_uniform_blocks;
            self
        }
        pub fn max_inline_uniform_total_size(mut self, max_inline_uniform_total_size: u32) -> Self {
            self.max_inline_uniform_total_size = max_inline_uniform_total_size;
            self
        }
        pub fn integer_dot_product8_bit_unsigned_accelerated(
            mut self,
            integer_dot_product8_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product8_bit_unsigned_accelerated =
                integer_dot_product8_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product8_bit_signed_accelerated(
            mut self,
            integer_dot_product8_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product8_bit_signed_accelerated =
                integer_dot_product8_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product8_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product8_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product8_bit_mixed_signedness_accelerated =
                integer_dot_product8_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product4x8_bit_packed_unsigned_accelerated(
            mut self,
            integer_dot_product4x8_bit_packed_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product4x8_bit_packed_unsigned_accelerated =
                integer_dot_product4x8_bit_packed_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product4x8_bit_packed_signed_accelerated(
            mut self,
            integer_dot_product4x8_bit_packed_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product4x8_bit_packed_signed_accelerated =
                integer_dot_product4x8_bit_packed_signed_accelerated;
            self
        }
        pub fn integer_dot_product4x8_bit_packed_mixed_signedness_accelerated(
            mut self,
            integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated =
                integer_dot_product4x8_bit_packed_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product16_bit_unsigned_accelerated(
            mut self,
            integer_dot_product16_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product16_bit_unsigned_accelerated =
                integer_dot_product16_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product16_bit_signed_accelerated(
            mut self,
            integer_dot_product16_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product16_bit_signed_accelerated =
                integer_dot_product16_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product16_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product16_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product16_bit_mixed_signedness_accelerated =
                integer_dot_product16_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product32_bit_unsigned_accelerated(
            mut self,
            integer_dot_product32_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product32_bit_unsigned_accelerated =
                integer_dot_product32_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product32_bit_signed_accelerated(
            mut self,
            integer_dot_product32_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product32_bit_signed_accelerated =
                integer_dot_product32_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product32_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product32_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product32_bit_mixed_signedness_accelerated =
                integer_dot_product32_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product64_bit_unsigned_accelerated(
            mut self,
            integer_dot_product64_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product64_bit_unsigned_accelerated =
                integer_dot_product64_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product64_bit_signed_accelerated(
            mut self,
            integer_dot_product64_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product64_bit_signed_accelerated =
                integer_dot_product64_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product64_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product64_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product64_bit_mixed_signedness_accelerated =
                integer_dot_product64_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating8_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating8_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating8_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated =
                integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated =
                integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated = integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating16_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating16_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating16_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating32_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating32_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating32_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating64_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating64_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating64_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated;
            self
        }
        pub fn storage_texel_buffer_offset_alignment_bytes(
            mut self,
            storage_texel_buffer_offset_alignment_bytes: DeviceSize,
        ) -> Self {
            self.storage_texel_buffer_offset_alignment_bytes =
                storage_texel_buffer_offset_alignment_bytes;
            self
        }
        pub fn storage_texel_buffer_offset_single_texel_alignment(
            mut self,
            storage_texel_buffer_offset_single_texel_alignment: Bool32,
        ) -> Self {
            self.storage_texel_buffer_offset_single_texel_alignment =
                storage_texel_buffer_offset_single_texel_alignment;
            self
        }
        pub fn uniform_texel_buffer_offset_alignment_bytes(
            mut self,
            uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
        ) -> Self {
            self.uniform_texel_buffer_offset_alignment_bytes =
                uniform_texel_buffer_offset_alignment_bytes;
            self
        }
        pub fn uniform_texel_buffer_offset_single_texel_alignment(
            mut self,
            uniform_texel_buffer_offset_single_texel_alignment: Bool32,
        ) -> Self {
            self.uniform_texel_buffer_offset_single_texel_alignment =
                uniform_texel_buffer_offset_single_texel_alignment;
            self
        }
        pub fn max_buffer_size(mut self, max_buffer_size: DeviceSize) -> Self {
            self.max_buffer_size = max_buffer_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceToolProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub name: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
        pub version: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
        pub purposes: ToolPurposeFlags,
        pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub layer: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceToolProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_TOOL_PROPERTIES;
    }
    impl Default for PhysicalDeviceToolProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                name: [Default::default(); _],
                version: [Default::default(); _],
                purposes: Default::default(),
                description: [Default::default(); _],
                layer: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceToolProperties<'a> {
        pub fn purposes(mut self, purposes: ToolPurposeFlags) -> Self {
            self.purposes = purposes;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_zero_initialize_workgroup_memory: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a>
    {
    }
    impl Default for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_zero_initialize_workgroup_memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a> {
        pub fn shader_zero_initialize_workgroup_memory(
            mut self,
            shader_zero_initialize_workgroup_memory: Bool32,
        ) -> Self {
            self.shader_zero_initialize_workgroup_memory = shader_zero_initialize_workgroup_memory;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceImageRobustnessFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub robust_image_access: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageRobustnessFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceImageRobustnessFeatures<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceImageRobustnessFeatures<'a> {}
    impl Default for PhysicalDeviceImageRobustnessFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                robust_image_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceImageRobustnessFeatures<'a> {
        pub fn robust_image_access(mut self, robust_image_access: Bool32) -> Self {
            self.robust_image_access = robust_image_access;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BufferCopy2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_offset: DeviceSize,
        pub dst_offset: DeviceSize,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BufferCopy2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_COPY_2;
    }
    impl Default for BufferCopy2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_offset: Default::default(),
                dst_offset: Default::default(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BufferCopy2<'a> {
        pub fn src_offset(mut self, src_offset: DeviceSize) -> Self {
            self.src_offset = src_offset;
            self
        }
        pub fn dst_offset(mut self, dst_offset: DeviceSize) -> Self {
            self.dst_offset = dst_offset;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageCopy2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_subresource: ImageSubresourceLayers,
        pub src_offset: Offset3D,
        pub dst_subresource: ImageSubresourceLayers,
        pub dst_offset: Offset3D,
        pub extent: Extent3D,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageCopy2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_COPY_2;
    }
    impl Default for ImageCopy2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_subresource: Default::default(),
                src_offset: Default::default(),
                dst_subresource: Default::default(),
                dst_offset: Default::default(),
                extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageCopy2<'a> {
        pub fn src_subresource(mut self, src_subresource: ImageSubresourceLayers) -> Self {
            self.src_subresource = src_subresource;
            self
        }
        pub fn src_offset(mut self, src_offset: Offset3D) -> Self {
            self.src_offset = src_offset;
            self
        }
        pub fn dst_subresource(mut self, dst_subresource: ImageSubresourceLayers) -> Self {
            self.dst_subresource = dst_subresource;
            self
        }
        pub fn dst_offset(mut self, dst_offset: Offset3D) -> Self {
            self.dst_offset = dst_offset;
            self
        }
        pub fn extent(mut self, extent: Extent3D) -> Self {
            self.extent = extent;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageBlit2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_subresource: ImageSubresourceLayers,
        pub src_offsets: [Offset3D; 2],
        pub dst_subresource: ImageSubresourceLayers,
        pub dst_offsets: [Offset3D; 2],
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageBlit2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_BLIT_2;
    }
    impl Default for ImageBlit2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_subresource: Default::default(),
                src_offsets: [Default::default(); _],
                dst_subresource: Default::default(),
                dst_offsets: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageBlit2<'a> {
        pub fn src_subresource(mut self, src_subresource: ImageSubresourceLayers) -> Self {
            self.src_subresource = src_subresource;
            self
        }
        pub fn src_offsets(mut self, src_offsets: [Offset3D; 2]) -> Self {
            self.src_offsets = src_offsets;
            self
        }
        pub fn dst_subresource(mut self, dst_subresource: ImageSubresourceLayers) -> Self {
            self.dst_subresource = dst_subresource;
            self
        }
        pub fn dst_offsets(mut self, dst_offsets: [Offset3D; 2]) -> Self {
            self.dst_offsets = dst_offsets;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BufferImageCopy2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer_offset: DeviceSize,
        pub buffer_row_length: u32,
        pub buffer_image_height: u32,
        pub image_subresource: ImageSubresourceLayers,
        pub image_offset: Offset3D,
        pub image_extent: Extent3D,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BufferImageCopy2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_IMAGE_COPY_2;
    }
    impl Default for BufferImageCopy2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                buffer_offset: Default::default(),
                buffer_row_length: Default::default(),
                buffer_image_height: Default::default(),
                image_subresource: Default::default(),
                image_offset: Default::default(),
                image_extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BufferImageCopy2<'a> {
        pub fn buffer_offset(mut self, buffer_offset: DeviceSize) -> Self {
            self.buffer_offset = buffer_offset;
            self
        }
        pub fn buffer_row_length(mut self, buffer_row_length: u32) -> Self {
            self.buffer_row_length = buffer_row_length;
            self
        }
        pub fn buffer_image_height(mut self, buffer_image_height: u32) -> Self {
            self.buffer_image_height = buffer_image_height;
            self
        }
        pub fn image_subresource(mut self, image_subresource: ImageSubresourceLayers) -> Self {
            self.image_subresource = image_subresource;
            self
        }
        pub fn image_offset(mut self, image_offset: Offset3D) -> Self {
            self.image_offset = image_offset;
            self
        }
        pub fn image_extent(mut self, image_extent: Extent3D) -> Self {
            self.image_extent = image_extent;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageResolve2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_subresource: ImageSubresourceLayers,
        pub src_offset: Offset3D,
        pub dst_subresource: ImageSubresourceLayers,
        pub dst_offset: Offset3D,
        pub extent: Extent3D,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageResolve2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_RESOLVE_2;
    }
    impl Default for ImageResolve2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_subresource: Default::default(),
                src_offset: Default::default(),
                dst_subresource: Default::default(),
                dst_offset: Default::default(),
                extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageResolve2<'a> {
        pub fn src_subresource(mut self, src_subresource: ImageSubresourceLayers) -> Self {
            self.src_subresource = src_subresource;
            self
        }
        pub fn src_offset(mut self, src_offset: Offset3D) -> Self {
            self.src_offset = src_offset;
            self
        }
        pub fn dst_subresource(mut self, dst_subresource: ImageSubresourceLayers) -> Self {
            self.dst_subresource = dst_subresource;
            self
        }
        pub fn dst_offset(mut self, dst_offset: Offset3D) -> Self {
            self.dst_offset = dst_offset;
            self
        }
        pub fn extent(mut self, extent: Extent3D) -> Self {
            self.extent = extent;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CopyBufferInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_buffer: Buffer,
        pub dst_buffer: Buffer,
        pub region_count: u32,
        pub p_regions: *const BufferCopy2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CopyBufferInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_BUFFER_INFO_2;
    }
    impl Default for CopyBufferInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_buffer: Default::default(),
                dst_buffer: Default::default(),
                region_count: Default::default(),
                p_regions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CopyBufferInfo2<'a> {
        pub fn src_buffer(mut self, src_buffer: Buffer) -> Self {
            self.src_buffer = src_buffer;
            self
        }
        pub fn dst_buffer(mut self, dst_buffer: Buffer) -> Self {
            self.dst_buffer = dst_buffer;
            self
        }
        pub fn regions(mut self, regions: &'a [BufferCopy2<'a>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CopyImageInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_image: Image,
        pub src_image_layout: ImageLayout,
        pub dst_image: Image,
        pub dst_image_layout: ImageLayout,
        pub region_count: u32,
        pub p_regions: *const ImageCopy2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CopyImageInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_IMAGE_INFO_2;
    }
    impl Default for CopyImageInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_image: Default::default(),
                src_image_layout: Default::default(),
                dst_image: Default::default(),
                dst_image_layout: Default::default(),
                region_count: Default::default(),
                p_regions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CopyImageInfo2<'a> {
        pub fn src_image(mut self, src_image: Image) -> Self {
            self.src_image = src_image;
            self
        }
        pub fn src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
            self.src_image_layout = src_image_layout;
            self
        }
        pub fn dst_image(mut self, dst_image: Image) -> Self {
            self.dst_image = dst_image;
            self
        }
        pub fn dst_image_layout(mut self, dst_image_layout: ImageLayout) -> Self {
            self.dst_image_layout = dst_image_layout;
            self
        }
        pub fn regions(mut self, regions: &'a [ImageCopy2<'a>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BlitImageInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_image: Image,
        pub src_image_layout: ImageLayout,
        pub dst_image: Image,
        pub dst_image_layout: ImageLayout,
        pub region_count: u32,
        pub p_regions: *const ImageBlit2<'a>,
        pub filter: Filter,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BlitImageInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BLIT_IMAGE_INFO_2;
    }
    impl Default for BlitImageInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_image: Default::default(),
                src_image_layout: Default::default(),
                dst_image: Default::default(),
                dst_image_layout: Default::default(),
                region_count: Default::default(),
                p_regions: core::ptr::null(),
                filter: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BlitImageInfo2<'a> {
        pub fn src_image(mut self, src_image: Image) -> Self {
            self.src_image = src_image;
            self
        }
        pub fn src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
            self.src_image_layout = src_image_layout;
            self
        }
        pub fn dst_image(mut self, dst_image: Image) -> Self {
            self.dst_image = dst_image;
            self
        }
        pub fn dst_image_layout(mut self, dst_image_layout: ImageLayout) -> Self {
            self.dst_image_layout = dst_image_layout;
            self
        }
        pub fn regions(mut self, regions: &'a [ImageBlit2<'a>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
        pub fn filter(mut self, filter: Filter) -> Self {
            self.filter = filter;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CopyBufferToImageInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_buffer: Buffer,
        pub dst_image: Image,
        pub dst_image_layout: ImageLayout,
        pub region_count: u32,
        pub p_regions: *const BufferImageCopy2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CopyBufferToImageInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_BUFFER_TO_IMAGE_INFO_2;
    }
    impl Default for CopyBufferToImageInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_buffer: Default::default(),
                dst_image: Default::default(),
                dst_image_layout: Default::default(),
                region_count: Default::default(),
                p_regions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CopyBufferToImageInfo2<'a> {
        pub fn src_buffer(mut self, src_buffer: Buffer) -> Self {
            self.src_buffer = src_buffer;
            self
        }
        pub fn dst_image(mut self, dst_image: Image) -> Self {
            self.dst_image = dst_image;
            self
        }
        pub fn dst_image_layout(mut self, dst_image_layout: ImageLayout) -> Self {
            self.dst_image_layout = dst_image_layout;
            self
        }
        pub fn regions(mut self, regions: &'a [BufferImageCopy2<'a>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CopyImageToBufferInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_image: Image,
        pub src_image_layout: ImageLayout,
        pub dst_buffer: Buffer,
        pub region_count: u32,
        pub p_regions: *const BufferImageCopy2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CopyImageToBufferInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_IMAGE_TO_BUFFER_INFO_2;
    }
    impl Default for CopyImageToBufferInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_image: Default::default(),
                src_image_layout: Default::default(),
                dst_buffer: Default::default(),
                region_count: Default::default(),
                p_regions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CopyImageToBufferInfo2<'a> {
        pub fn src_image(mut self, src_image: Image) -> Self {
            self.src_image = src_image;
            self
        }
        pub fn src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
            self.src_image_layout = src_image_layout;
            self
        }
        pub fn dst_buffer(mut self, dst_buffer: Buffer) -> Self {
            self.dst_buffer = dst_buffer;
            self
        }
        pub fn regions(mut self, regions: &'a [BufferImageCopy2<'a>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ResolveImageInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_image: Image,
        pub src_image_layout: ImageLayout,
        pub dst_image: Image,
        pub dst_image_layout: ImageLayout,
        pub region_count: u32,
        pub p_regions: *const ImageResolve2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ResolveImageInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RESOLVE_IMAGE_INFO_2;
    }
    impl Default for ResolveImageInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_image: Default::default(),
                src_image_layout: Default::default(),
                dst_image: Default::default(),
                dst_image_layout: Default::default(),
                region_count: Default::default(),
                p_regions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ResolveImageInfo2<'a> {
        pub fn src_image(mut self, src_image: Image) -> Self {
            self.src_image = src_image;
            self
        }
        pub fn src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
            self.src_image_layout = src_image_layout;
            self
        }
        pub fn dst_image(mut self, dst_image: Image) -> Self {
            self.dst_image = dst_image;
            self
        }
        pub fn dst_image_layout(mut self, dst_image_layout: ImageLayout) -> Self {
            self.dst_image_layout = dst_image_layout;
            self
        }
        pub fn regions(mut self, regions: &'a [ImageResolve2<'a>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderTerminateInvocationFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_terminate_invocation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderTerminateInvocationFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderTerminateInvocationFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceShaderTerminateInvocationFeatures<'a>
    {
    }
    impl Default for PhysicalDeviceShaderTerminateInvocationFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_terminate_invocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderTerminateInvocationFeatures<'a> {
        pub fn shader_terminate_invocation(mut self, shader_terminate_invocation: Bool32) -> Self {
            self.shader_terminate_invocation = shader_terminate_invocation;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryBarrier2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_stage_mask: PipelineStageFlags2,
        pub src_access_mask: AccessFlags2,
        pub dst_stage_mask: PipelineStageFlags2,
        pub dst_access_mask: AccessFlags2,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryBarrier2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_BARRIER_2;
    }
    unsafe impl<'a> Extends<SubpassDependency2<'a>> for MemoryBarrier2<'a> {}
    impl Default for MemoryBarrier2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_stage_mask: Default::default(),
                src_access_mask: Default::default(),
                dst_stage_mask: Default::default(),
                dst_access_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryBarrier2<'a> {
        pub fn src_stage_mask(mut self, src_stage_mask: PipelineStageFlags2) -> Self {
            self.src_stage_mask = src_stage_mask;
            self
        }
        pub fn src_access_mask(mut self, src_access_mask: AccessFlags2) -> Self {
            self.src_access_mask = src_access_mask;
            self
        }
        pub fn dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags2) -> Self {
            self.dst_stage_mask = dst_stage_mask;
            self
        }
        pub fn dst_access_mask(mut self, dst_access_mask: AccessFlags2) -> Self {
            self.dst_access_mask = dst_access_mask;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageMemoryBarrier2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_stage_mask: PipelineStageFlags2,
        pub src_access_mask: AccessFlags2,
        pub dst_stage_mask: PipelineStageFlags2,
        pub dst_access_mask: AccessFlags2,
        pub old_layout: ImageLayout,
        pub new_layout: ImageLayout,
        pub src_queue_family_index: u32,
        pub dst_queue_family_index: u32,
        pub image: Image,
        pub subresource_range: ImageSubresourceRange,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageMemoryBarrier2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_MEMORY_BARRIER_2;
    }
    impl Default for ImageMemoryBarrier2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_stage_mask: Default::default(),
                src_access_mask: Default::default(),
                dst_stage_mask: Default::default(),
                dst_access_mask: Default::default(),
                old_layout: Default::default(),
                new_layout: Default::default(),
                src_queue_family_index: Default::default(),
                dst_queue_family_index: Default::default(),
                image: Default::default(),
                subresource_range: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageMemoryBarrier2<'a> {
        pub fn src_stage_mask(mut self, src_stage_mask: PipelineStageFlags2) -> Self {
            self.src_stage_mask = src_stage_mask;
            self
        }
        pub fn src_access_mask(mut self, src_access_mask: AccessFlags2) -> Self {
            self.src_access_mask = src_access_mask;
            self
        }
        pub fn dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags2) -> Self {
            self.dst_stage_mask = dst_stage_mask;
            self
        }
        pub fn dst_access_mask(mut self, dst_access_mask: AccessFlags2) -> Self {
            self.dst_access_mask = dst_access_mask;
            self
        }
        pub fn old_layout(mut self, old_layout: ImageLayout) -> Self {
            self.old_layout = old_layout;
            self
        }
        pub fn new_layout(mut self, new_layout: ImageLayout) -> Self {
            self.new_layout = new_layout;
            self
        }
        pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
            self.src_queue_family_index = src_queue_family_index;
            self
        }
        pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
            self.dst_queue_family_index = dst_queue_family_index;
            self
        }
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
        pub fn subresource_range(mut self, subresource_range: ImageSubresourceRange) -> Self {
            self.subresource_range = subresource_range;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BufferMemoryBarrier2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_stage_mask: PipelineStageFlags2,
        pub src_access_mask: AccessFlags2,
        pub dst_stage_mask: PipelineStageFlags2,
        pub dst_access_mask: AccessFlags2,
        pub src_queue_family_index: u32,
        pub dst_queue_family_index: u32,
        pub buffer: Buffer,
        pub offset: DeviceSize,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BufferMemoryBarrier2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_MEMORY_BARRIER_2;
    }
    impl Default for BufferMemoryBarrier2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_stage_mask: Default::default(),
                src_access_mask: Default::default(),
                dst_stage_mask: Default::default(),
                dst_access_mask: Default::default(),
                src_queue_family_index: Default::default(),
                dst_queue_family_index: Default::default(),
                buffer: Default::default(),
                offset: Default::default(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BufferMemoryBarrier2<'a> {
        pub fn src_stage_mask(mut self, src_stage_mask: PipelineStageFlags2) -> Self {
            self.src_stage_mask = src_stage_mask;
            self
        }
        pub fn src_access_mask(mut self, src_access_mask: AccessFlags2) -> Self {
            self.src_access_mask = src_access_mask;
            self
        }
        pub fn dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags2) -> Self {
            self.dst_stage_mask = dst_stage_mask;
            self
        }
        pub fn dst_access_mask(mut self, dst_access_mask: AccessFlags2) -> Self {
            self.dst_access_mask = dst_access_mask;
            self
        }
        pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
            self.src_queue_family_index = src_queue_family_index;
            self
        }
        pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
            self.dst_queue_family_index = dst_queue_family_index;
            self
        }
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DependencyInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub dependency_flags: DependencyFlags,
        pub memory_barrier_count: u32,
        pub p_memory_barriers: *const MemoryBarrier2<'a>,
        pub buffer_memory_barrier_count: u32,
        pub p_buffer_memory_barriers: *const BufferMemoryBarrier2<'a>,
        pub image_memory_barrier_count: u32,
        pub p_image_memory_barriers: *const ImageMemoryBarrier2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DependencyInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEPENDENCY_INFO;
    }
    impl Default for DependencyInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                dependency_flags: Default::default(),
                memory_barrier_count: Default::default(),
                p_memory_barriers: core::ptr::null(),
                buffer_memory_barrier_count: Default::default(),
                p_buffer_memory_barriers: core::ptr::null(),
                image_memory_barrier_count: Default::default(),
                p_image_memory_barriers: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DependencyInfo<'a> {
        pub fn dependency_flags(mut self, dependency_flags: DependencyFlags) -> Self {
            self.dependency_flags = dependency_flags;
            self
        }
        pub fn memory_barriers(mut self, memory_barriers: &'a [MemoryBarrier2<'a>]) -> Self {
            self.memory_barrier_count = memory_barriers.len().try_into().unwrap();
            self.p_memory_barriers = memory_barriers.as_ptr();
            self
        }
        pub fn buffer_memory_barriers(
            mut self,
            buffer_memory_barriers: &'a [BufferMemoryBarrier2<'a>],
        ) -> Self {
            self.buffer_memory_barrier_count = buffer_memory_barriers.len().try_into().unwrap();
            self.p_buffer_memory_barriers = buffer_memory_barriers.as_ptr();
            self
        }
        pub fn image_memory_barriers(
            mut self,
            image_memory_barriers: &'a [ImageMemoryBarrier2<'a>],
        ) -> Self {
            self.image_memory_barrier_count = image_memory_barriers.len().try_into().unwrap();
            self.p_image_memory_barriers = image_memory_barriers.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SemaphoreSubmitInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore: Semaphore,
        pub value: u64,
        pub stage_mask: PipelineStageFlags2,
        pub device_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SemaphoreSubmitInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SEMAPHORE_SUBMIT_INFO;
    }
    impl Default for SemaphoreSubmitInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                semaphore: Default::default(),
                value: Default::default(),
                stage_mask: Default::default(),
                device_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SemaphoreSubmitInfo<'a> {
        pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
            self.semaphore = semaphore;
            self
        }
        pub fn value(mut self, value: u64) -> Self {
            self.value = value;
            self
        }
        pub fn stage_mask(mut self, stage_mask: PipelineStageFlags2) -> Self {
            self.stage_mask = stage_mask;
            self
        }
        pub fn device_index(mut self, device_index: u32) -> Self {
            self.device_index = device_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CommandBufferSubmitInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub command_buffer: CommandBuffer,
        pub device_mask: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CommandBufferSubmitInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COMMAND_BUFFER_SUBMIT_INFO;
    }
    impl Default for CommandBufferSubmitInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                command_buffer: Default::default(),
                device_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CommandBufferSubmitInfo<'a> {
        pub fn command_buffer(mut self, command_buffer: CommandBuffer) -> Self {
            self.command_buffer = command_buffer;
            self
        }
        pub fn device_mask(mut self, device_mask: u32) -> Self {
            self.device_mask = device_mask;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubmitInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: SubmitFlags,
        pub wait_semaphore_info_count: u32,
        pub p_wait_semaphore_infos: *const SemaphoreSubmitInfo<'a>,
        pub command_buffer_info_count: u32,
        pub p_command_buffer_infos: *const CommandBufferSubmitInfo<'a>,
        pub signal_semaphore_info_count: u32,
        pub p_signal_semaphore_infos: *const SemaphoreSubmitInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SubmitInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SUBMIT_INFO_2;
    }
    impl Default for SubmitInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                wait_semaphore_info_count: Default::default(),
                p_wait_semaphore_infos: core::ptr::null(),
                command_buffer_info_count: Default::default(),
                p_command_buffer_infos: core::ptr::null(),
                signal_semaphore_info_count: Default::default(),
                p_signal_semaphore_infos: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SubmitInfo2<'a> {
        pub fn flags(mut self, flags: SubmitFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn wait_semaphore_infos(
            mut self,
            wait_semaphore_infos: &'a [SemaphoreSubmitInfo<'a>],
        ) -> Self {
            self.wait_semaphore_info_count = wait_semaphore_infos.len().try_into().unwrap();
            self.p_wait_semaphore_infos = wait_semaphore_infos.as_ptr();
            self
        }
        pub fn command_buffer_infos(
            mut self,
            command_buffer_infos: &'a [CommandBufferSubmitInfo<'a>],
        ) -> Self {
            self.command_buffer_info_count = command_buffer_infos.len().try_into().unwrap();
            self.p_command_buffer_infos = command_buffer_infos.as_ptr();
            self
        }
        pub fn signal_semaphore_infos(
            mut self,
            signal_semaphore_infos: &'a [SemaphoreSubmitInfo<'a>],
        ) -> Self {
            self.signal_semaphore_info_count = signal_semaphore_infos.len().try_into().unwrap();
            self.p_signal_semaphore_infos = signal_semaphore_infos.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceSynchronization2Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub synchronization2: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSynchronization2Features<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceSynchronization2Features<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceSynchronization2Features<'a> {}
    impl Default for PhysicalDeviceSynchronization2Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                synchronization2: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceSynchronization2Features<'a> {
        pub fn synchronization2(mut self, synchronization2: Bool32) -> Self {
            self.synchronization2 = synchronization2;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderIntegerDotProductFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_integer_dot_product: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderIntegerDotProductFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderIntegerDotProductFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceShaderIntegerDotProductFeatures<'a>
    {
    }
    impl Default for PhysicalDeviceShaderIntegerDotProductFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_integer_dot_product: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderIntegerDotProductFeatures<'a> {
        pub fn shader_integer_dot_product(mut self, shader_integer_dot_product: Bool32) -> Self {
            self.shader_integer_dot_product = shader_integer_dot_product;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderIntegerDotProductProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub integer_dot_product8_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product8_bit_signed_accelerated: Bool32,
        pub integer_dot_product8_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product4x8_bit_packed_unsigned_accelerated: Bool32,
        pub integer_dot_product4x8_bit_packed_signed_accelerated: Bool32,
        pub integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product16_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product16_bit_signed_accelerated: Bool32,
        pub integer_dot_product16_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product32_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product32_bit_signed_accelerated: Bool32,
        pub integer_dot_product32_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product64_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product64_bit_signed_accelerated: Bool32,
        pub integer_dot_product64_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated:
            Bool32,
        pub integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderIntegerDotProductProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceShaderIntegerDotProductProperties<'a>
    {
    }
    impl Default for PhysicalDeviceShaderIntegerDotProductProperties<'_> {
        fn default() -> Self {
            Self {
s_type: Self::STRUCTURE_TYPE
,
p_next: core::ptr::null_mut(),
integer_dot_product8_bit_unsigned_accelerated: Default::default(),
integer_dot_product8_bit_signed_accelerated: Default::default(),
integer_dot_product8_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product4x8_bit_packed_unsigned_accelerated: Default::default(),
integer_dot_product4x8_bit_packed_signed_accelerated: Default::default(),
integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Default::default(),
integer_dot_product16_bit_unsigned_accelerated: Default::default(),
integer_dot_product16_bit_signed_accelerated: Default::default(),
integer_dot_product16_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product32_bit_unsigned_accelerated: Default::default(),
integer_dot_product32_bit_signed_accelerated: Default::default(),
integer_dot_product32_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product64_bit_unsigned_accelerated: Default::default(),
integer_dot_product64_bit_signed_accelerated: Default::default(),
integer_dot_product64_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Default::default(),
_marker: PhantomData
}
        }
    }
    impl<'a> PhysicalDeviceShaderIntegerDotProductProperties<'a> {
        pub fn integer_dot_product8_bit_unsigned_accelerated(
            mut self,
            integer_dot_product8_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product8_bit_unsigned_accelerated =
                integer_dot_product8_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product8_bit_signed_accelerated(
            mut self,
            integer_dot_product8_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product8_bit_signed_accelerated =
                integer_dot_product8_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product8_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product8_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product8_bit_mixed_signedness_accelerated =
                integer_dot_product8_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product4x8_bit_packed_unsigned_accelerated(
            mut self,
            integer_dot_product4x8_bit_packed_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product4x8_bit_packed_unsigned_accelerated =
                integer_dot_product4x8_bit_packed_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product4x8_bit_packed_signed_accelerated(
            mut self,
            integer_dot_product4x8_bit_packed_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product4x8_bit_packed_signed_accelerated =
                integer_dot_product4x8_bit_packed_signed_accelerated;
            self
        }
        pub fn integer_dot_product4x8_bit_packed_mixed_signedness_accelerated(
            mut self,
            integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated =
                integer_dot_product4x8_bit_packed_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product16_bit_unsigned_accelerated(
            mut self,
            integer_dot_product16_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product16_bit_unsigned_accelerated =
                integer_dot_product16_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product16_bit_signed_accelerated(
            mut self,
            integer_dot_product16_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product16_bit_signed_accelerated =
                integer_dot_product16_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product16_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product16_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product16_bit_mixed_signedness_accelerated =
                integer_dot_product16_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product32_bit_unsigned_accelerated(
            mut self,
            integer_dot_product32_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product32_bit_unsigned_accelerated =
                integer_dot_product32_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product32_bit_signed_accelerated(
            mut self,
            integer_dot_product32_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product32_bit_signed_accelerated =
                integer_dot_product32_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product32_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product32_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product32_bit_mixed_signedness_accelerated =
                integer_dot_product32_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product64_bit_unsigned_accelerated(
            mut self,
            integer_dot_product64_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product64_bit_unsigned_accelerated =
                integer_dot_product64_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product64_bit_signed_accelerated(
            mut self,
            integer_dot_product64_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product64_bit_signed_accelerated =
                integer_dot_product64_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product64_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product64_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product64_bit_mixed_signedness_accelerated =
                integer_dot_product64_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating8_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating8_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating8_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated =
                integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated =
                integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated = integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating16_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating16_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating16_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating32_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating32_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating32_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating64_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating64_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating64_bit_signed_accelerated;
            self
        }
        pub fn integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Bool32,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FormatProperties3<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub linear_tiling_features: FormatFeatureFlags2,
        pub optimal_tiling_features: FormatFeatureFlags2,
        pub buffer_features: FormatFeatureFlags2,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for FormatProperties3<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FORMAT_PROPERTIES_3;
    }
    unsafe impl<'a> Extends<FormatProperties2<'a>> for FormatProperties3<'a> {}
    impl Default for FormatProperties3<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                linear_tiling_features: Default::default(),
                optimal_tiling_features: Default::default(),
                buffer_features: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> FormatProperties3<'a> {
        pub fn linear_tiling_features(
            mut self,
            linear_tiling_features: FormatFeatureFlags2,
        ) -> Self {
            self.linear_tiling_features = linear_tiling_features;
            self
        }
        pub fn optimal_tiling_features(
            mut self,
            optimal_tiling_features: FormatFeatureFlags2,
        ) -> Self {
            self.optimal_tiling_features = optimal_tiling_features;
            self
        }
        pub fn buffer_features(mut self, buffer_features: FormatFeatureFlags2) -> Self {
            self.buffer_features = buffer_features;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineRenderingCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub view_mask: u32,
        pub color_attachment_count: u32,
        pub p_color_attachment_formats: *const Format,
        pub depth_attachment_format: Format,
        pub stencil_attachment_format: Format,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineRenderingCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_RENDERING_CREATE_INFO;
    }
    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>> for PipelineRenderingCreateInfo<'a> {}
    impl Default for PipelineRenderingCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                view_mask: Default::default(),
                color_attachment_count: Default::default(),
                p_color_attachment_formats: core::ptr::null(),
                depth_attachment_format: Default::default(),
                stencil_attachment_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineRenderingCreateInfo<'a> {
        pub fn view_mask(mut self, view_mask: u32) -> Self {
            self.view_mask = view_mask;
            self
        }
        pub fn color_attachment_formats(mut self, color_attachment_formats: &'a [Format]) -> Self {
            self.color_attachment_count = color_attachment_formats.len().try_into().unwrap();
            self.p_color_attachment_formats = color_attachment_formats.as_ptr();
            self
        }
        pub fn depth_attachment_format(mut self, depth_attachment_format: Format) -> Self {
            self.depth_attachment_format = depth_attachment_format;
            self
        }
        pub fn stencil_attachment_format(mut self, stencil_attachment_format: Format) -> Self {
            self.stencil_attachment_format = stencil_attachment_format;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderingInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: RenderingFlags,
        pub render_area: Rect2D,
        pub layer_count: u32,
        pub view_mask: u32,
        pub color_attachment_count: u32,
        pub p_color_attachments: *const RenderingAttachmentInfo<'a>,
        pub p_depth_attachment: *const RenderingAttachmentInfo<'a>,
        pub p_stencil_attachment: *const RenderingAttachmentInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for RenderingInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDERING_INFO;
    }
    impl Default for RenderingInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                render_area: Default::default(),
                layer_count: Default::default(),
                view_mask: Default::default(),
                color_attachment_count: Default::default(),
                p_color_attachments: core::ptr::null(),
                p_depth_attachment: core::ptr::null(),
                p_stencil_attachment: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderingInfo<'a> {
        pub fn flags(mut self, flags: RenderingFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn render_area(mut self, render_area: Rect2D) -> Self {
            self.render_area = render_area;
            self
        }
        pub fn layer_count(mut self, layer_count: u32) -> Self {
            self.layer_count = layer_count;
            self
        }
        pub fn view_mask(mut self, view_mask: u32) -> Self {
            self.view_mask = view_mask;
            self
        }
        pub fn color_attachments(
            mut self,
            color_attachments: &'a [RenderingAttachmentInfo<'a>],
        ) -> Self {
            self.color_attachment_count = color_attachments.len().try_into().unwrap();
            self.p_color_attachments = color_attachments.as_ptr();
            self
        }
        pub fn depth_attachment(
            mut self,
            depth_attachment: &'a RenderingAttachmentInfo<'a>,
        ) -> Self {
            self.p_depth_attachment = depth_attachment;
            self
        }
        pub fn stencil_attachment(
            mut self,
            stencil_attachment: &'a RenderingAttachmentInfo<'a>,
        ) -> Self {
            self.p_stencil_attachment = stencil_attachment;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderingAttachmentInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image_view: ImageView,
        pub image_layout: ImageLayout,
        pub resolve_mode: ResolveModeFlagBits,
        pub resolve_image_view: ImageView,
        pub resolve_image_layout: ImageLayout,
        pub load_op: AttachmentLoadOp,
        pub store_op: AttachmentStoreOp,
        pub clear_value: ClearValue,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for RenderingAttachmentInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDERING_ATTACHMENT_INFO;
    }
    impl Default for RenderingAttachmentInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                image_view: Default::default(),
                image_layout: Default::default(),
                resolve_mode: Default::default(),
                resolve_image_view: Default::default(),
                resolve_image_layout: Default::default(),
                load_op: Default::default(),
                store_op: Default::default(),
                clear_value: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderingAttachmentInfo<'a> {
        pub fn image_view(mut self, image_view: ImageView) -> Self {
            self.image_view = image_view;
            self
        }
        pub fn image_layout(mut self, image_layout: ImageLayout) -> Self {
            self.image_layout = image_layout;
            self
        }
        pub fn resolve_mode(mut self, resolve_mode: ResolveModeFlagBits) -> Self {
            self.resolve_mode = resolve_mode;
            self
        }
        pub fn resolve_image_view(mut self, resolve_image_view: ImageView) -> Self {
            self.resolve_image_view = resolve_image_view;
            self
        }
        pub fn resolve_image_layout(mut self, resolve_image_layout: ImageLayout) -> Self {
            self.resolve_image_layout = resolve_image_layout;
            self
        }
        pub fn load_op(mut self, load_op: AttachmentLoadOp) -> Self {
            self.load_op = load_op;
            self
        }
        pub fn store_op(mut self, store_op: AttachmentStoreOp) -> Self {
            self.store_op = store_op;
            self
        }
        pub fn clear_value(mut self, clear_value: ClearValue) -> Self {
            self.clear_value = clear_value;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDynamicRenderingFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub dynamic_rendering: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDynamicRenderingFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDynamicRenderingFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceDynamicRenderingFeatures<'a> {}
    impl Default for PhysicalDeviceDynamicRenderingFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                dynamic_rendering: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDynamicRenderingFeatures<'a> {
        pub fn dynamic_rendering(mut self, dynamic_rendering: Bool32) -> Self {
            self.dynamic_rendering = dynamic_rendering;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CommandBufferInheritanceRenderingInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: RenderingFlags,
        pub view_mask: u32,
        pub color_attachment_count: u32,
        pub p_color_attachment_formats: *const Format,
        pub depth_attachment_format: Format,
        pub stencil_attachment_format: Format,
        pub rasterization_samples: SampleCountFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CommandBufferInheritanceRenderingInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO;
    }
    unsafe impl<'a> Extends<CommandBufferInheritanceInfo<'a>>
        for CommandBufferInheritanceRenderingInfo<'a>
    {
    }
    impl Default for CommandBufferInheritanceRenderingInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                view_mask: Default::default(),
                color_attachment_count: Default::default(),
                p_color_attachment_formats: core::ptr::null(),
                depth_attachment_format: Default::default(),
                stencil_attachment_format: Default::default(),
                rasterization_samples: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CommandBufferInheritanceRenderingInfo<'a> {
        pub fn flags(mut self, flags: RenderingFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn view_mask(mut self, view_mask: u32) -> Self {
            self.view_mask = view_mask;
            self
        }
        pub fn color_attachment_formats(mut self, color_attachment_formats: &'a [Format]) -> Self {
            self.color_attachment_count = color_attachment_formats.len().try_into().unwrap();
            self.p_color_attachment_formats = color_attachment_formats.as_ptr();
            self
        }
        pub fn depth_attachment_format(mut self, depth_attachment_format: Format) -> Self {
            self.depth_attachment_format = depth_attachment_format;
            self
        }
        pub fn stencil_attachment_format(mut self, stencil_attachment_format: Format) -> Self {
            self.stencil_attachment_format = stencil_attachment_format;
            self
        }
        pub fn rasterization_samples(mut self, rasterization_samples: SampleCountFlagBits) -> Self {
            self.rasterization_samples = rasterization_samples;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PrivateDataSlotCreateFlags(Flags);
    vk_bitflags_wrapped!(PrivateDataSlotCreateFlags, Flags);
    impl PrivateDataSlotCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineCreationFeedbackFlags(Flags);
    vk_bitflags_wrapped!(PipelineCreationFeedbackFlags, Flags);
    impl PipelineCreationFeedbackFlags {
        pub const VALID: Self = Self(PipelineCreationFeedbackFlagBits::VALID.0);
        pub const APPLICATION_PIPELINE_CACHE_HIT: Self =
            Self(PipelineCreationFeedbackFlagBits::APPLICATION_PIPELINE_CACHE_HIT.0);
        pub const BASE_PIPELINE_ACCELERATION: Self =
            Self(PipelineCreationFeedbackFlagBits::BASE_PIPELINE_ACCELERATION.0);
        pub const APPLICATION_PIPELINE_CACHE_HIT_EXT: Self = Self::APPLICATION_PIPELINE_CACHE_HIT;
        pub const BASE_PIPELINE_ACCELERATION_EXT: Self = Self::BASE_PIPELINE_ACCELERATION;
        pub const VALID_EXT: Self = Self::VALID;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineCreationFeedbackFlagBits(u32);
    impl PipelineCreationFeedbackFlagBits {
        pub const VALID: Self = Self(1 << 0);
        pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(1 << 1);
        pub const BASE_PIPELINE_ACCELERATION: Self = Self(1 << 2);
        pub const APPLICATION_PIPELINE_CACHE_HIT_EXT: Self = Self::APPLICATION_PIPELINE_CACHE_HIT;
        pub const BASE_PIPELINE_ACCELERATION_EXT: Self = Self::BASE_PIPELINE_ACCELERATION;
        pub const VALID_EXT: Self = Self::VALID;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AccessFlags2(Flags64);
    vk_bitflags_wrapped!(AccessFlags2, Flags64);
    impl AccessFlags2 {
        pub const INDIRECT_COMMAND_READ: Self = Self(AccessFlagBits2::INDIRECT_COMMAND_READ.0);
        pub const INDEX_READ: Self = Self(AccessFlagBits2::INDEX_READ.0);
        pub const VERTEX_ATTRIBUTE_READ: Self = Self(AccessFlagBits2::VERTEX_ATTRIBUTE_READ.0);
        pub const UNIFORM_READ: Self = Self(AccessFlagBits2::UNIFORM_READ.0);
        pub const INPUT_ATTACHMENT_READ: Self = Self(AccessFlagBits2::INPUT_ATTACHMENT_READ.0);
        pub const SHADER_READ: Self = Self(AccessFlagBits2::SHADER_READ.0);
        pub const SHADER_WRITE: Self = Self(AccessFlagBits2::SHADER_WRITE.0);
        pub const COLOR_ATTACHMENT_READ: Self = Self(AccessFlagBits2::COLOR_ATTACHMENT_READ.0);
        pub const COLOR_ATTACHMENT_WRITE: Self = Self(AccessFlagBits2::COLOR_ATTACHMENT_WRITE.0);
        pub const DEPTH_STENCIL_ATTACHMENT_READ: Self =
            Self(AccessFlagBits2::DEPTH_STENCIL_ATTACHMENT_READ.0);
        pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self =
            Self(AccessFlagBits2::DEPTH_STENCIL_ATTACHMENT_WRITE.0);
        pub const TRANSFER_READ: Self = Self(AccessFlagBits2::TRANSFER_READ.0);
        pub const TRANSFER_WRITE: Self = Self(AccessFlagBits2::TRANSFER_WRITE.0);
        pub const HOST_READ: Self = Self(AccessFlagBits2::HOST_READ.0);
        pub const HOST_WRITE: Self = Self(AccessFlagBits2::HOST_WRITE.0);
        pub const MEMORY_READ: Self = Self(AccessFlagBits2::MEMORY_READ.0);
        pub const MEMORY_WRITE: Self = Self(AccessFlagBits2::MEMORY_WRITE.0);
        pub const COMMAND_PREPROCESS_READ_EXT: Self =
            Self(AccessFlagBits2::COMMAND_PREPROCESS_READ_EXT.0);
        pub const COMMAND_PREPROCESS_WRITE_EXT: Self =
            Self(AccessFlagBits2::COMMAND_PREPROCESS_WRITE_EXT.0);
        pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self =
            Self(AccessFlagBits2::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT.0);
        pub const CONDITIONAL_RENDERING_READ_EXT: Self =
            Self(AccessFlagBits2::CONDITIONAL_RENDERING_READ_EXT.0);
        pub const ACCELERATION_STRUCTURE_READ_KHR: Self =
            Self(AccessFlagBits2::ACCELERATION_STRUCTURE_READ_KHR.0);
        pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self =
            Self(AccessFlagBits2::ACCELERATION_STRUCTURE_WRITE_KHR.0);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self =
            Self(AccessFlagBits2::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR.0);
        pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self =
            Self(AccessFlagBits2::FRAGMENT_DENSITY_MAP_READ_EXT.0);
        pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self =
            Self(AccessFlagBits2::TRANSFORM_FEEDBACK_WRITE_EXT.0);
        pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self =
            Self(AccessFlagBits2::TRANSFORM_FEEDBACK_COUNTER_READ_EXT.0);
        pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self =
            Self(AccessFlagBits2::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT.0);
        pub const SHADER_SAMPLED_READ: Self = Self(AccessFlagBits2::SHADER_SAMPLED_READ.0);
        pub const SHADER_STORAGE_READ: Self = Self(AccessFlagBits2::SHADER_STORAGE_READ.0);
        pub const SHADER_STORAGE_WRITE: Self = Self(AccessFlagBits2::SHADER_STORAGE_WRITE.0);
        pub const VIDEO_DECODE_READ_KHR: Self = Self(AccessFlagBits2::VIDEO_DECODE_READ_KHR.0);
        pub const VIDEO_DECODE_WRITE_KHR: Self = Self(AccessFlagBits2::VIDEO_DECODE_WRITE_KHR.0);
        pub const VIDEO_ENCODE_READ_KHR: Self = Self(AccessFlagBits2::VIDEO_ENCODE_READ_KHR.0);
        pub const VIDEO_ENCODE_WRITE_KHR: Self = Self(AccessFlagBits2::VIDEO_ENCODE_WRITE_KHR.0);
        pub const INVOCATION_MASK_READ_HUAWEI: Self =
            Self(AccessFlagBits2::INVOCATION_MASK_READ_HUAWEI.0);
        pub const SHADER_BINDING_TABLE_READ_KHR: Self =
            Self(AccessFlagBits2::SHADER_BINDING_TABLE_READ_KHR.0);
        pub const DESCRIPTOR_BUFFER_READ_EXT: Self =
            Self(AccessFlagBits2::DESCRIPTOR_BUFFER_READ_EXT.0);
        pub const OPTICAL_FLOW_READ_NV: Self = Self(AccessFlagBits2::OPTICAL_FLOW_READ_NV.0);
        pub const OPTICAL_FLOW_WRITE_NV: Self = Self(AccessFlagBits2::OPTICAL_FLOW_WRITE_NV.0);
        pub const MICROMAP_READ_EXT: Self = Self(AccessFlagBits2::MICROMAP_READ_EXT.0);
        pub const MICROMAP_WRITE_EXT: Self = Self(AccessFlagBits2::MICROMAP_WRITE_EXT.0);
        pub const DATA_GRAPH_READ_ARM: Self = Self(AccessFlagBits2::DATA_GRAPH_READ_ARM.0);
        pub const DATA_GRAPH_WRITE_ARM: Self = Self(AccessFlagBits2::DATA_GRAPH_WRITE_ARM.0);
        pub const SHADER_TILE_ATTACHMENT_READ_QCOM: Self =
            Self(AccessFlagBits2::SHADER_TILE_ATTACHMENT_READ_QCOM.0);
        pub const SHADER_TILE_ATTACHMENT_WRITE_QCOM: Self =
            Self(AccessFlagBits2::SHADER_TILE_ATTACHMENT_WRITE_QCOM.0);
        pub const MEMORY_DECOMPRESSION_READ_EXT: Self =
            Self(AccessFlagBits2::MEMORY_DECOMPRESSION_READ_EXT.0);
        pub const MEMORY_DECOMPRESSION_WRITE_EXT: Self =
            Self(AccessFlagBits2::MEMORY_DECOMPRESSION_WRITE_EXT.0);
        pub const SAMPLER_HEAP_READ_EXT: Self = Self(AccessFlagBits2::SAMPLER_HEAP_READ_EXT.0);
        pub const RESOURCE_HEAP_READ_EXT: Self = Self(AccessFlagBits2::RESOURCE_HEAP_READ_EXT.0);
        pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
        pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;
        pub const COLOR_ATTACHMENT_READ_KHR: Self = Self::COLOR_ATTACHMENT_READ;
        pub const COLOR_ATTACHMENT_WRITE_KHR: Self = Self::COLOR_ATTACHMENT_WRITE;
        pub const COMMAND_PREPROCESS_READ_NV: Self = Self::COMMAND_PREPROCESS_READ_EXT;
        pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self::COMMAND_PREPROCESS_WRITE_EXT;
        pub const DEPTH_STENCIL_ATTACHMENT_READ_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_READ;
        pub const DEPTH_STENCIL_ATTACHMENT_WRITE_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_WRITE;
        pub const HOST_READ_KHR: Self = Self::HOST_READ;
        pub const HOST_WRITE_KHR: Self = Self::HOST_WRITE;
        pub const INDEX_READ_KHR: Self = Self::INDEX_READ;
        pub const INDIRECT_COMMAND_READ_KHR: Self = Self::INDIRECT_COMMAND_READ;
        pub const INPUT_ATTACHMENT_READ_KHR: Self = Self::INPUT_ATTACHMENT_READ;
        pub const MEMORY_READ_KHR: Self = Self::MEMORY_READ;
        pub const MEMORY_WRITE_KHR: Self = Self::MEMORY_WRITE;
        pub const NONE_KHR: Self = Self::NONE;
        pub const SHADER_READ_KHR: Self = Self::SHADER_READ;
        pub const SHADER_SAMPLED_READ_KHR: Self = Self::SHADER_SAMPLED_READ;
        pub const SHADER_STORAGE_READ_KHR: Self = Self::SHADER_STORAGE_READ;
        pub const SHADER_STORAGE_WRITE_KHR: Self = Self::SHADER_STORAGE_WRITE;
        pub const SHADER_WRITE_KHR: Self = Self::SHADER_WRITE;
        pub const SHADING_RATE_IMAGE_READ_NV: Self =
            Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
        pub const TRANSFER_READ_KHR: Self = Self::TRANSFER_READ;
        pub const TRANSFER_WRITE_KHR: Self = Self::TRANSFER_WRITE;
        pub const UNIFORM_READ_KHR: Self = Self::UNIFORM_READ;
        pub const VERTEX_ATTRIBUTE_READ_KHR: Self = Self::VERTEX_ATTRIBUTE_READ;
        pub const NONE: Self = Self(0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct AccessFlagBits2(u64);
    impl AccessFlagBits2 {
        pub const INDIRECT_COMMAND_READ: Self = Self(1 << 0);
        pub const INDEX_READ: Self = Self(1 << 1);
        pub const VERTEX_ATTRIBUTE_READ: Self = Self(1 << 2);
        pub const UNIFORM_READ: Self = Self(1 << 3);
        pub const INPUT_ATTACHMENT_READ: Self = Self(1 << 4);
        pub const SHADER_READ: Self = Self(1 << 5);
        pub const SHADER_WRITE: Self = Self(1 << 6);
        pub const COLOR_ATTACHMENT_READ: Self = Self(1 << 7);
        pub const COLOR_ATTACHMENT_WRITE: Self = Self(1 << 8);
        pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(1 << 9);
        pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1 << 10);
        pub const TRANSFER_READ: Self = Self(1 << 11);
        pub const TRANSFER_WRITE: Self = Self(1 << 12);
        pub const HOST_READ: Self = Self(1 << 13);
        pub const HOST_WRITE: Self = Self(1 << 14);
        pub const MEMORY_READ: Self = Self(1 << 15);
        pub const MEMORY_WRITE: Self = Self(1 << 16);
        pub const COMMAND_PREPROCESS_READ_EXT: Self = Self(1 << 17);
        pub const COMMAND_PREPROCESS_WRITE_EXT: Self = Self(1 << 18);
        pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(1 << 19);
        pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(1 << 20);
        pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(1 << 21);
        pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(1 << 22);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(1 << 23);
        pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(1 << 24);
        pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(1 << 25);
        pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(1 << 26);
        pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(1 << 27);
        pub const SHADER_SAMPLED_READ: Self = Self(1 << 32);
        pub const SHADER_STORAGE_READ: Self = Self(1 << 33);
        pub const SHADER_STORAGE_WRITE: Self = Self(1 << 34);
        pub const VIDEO_DECODE_READ_KHR: Self = Self(1 << 35);
        pub const VIDEO_DECODE_WRITE_KHR: Self = Self(1 << 36);
        pub const VIDEO_ENCODE_READ_KHR: Self = Self(1 << 37);
        pub const VIDEO_ENCODE_WRITE_KHR: Self = Self(1 << 38);
        pub const INVOCATION_MASK_READ_HUAWEI: Self = Self(1 << 39);
        pub const SHADER_BINDING_TABLE_READ_KHR: Self = Self(1 << 40);
        pub const DESCRIPTOR_BUFFER_READ_EXT: Self = Self(1 << 41);
        pub const OPTICAL_FLOW_READ_NV: Self = Self(1 << 42);
        pub const OPTICAL_FLOW_WRITE_NV: Self = Self(1 << 43);
        pub const MICROMAP_READ_EXT: Self = Self(1 << 44);
        pub const MICROMAP_WRITE_EXT: Self = Self(1 << 45);
        pub const DATA_GRAPH_READ_ARM: Self = Self(1 << 47);
        pub const DATA_GRAPH_WRITE_ARM: Self = Self(1 << 48);
        pub const SHADER_TILE_ATTACHMENT_READ_QCOM: Self = Self(1 << 51);
        pub const SHADER_TILE_ATTACHMENT_WRITE_QCOM: Self = Self(1 << 52);
        pub const MEMORY_DECOMPRESSION_READ_EXT: Self = Self(1 << 55);
        pub const MEMORY_DECOMPRESSION_WRITE_EXT: Self = Self(1 << 56);
        pub const SAMPLER_HEAP_READ_EXT: Self = Self(1 << 57);
        pub const RESOURCE_HEAP_READ_EXT: Self = Self(1 << 58);
        pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
        pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;
        pub const COLOR_ATTACHMENT_READ_KHR: Self = Self::COLOR_ATTACHMENT_READ;
        pub const COLOR_ATTACHMENT_WRITE_KHR: Self = Self::COLOR_ATTACHMENT_WRITE;
        pub const COMMAND_PREPROCESS_READ_NV: Self = Self::COMMAND_PREPROCESS_READ_EXT;
        pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self::COMMAND_PREPROCESS_WRITE_EXT;
        pub const DEPTH_STENCIL_ATTACHMENT_READ_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_READ;
        pub const DEPTH_STENCIL_ATTACHMENT_WRITE_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_WRITE;
        pub const HOST_READ_KHR: Self = Self::HOST_READ;
        pub const HOST_WRITE_KHR: Self = Self::HOST_WRITE;
        pub const INDEX_READ_KHR: Self = Self::INDEX_READ;
        pub const INDIRECT_COMMAND_READ_KHR: Self = Self::INDIRECT_COMMAND_READ;
        pub const INPUT_ATTACHMENT_READ_KHR: Self = Self::INPUT_ATTACHMENT_READ;
        pub const MEMORY_READ_KHR: Self = Self::MEMORY_READ;
        pub const MEMORY_WRITE_KHR: Self = Self::MEMORY_WRITE;
        pub const SHADER_READ_KHR: Self = Self::SHADER_READ;
        pub const SHADER_SAMPLED_READ_KHR: Self = Self::SHADER_SAMPLED_READ;
        pub const SHADER_STORAGE_READ_KHR: Self = Self::SHADER_STORAGE_READ;
        pub const SHADER_STORAGE_WRITE_KHR: Self = Self::SHADER_STORAGE_WRITE;
        pub const SHADER_WRITE_KHR: Self = Self::SHADER_WRITE;
        pub const SHADING_RATE_IMAGE_READ_NV: Self =
            Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
        pub const TRANSFER_READ_KHR: Self = Self::TRANSFER_READ;
        pub const TRANSFER_WRITE_KHR: Self = Self::TRANSFER_WRITE;
        pub const UNIFORM_READ_KHR: Self = Self::UNIFORM_READ;
        pub const VERTEX_ATTRIBUTE_READ_KHR: Self = Self::VERTEX_ATTRIBUTE_READ;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineStageFlags2(Flags64);
    vk_bitflags_wrapped!(PipelineStageFlags2, Flags64);
    impl PipelineStageFlags2 {
        pub const TOP_OF_PIPE: Self = Self(PipelineStageFlagBits2::TOP_OF_PIPE.0);
        pub const DRAW_INDIRECT: Self = Self(PipelineStageFlagBits2::DRAW_INDIRECT.0);
        pub const VERTEX_INPUT: Self = Self(PipelineStageFlagBits2::VERTEX_INPUT.0);
        pub const VERTEX_SHADER: Self = Self(PipelineStageFlagBits2::VERTEX_SHADER.0);
        pub const TESSELLATION_CONTROL_SHADER: Self =
            Self(PipelineStageFlagBits2::TESSELLATION_CONTROL_SHADER.0);
        pub const TESSELLATION_EVALUATION_SHADER: Self =
            Self(PipelineStageFlagBits2::TESSELLATION_EVALUATION_SHADER.0);
        pub const GEOMETRY_SHADER: Self = Self(PipelineStageFlagBits2::GEOMETRY_SHADER.0);
        pub const FRAGMENT_SHADER: Self = Self(PipelineStageFlagBits2::FRAGMENT_SHADER.0);
        pub const EARLY_FRAGMENT_TESTS: Self = Self(PipelineStageFlagBits2::EARLY_FRAGMENT_TESTS.0);
        pub const LATE_FRAGMENT_TESTS: Self = Self(PipelineStageFlagBits2::LATE_FRAGMENT_TESTS.0);
        pub const COLOR_ATTACHMENT_OUTPUT: Self =
            Self(PipelineStageFlagBits2::COLOR_ATTACHMENT_OUTPUT.0);
        pub const COMPUTE_SHADER: Self = Self(PipelineStageFlagBits2::COMPUTE_SHADER.0);
        pub const ALL_TRANSFER: Self = Self(PipelineStageFlagBits2::ALL_TRANSFER.0);
        pub const BOTTOM_OF_PIPE: Self = Self(PipelineStageFlagBits2::BOTTOM_OF_PIPE.0);
        pub const HOST: Self = Self(PipelineStageFlagBits2::HOST.0);
        pub const ALL_GRAPHICS: Self = Self(PipelineStageFlagBits2::ALL_GRAPHICS.0);
        pub const ALL_COMMANDS: Self = Self(PipelineStageFlagBits2::ALL_COMMANDS.0);
        pub const COMMAND_PREPROCESS_EXT: Self =
            Self(PipelineStageFlagBits2::COMMAND_PREPROCESS_EXT.0);
        pub const CONDITIONAL_RENDERING_EXT: Self =
            Self(PipelineStageFlagBits2::CONDITIONAL_RENDERING_EXT.0);
        pub const TASK_SHADER_EXT: Self = Self(PipelineStageFlagBits2::TASK_SHADER_EXT.0);
        pub const MESH_SHADER_EXT: Self = Self(PipelineStageFlagBits2::MESH_SHADER_EXT.0);
        pub const RAY_TRACING_SHADER_KHR: Self =
            Self(PipelineStageFlagBits2::RAY_TRACING_SHADER_KHR.0);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
            Self(PipelineStageFlagBits2::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0);
        pub const FRAGMENT_DENSITY_PROCESS_EXT: Self =
            Self(PipelineStageFlagBits2::FRAGMENT_DENSITY_PROCESS_EXT.0);
        pub const TRANSFORM_FEEDBACK_EXT: Self =
            Self(PipelineStageFlagBits2::TRANSFORM_FEEDBACK_EXT.0);
        pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self =
            Self(PipelineStageFlagBits2::ACCELERATION_STRUCTURE_BUILD_KHR.0);
        pub const VIDEO_DECODE_KHR: Self = Self(PipelineStageFlagBits2::VIDEO_DECODE_KHR.0);
        pub const VIDEO_ENCODE_KHR: Self = Self(PipelineStageFlagBits2::VIDEO_ENCODE_KHR.0);
        pub const ACCELERATION_STRUCTURE_COPY_KHR: Self =
            Self(PipelineStageFlagBits2::ACCELERATION_STRUCTURE_COPY_KHR.0);
        pub const OPTICAL_FLOW_NV: Self = Self(PipelineStageFlagBits2::OPTICAL_FLOW_NV.0);
        pub const MICROMAP_BUILD_EXT: Self = Self(PipelineStageFlagBits2::MICROMAP_BUILD_EXT.0);
        pub const COPY: Self = Self(PipelineStageFlagBits2::COPY.0);
        pub const RESOLVE: Self = Self(PipelineStageFlagBits2::RESOLVE.0);
        pub const BLIT: Self = Self(PipelineStageFlagBits2::BLIT.0);
        pub const CLEAR: Self = Self(PipelineStageFlagBits2::CLEAR.0);
        pub const INDEX_INPUT: Self = Self(PipelineStageFlagBits2::INDEX_INPUT.0);
        pub const VERTEX_ATTRIBUTE_INPUT: Self =
            Self(PipelineStageFlagBits2::VERTEX_ATTRIBUTE_INPUT.0);
        pub const PRE_RASTERIZATION_SHADERS: Self =
            Self(PipelineStageFlagBits2::PRE_RASTERIZATION_SHADERS.0);
        pub const SUBPASS_SHADER_HUAWEI: Self =
            Self(PipelineStageFlagBits2::SUBPASS_SHADER_HUAWEI.0);
        pub const INVOCATION_MASK_HUAWEI: Self =
            Self(PipelineStageFlagBits2::INVOCATION_MASK_HUAWEI.0);
        pub const CLUSTER_CULLING_SHADER_HUAWEI: Self =
            Self(PipelineStageFlagBits2::CLUSTER_CULLING_SHADER_HUAWEI.0);
        pub const DATA_GRAPH_ARM: Self = Self(PipelineStageFlagBits2::DATA_GRAPH_ARM.0);
        pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_NV: Self =
            Self(PipelineStageFlagBits2::CONVERT_COOPERATIVE_VECTOR_MATRIX_NV.0);
        pub const MEMORY_DECOMPRESSION_EXT: Self =
            Self(PipelineStageFlagBits2::MEMORY_DECOMPRESSION_EXT.0);
        pub const COPY_INDIRECT_KHR: Self = Self(PipelineStageFlagBits2::COPY_INDIRECT_KHR.0);
        pub const TRANSFER: Self = Self::ALL_TRANSFER;
        pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
        pub const ALL_COMMANDS_KHR: Self = Self::ALL_COMMANDS;
        pub const ALL_GRAPHICS_KHR: Self = Self::ALL_GRAPHICS;
        pub const ALL_TRANSFER_KHR: Self = Self::ALL_TRANSFER;
        pub const BLIT_KHR: Self = Self::BLIT;
        pub const BOTTOM_OF_PIPE_KHR: Self = Self::BOTTOM_OF_PIPE;
        pub const CLEAR_KHR: Self = Self::CLEAR;
        pub const COLOR_ATTACHMENT_OUTPUT_KHR: Self = Self::COLOR_ATTACHMENT_OUTPUT;
        pub const COMMAND_PREPROCESS_NV: Self = Self::COMMAND_PREPROCESS_EXT;
        pub const COMPUTE_SHADER_KHR: Self = Self::COMPUTE_SHADER;
        pub const COPY_KHR: Self = Self::COPY;
        pub const DRAW_INDIRECT_KHR: Self = Self::DRAW_INDIRECT;
        pub const EARLY_FRAGMENT_TESTS_KHR: Self = Self::EARLY_FRAGMENT_TESTS;
        pub const FRAGMENT_SHADER_KHR: Self = Self::FRAGMENT_SHADER;
        pub const GEOMETRY_SHADER_KHR: Self = Self::GEOMETRY_SHADER;
        pub const HOST_KHR: Self = Self::HOST;
        pub const INDEX_INPUT_KHR: Self = Self::INDEX_INPUT;
        pub const LATE_FRAGMENT_TESTS_KHR: Self = Self::LATE_FRAGMENT_TESTS;
        pub const MESH_SHADER_NV: Self = Self::MESH_SHADER_EXT;
        pub const NONE_KHR: Self = Self::NONE;
        pub const PRE_RASTERIZATION_SHADERS_KHR: Self = Self::PRE_RASTERIZATION_SHADERS;
        pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
        pub const RESOLVE_KHR: Self = Self::RESOLVE;
        pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
        pub const SUBPASS_SHADING_HUAWEI: Self = Self::SUBPASS_SHADER_HUAWEI;
        pub const TASK_SHADER_NV: Self = Self::TASK_SHADER_EXT;
        pub const TESSELLATION_CONTROL_SHADER_KHR: Self = Self::TESSELLATION_CONTROL_SHADER;
        pub const TESSELLATION_EVALUATION_SHADER_KHR: Self = Self::TESSELLATION_EVALUATION_SHADER;
        pub const TOP_OF_PIPE_KHR: Self = Self::TOP_OF_PIPE;
        pub const TRANSFER_KHR: Self = Self::ALL_TRANSFER;
        pub const VERTEX_ATTRIBUTE_INPUT_KHR: Self = Self::VERTEX_ATTRIBUTE_INPUT;
        pub const VERTEX_INPUT_KHR: Self = Self::VERTEX_INPUT;
        pub const VERTEX_SHADER_KHR: Self = Self::VERTEX_SHADER;
        pub const NONE: Self = Self(0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineStageFlagBits2(u64);
    impl PipelineStageFlagBits2 {
        pub const TOP_OF_PIPE: Self = Self(1 << 0);
        pub const DRAW_INDIRECT: Self = Self(1 << 1);
        pub const VERTEX_INPUT: Self = Self(1 << 2);
        pub const VERTEX_SHADER: Self = Self(1 << 3);
        pub const TESSELLATION_CONTROL_SHADER: Self = Self(1 << 4);
        pub const TESSELLATION_EVALUATION_SHADER: Self = Self(1 << 5);
        pub const GEOMETRY_SHADER: Self = Self(1 << 6);
        pub const FRAGMENT_SHADER: Self = Self(1 << 7);
        pub const EARLY_FRAGMENT_TESTS: Self = Self(1 << 8);
        pub const LATE_FRAGMENT_TESTS: Self = Self(1 << 9);
        pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(1 << 10);
        pub const COMPUTE_SHADER: Self = Self(1 << 11);
        pub const ALL_TRANSFER: Self = Self(1 << 12);
        pub const BOTTOM_OF_PIPE: Self = Self(1 << 13);
        pub const HOST: Self = Self(1 << 14);
        pub const ALL_GRAPHICS: Self = Self(1 << 15);
        pub const ALL_COMMANDS: Self = Self(1 << 16);
        pub const COMMAND_PREPROCESS_EXT: Self = Self(1 << 17);
        pub const CONDITIONAL_RENDERING_EXT: Self = Self(1 << 18);
        pub const TASK_SHADER_EXT: Self = Self(1 << 19);
        pub const MESH_SHADER_EXT: Self = Self(1 << 20);
        pub const RAY_TRACING_SHADER_KHR: Self = Self(1 << 21);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 22);
        pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(1 << 23);
        pub const TRANSFORM_FEEDBACK_EXT: Self = Self(1 << 24);
        pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(1 << 25);
        pub const VIDEO_DECODE_KHR: Self = Self(1 << 26);
        pub const VIDEO_ENCODE_KHR: Self = Self(1 << 27);
        pub const ACCELERATION_STRUCTURE_COPY_KHR: Self = Self(1 << 28);
        pub const OPTICAL_FLOW_NV: Self = Self(1 << 29);
        pub const MICROMAP_BUILD_EXT: Self = Self(1 << 30);
        pub const COPY: Self = Self(1 << 32);
        pub const RESOLVE: Self = Self(1 << 33);
        pub const BLIT: Self = Self(1 << 34);
        pub const CLEAR: Self = Self(1 << 35);
        pub const INDEX_INPUT: Self = Self(1 << 36);
        pub const VERTEX_ATTRIBUTE_INPUT: Self = Self(1 << 37);
        pub const PRE_RASTERIZATION_SHADERS: Self = Self(1 << 38);
        pub const SUBPASS_SHADER_HUAWEI: Self = Self(1 << 39);
        pub const INVOCATION_MASK_HUAWEI: Self = Self(1 << 40);
        pub const CLUSTER_CULLING_SHADER_HUAWEI: Self = Self(1 << 41);
        pub const DATA_GRAPH_ARM: Self = Self(1 << 42);
        pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_NV: Self = Self(1 << 44);
        pub const MEMORY_DECOMPRESSION_EXT: Self = Self(1 << 45);
        pub const COPY_INDIRECT_KHR: Self = Self(1 << 46);
        pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
        pub const ALL_COMMANDS_KHR: Self = Self::ALL_COMMANDS;
        pub const ALL_GRAPHICS_KHR: Self = Self::ALL_GRAPHICS;
        pub const ALL_TRANSFER_KHR: Self = Self::ALL_TRANSFER;
        pub const BLIT_KHR: Self = Self::BLIT;
        pub const BOTTOM_OF_PIPE_KHR: Self = Self::BOTTOM_OF_PIPE;
        pub const CLEAR_KHR: Self = Self::CLEAR;
        pub const COLOR_ATTACHMENT_OUTPUT_KHR: Self = Self::COLOR_ATTACHMENT_OUTPUT;
        pub const COMMAND_PREPROCESS_NV: Self = Self::COMMAND_PREPROCESS_EXT;
        pub const COMPUTE_SHADER_KHR: Self = Self::COMPUTE_SHADER;
        pub const COPY_KHR: Self = Self::COPY;
        pub const DRAW_INDIRECT_KHR: Self = Self::DRAW_INDIRECT;
        pub const EARLY_FRAGMENT_TESTS_KHR: Self = Self::EARLY_FRAGMENT_TESTS;
        pub const FRAGMENT_SHADER_KHR: Self = Self::FRAGMENT_SHADER;
        pub const GEOMETRY_SHADER_KHR: Self = Self::GEOMETRY_SHADER;
        pub const HOST_KHR: Self = Self::HOST;
        pub const INDEX_INPUT_KHR: Self = Self::INDEX_INPUT;
        pub const LATE_FRAGMENT_TESTS_KHR: Self = Self::LATE_FRAGMENT_TESTS;
        pub const MESH_SHADER_NV: Self = Self::MESH_SHADER_EXT;
        pub const PRE_RASTERIZATION_SHADERS_KHR: Self = Self::PRE_RASTERIZATION_SHADERS;
        pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
        pub const RESOLVE_KHR: Self = Self::RESOLVE;
        pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
        pub const SUBPASS_SHADING_HUAWEI: Self = Self::SUBPASS_SHADER_HUAWEI;
        pub const TASK_SHADER_NV: Self = Self::TASK_SHADER_EXT;
        pub const TESSELLATION_CONTROL_SHADER_KHR: Self = Self::TESSELLATION_CONTROL_SHADER;
        pub const TESSELLATION_EVALUATION_SHADER_KHR: Self = Self::TESSELLATION_EVALUATION_SHADER;
        pub const TOP_OF_PIPE_KHR: Self = Self::TOP_OF_PIPE;
        pub const TRANSFER_KHR: Self = Self::ALL_TRANSFER;
        pub const VERTEX_ATTRIBUTE_INPUT_KHR: Self = Self::VERTEX_ATTRIBUTE_INPUT;
        pub const VERTEX_INPUT_KHR: Self = Self::VERTEX_INPUT;
        pub const VERTEX_SHADER_KHR: Self = Self::VERTEX_SHADER;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FormatFeatureFlags2(Flags64);
    vk_bitflags_wrapped!(FormatFeatureFlags2, Flags64);
    impl FormatFeatureFlags2 {
        pub const SAMPLED_IMAGE: Self = Self(FormatFeatureFlagBits2::SAMPLED_IMAGE.0);
        pub const STORAGE_IMAGE: Self = Self(FormatFeatureFlagBits2::STORAGE_IMAGE.0);
        pub const STORAGE_IMAGE_ATOMIC: Self = Self(FormatFeatureFlagBits2::STORAGE_IMAGE_ATOMIC.0);
        pub const UNIFORM_TEXEL_BUFFER: Self = Self(FormatFeatureFlagBits2::UNIFORM_TEXEL_BUFFER.0);
        pub const STORAGE_TEXEL_BUFFER: Self = Self(FormatFeatureFlagBits2::STORAGE_TEXEL_BUFFER.0);
        pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self =
            Self(FormatFeatureFlagBits2::STORAGE_TEXEL_BUFFER_ATOMIC.0);
        pub const VERTEX_BUFFER: Self = Self(FormatFeatureFlagBits2::VERTEX_BUFFER.0);
        pub const COLOR_ATTACHMENT: Self = Self(FormatFeatureFlagBits2::COLOR_ATTACHMENT.0);
        pub const COLOR_ATTACHMENT_BLEND: Self =
            Self(FormatFeatureFlagBits2::COLOR_ATTACHMENT_BLEND.0);
        pub const DEPTH_STENCIL_ATTACHMENT: Self =
            Self(FormatFeatureFlagBits2::DEPTH_STENCIL_ATTACHMENT.0);
        pub const BLIT_SRC: Self = Self(FormatFeatureFlagBits2::BLIT_SRC.0);
        pub const BLIT_DST: Self = Self(FormatFeatureFlagBits2::BLIT_DST.0);
        pub const SAMPLED_IMAGE_FILTER_LINEAR: Self =
            Self(FormatFeatureFlagBits2::SAMPLED_IMAGE_FILTER_LINEAR.0);
        pub const SAMPLED_IMAGE_FILTER_CUBIC: Self =
            Self(FormatFeatureFlagBits2::SAMPLED_IMAGE_FILTER_CUBIC.0);
        pub const TRANSFER_SRC: Self = Self(FormatFeatureFlagBits2::TRANSFER_SRC.0);
        pub const TRANSFER_DST: Self = Self(FormatFeatureFlagBits2::TRANSFER_DST.0);
        pub const SAMPLED_IMAGE_FILTER_MINMAX: Self =
            Self(FormatFeatureFlagBits2::SAMPLED_IMAGE_FILTER_MINMAX.0);
        pub const MIDPOINT_CHROMA_SAMPLES: Self =
            Self(FormatFeatureFlagBits2::MIDPOINT_CHROMA_SAMPLES.0);
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self =
            Self(FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER.0);
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self(
            FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER.0,
        );
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self = Self(
            FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT.0,
        );
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self = Self(FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE.0);
        pub const DISJOINT: Self = Self(FormatFeatureFlagBits2::DISJOINT.0);
        pub const COSITED_CHROMA_SAMPLES: Self =
            Self(FormatFeatureFlagBits2::COSITED_CHROMA_SAMPLES.0);
        pub const FRAGMENT_DENSITY_MAP_EXT: Self =
            Self(FormatFeatureFlagBits2::FRAGMENT_DENSITY_MAP_EXT.0);
        pub const VIDEO_DECODE_OUTPUT_KHR: Self =
            Self(FormatFeatureFlagBits2::VIDEO_DECODE_OUTPUT_KHR.0);
        pub const VIDEO_DECODE_DPB_KHR: Self = Self(FormatFeatureFlagBits2::VIDEO_DECODE_DPB_KHR.0);
        pub const VIDEO_ENCODE_INPUT_KHR: Self =
            Self(FormatFeatureFlagBits2::VIDEO_ENCODE_INPUT_KHR.0);
        pub const VIDEO_ENCODE_DPB_KHR: Self = Self(FormatFeatureFlagBits2::VIDEO_ENCODE_DPB_KHR.0);
        pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self =
            Self(FormatFeatureFlagBits2::ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR.0);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
            Self(FormatFeatureFlagBits2::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0);
        pub const STORAGE_READ_WITHOUT_FORMAT: Self =
            Self(FormatFeatureFlagBits2::STORAGE_READ_WITHOUT_FORMAT.0);
        pub const STORAGE_WRITE_WITHOUT_FORMAT: Self =
            Self(FormatFeatureFlagBits2::STORAGE_WRITE_WITHOUT_FORMAT.0);
        pub const SAMPLED_IMAGE_DEPTH_COMPARISON: Self =
            Self(FormatFeatureFlagBits2::SAMPLED_IMAGE_DEPTH_COMPARISON.0);
        pub const WEIGHT_IMAGE_QCOM: Self = Self(FormatFeatureFlagBits2::WEIGHT_IMAGE_QCOM.0);
        pub const WEIGHT_SAMPLED_IMAGE_QCOM: Self =
            Self(FormatFeatureFlagBits2::WEIGHT_SAMPLED_IMAGE_QCOM.0);
        pub const BLOCK_MATCHING_QCOM: Self = Self(FormatFeatureFlagBits2::BLOCK_MATCHING_QCOM.0);
        pub const BOX_FILTER_SAMPLED_QCOM: Self =
            Self(FormatFeatureFlagBits2::BOX_FILTER_SAMPLED_QCOM.0);
        pub const LINEAR_COLOR_ATTACHMENT_NV: Self =
            Self(FormatFeatureFlagBits2::LINEAR_COLOR_ATTACHMENT_NV.0);
        pub const TENSOR_SHADER_ARM: Self = Self(FormatFeatureFlagBits2::TENSOR_SHADER_ARM.0);
        pub const OPTICAL_FLOW_IMAGE_NV: Self =
            Self(FormatFeatureFlagBits2::OPTICAL_FLOW_IMAGE_NV.0);
        pub const OPTICAL_FLOW_VECTOR_NV: Self =
            Self(FormatFeatureFlagBits2::OPTICAL_FLOW_VECTOR_NV.0);
        pub const OPTICAL_FLOW_COST_NV: Self = Self(FormatFeatureFlagBits2::OPTICAL_FLOW_COST_NV.0);
        pub const TENSOR_IMAGE_ALIASING_ARM: Self =
            Self(FormatFeatureFlagBits2::TENSOR_IMAGE_ALIASING_ARM.0);
        pub const HOST_IMAGE_TRANSFER: Self = Self(FormatFeatureFlagBits2::HOST_IMAGE_TRANSFER.0);
        pub const TENSOR_DATA_GRAPH_ARM: Self =
            Self(FormatFeatureFlagBits2::TENSOR_DATA_GRAPH_ARM.0);
        pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self =
            Self(FormatFeatureFlagBits2::VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR.0);
        pub const VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self =
            Self(FormatFeatureFlagBits2::VIDEO_ENCODE_EMPHASIS_MAP_KHR.0);
        pub const ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV: Self =
            Self(FormatFeatureFlagBits2::ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV.0);
        pub const DEPTH_COPY_ON_COMPUTE_QUEUE_KHR: Self =
            Self(FormatFeatureFlagBits2::DEPTH_COPY_ON_COMPUTE_QUEUE_KHR.0);
        pub const DEPTH_COPY_ON_TRANSFER_QUEUE_KHR: Self =
            Self(FormatFeatureFlagBits2::DEPTH_COPY_ON_TRANSFER_QUEUE_KHR.0);
        pub const STENCIL_COPY_ON_COMPUTE_QUEUE_KHR: Self =
            Self(FormatFeatureFlagBits2::STENCIL_COPY_ON_COMPUTE_QUEUE_KHR.0);
        pub const STENCIL_COPY_ON_TRANSFER_QUEUE_KHR: Self =
            Self(FormatFeatureFlagBits2::STENCIL_COPY_ON_TRANSFER_QUEUE_KHR.0);
        pub const COPY_IMAGE_INDIRECT_DST_KHR: Self =
            Self(FormatFeatureFlagBits2::COPY_IMAGE_INDIRECT_DST_KHR.0);
        pub const BLIT_DST_KHR: Self = Self::BLIT_DST;
        pub const BLIT_SRC_KHR: Self = Self::BLIT_SRC;
        pub const COLOR_ATTACHMENT_KHR: Self = Self::COLOR_ATTACHMENT;
        pub const COLOR_ATTACHMENT_BLEND_KHR: Self = Self::COLOR_ATTACHMENT_BLEND;
        pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self::COSITED_CHROMA_SAMPLES;
        pub const DEPTH_STENCIL_ATTACHMENT_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT;
        pub const DISJOINT_KHR: Self = Self::DISJOINT;
        pub const HOST_IMAGE_TRANSFER_EXT: Self = Self::HOST_IMAGE_TRANSFER;
        pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES;
        pub const SAMPLED_IMAGE_KHR: Self = Self::SAMPLED_IMAGE;
        pub const SAMPLED_IMAGE_DEPTH_COMPARISON_KHR: Self = Self::SAMPLED_IMAGE_DEPTH_COMPARISON;
        pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC;
        pub const SAMPLED_IMAGE_FILTER_LINEAR_KHR: Self = Self::SAMPLED_IMAGE_FILTER_LINEAR;
        pub const SAMPLED_IMAGE_FILTER_MINMAX_KHR: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR:
            Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
        pub const STORAGE_IMAGE_ATOMIC_KHR: Self = Self::STORAGE_IMAGE_ATOMIC;
        pub const STORAGE_IMAGE_KHR: Self = Self::STORAGE_IMAGE;
        pub const STORAGE_READ_WITHOUT_FORMAT_KHR: Self = Self::STORAGE_READ_WITHOUT_FORMAT;
        pub const STORAGE_TEXEL_BUFFER_ATOMIC_KHR: Self = Self::STORAGE_TEXEL_BUFFER_ATOMIC;
        pub const STORAGE_TEXEL_BUFFER_KHR: Self = Self::STORAGE_TEXEL_BUFFER;
        pub const STORAGE_WRITE_WITHOUT_FORMAT_KHR: Self = Self::STORAGE_WRITE_WITHOUT_FORMAT;
        pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
        pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
        pub const UNIFORM_TEXEL_BUFFER_KHR: Self = Self::UNIFORM_TEXEL_BUFFER;
        pub const VERTEX_BUFFER_KHR: Self = Self::VERTEX_BUFFER;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct FormatFeatureFlagBits2(u64);
    impl FormatFeatureFlagBits2 {
        pub const SAMPLED_IMAGE: Self = Self(1 << 0);
        pub const STORAGE_IMAGE: Self = Self(1 << 1);
        pub const STORAGE_IMAGE_ATOMIC: Self = Self(1 << 2);
        pub const UNIFORM_TEXEL_BUFFER: Self = Self(1 << 3);
        pub const STORAGE_TEXEL_BUFFER: Self = Self(1 << 4);
        pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(1 << 5);
        pub const VERTEX_BUFFER: Self = Self(1 << 6);
        pub const COLOR_ATTACHMENT: Self = Self(1 << 7);
        pub const COLOR_ATTACHMENT_BLEND: Self = Self(1 << 8);
        pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(1 << 9);
        pub const BLIT_SRC: Self = Self(1 << 10);
        pub const BLIT_DST: Self = Self(1 << 11);
        pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(1 << 12);
        pub const SAMPLED_IMAGE_FILTER_CUBIC: Self = Self(1 << 13);
        pub const TRANSFER_SRC: Self = Self(1 << 14);
        pub const TRANSFER_DST: Self = Self(1 << 15);
        pub const SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(1 << 16);
        pub const MIDPOINT_CHROMA_SAMPLES: Self = Self(1 << 17);
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(1 << 18);
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self =
            Self(1 << 19);
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self =
            Self(1 << 20);
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
            Self(1 << 21);
        pub const DISJOINT: Self = Self(1 << 22);
        pub const COSITED_CHROMA_SAMPLES: Self = Self(1 << 23);
        pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(1 << 24);
        pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(1 << 25);
        pub const VIDEO_DECODE_DPB_KHR: Self = Self(1 << 26);
        pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(1 << 27);
        pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1 << 28);
        pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(1 << 29);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 30);
        pub const STORAGE_READ_WITHOUT_FORMAT: Self = Self(1 << 31);
        pub const STORAGE_WRITE_WITHOUT_FORMAT: Self = Self(1 << 32);
        pub const SAMPLED_IMAGE_DEPTH_COMPARISON: Self = Self(1 << 33);
        pub const WEIGHT_IMAGE_QCOM: Self = Self(1 << 34);
        pub const WEIGHT_SAMPLED_IMAGE_QCOM: Self = Self(1 << 35);
        pub const BLOCK_MATCHING_QCOM: Self = Self(1 << 36);
        pub const BOX_FILTER_SAMPLED_QCOM: Self = Self(1 << 37);
        pub const LINEAR_COLOR_ATTACHMENT_NV: Self = Self(1 << 38);
        pub const TENSOR_SHADER_ARM: Self = Self(1 << 39);
        pub const OPTICAL_FLOW_IMAGE_NV: Self = Self(1 << 40);
        pub const OPTICAL_FLOW_VECTOR_NV: Self = Self(1 << 41);
        pub const OPTICAL_FLOW_COST_NV: Self = Self(1 << 42);
        pub const TENSOR_IMAGE_ALIASING_ARM: Self = Self(1 << 43);
        pub const HOST_IMAGE_TRANSFER: Self = Self(1 << 46);
        pub const TENSOR_DATA_GRAPH_ARM: Self = Self(1 << 48);
        pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 49);
        pub const VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self = Self(1 << 50);
        pub const ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV: Self = Self(1 << 51);
        pub const DEPTH_COPY_ON_COMPUTE_QUEUE_KHR: Self = Self(1 << 52);
        pub const DEPTH_COPY_ON_TRANSFER_QUEUE_KHR: Self = Self(1 << 53);
        pub const STENCIL_COPY_ON_COMPUTE_QUEUE_KHR: Self = Self(1 << 54);
        pub const STENCIL_COPY_ON_TRANSFER_QUEUE_KHR: Self = Self(1 << 55);
        pub const COPY_IMAGE_INDIRECT_DST_KHR: Self = Self(1 << 59);
        pub const BLIT_DST_KHR: Self = Self::BLIT_DST;
        pub const BLIT_SRC_KHR: Self = Self::BLIT_SRC;
        pub const COLOR_ATTACHMENT_KHR: Self = Self::COLOR_ATTACHMENT;
        pub const COLOR_ATTACHMENT_BLEND_KHR: Self = Self::COLOR_ATTACHMENT_BLEND;
        pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self::COSITED_CHROMA_SAMPLES;
        pub const DEPTH_STENCIL_ATTACHMENT_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT;
        pub const DISJOINT_KHR: Self = Self::DISJOINT;
        pub const HOST_IMAGE_TRANSFER_EXT: Self = Self::HOST_IMAGE_TRANSFER;
        pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES;
        pub const SAMPLED_IMAGE_KHR: Self = Self::SAMPLED_IMAGE;
        pub const SAMPLED_IMAGE_DEPTH_COMPARISON_KHR: Self = Self::SAMPLED_IMAGE_DEPTH_COMPARISON;
        pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC;
        pub const SAMPLED_IMAGE_FILTER_LINEAR_KHR: Self = Self::SAMPLED_IMAGE_FILTER_LINEAR;
        pub const SAMPLED_IMAGE_FILTER_MINMAX_KHR: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR:
            Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
        pub const STORAGE_IMAGE_ATOMIC_KHR: Self = Self::STORAGE_IMAGE_ATOMIC;
        pub const STORAGE_IMAGE_KHR: Self = Self::STORAGE_IMAGE;
        pub const STORAGE_READ_WITHOUT_FORMAT_KHR: Self = Self::STORAGE_READ_WITHOUT_FORMAT;
        pub const STORAGE_TEXEL_BUFFER_ATOMIC_KHR: Self = Self::STORAGE_TEXEL_BUFFER_ATOMIC;
        pub const STORAGE_TEXEL_BUFFER_KHR: Self = Self::STORAGE_TEXEL_BUFFER;
        pub const STORAGE_WRITE_WITHOUT_FORMAT_KHR: Self = Self::STORAGE_WRITE_WITHOUT_FORMAT;
        pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
        pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
        pub const UNIFORM_TEXEL_BUFFER_KHR: Self = Self::UNIFORM_TEXEL_BUFFER;
        pub const VERTEX_BUFFER_KHR: Self = Self::VERTEX_BUFFER;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct RenderingFlags(Flags);
    vk_bitflags_wrapped!(RenderingFlags, Flags);
    impl RenderingFlags {
        pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self =
            Self(RenderingFlagBits::CONTENTS_SECONDARY_COMMAND_BUFFERS.0);
        pub const SUSPENDING: Self = Self(RenderingFlagBits::SUSPENDING.0);
        pub const RESUMING: Self = Self(RenderingFlagBits::RESUMING.0);
        pub const ENABLE_LEGACY_DITHERING_EXT: Self =
            Self(RenderingFlagBits::ENABLE_LEGACY_DITHERING_EXT.0);
        pub const CONTENTS_INLINE_KHR: Self = Self(RenderingFlagBits::CONTENTS_INLINE_KHR.0);
        pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self =
            Self(RenderingFlagBits::PER_LAYER_FRAGMENT_DENSITY_VALVE.0);
        pub const FRAGMENT_REGION_EXT: Self = Self(RenderingFlagBits::FRAGMENT_REGION_EXT.0);
        pub const CUSTOM_RESOLVE_EXT: Self = Self(RenderingFlagBits::CUSTOM_RESOLVE_EXT.0);
        pub const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR: Self =
            Self(RenderingFlagBits::LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR.0);
        pub const CONTENTS_INLINE_EXT: Self = Self::CONTENTS_INLINE_KHR;
        pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR: Self =
            Self::CONTENTS_SECONDARY_COMMAND_BUFFERS;
        pub const RESUMING_KHR: Self = Self::RESUMING;
        pub const SUSPENDING_KHR: Self = Self::SUSPENDING;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct RenderingFlagBits(u32);
    impl RenderingFlagBits {
        pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(1 << 0);
        pub const SUSPENDING: Self = Self(1 << 1);
        pub const RESUMING: Self = Self(1 << 2);
        pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(1 << 3);
        pub const CONTENTS_INLINE_KHR: Self = Self(1 << 4);
        pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self = Self(1 << 5);
        pub const FRAGMENT_REGION_EXT: Self = Self(1 << 6);
        pub const CUSTOM_RESOLVE_EXT: Self = Self(1 << 7);
        pub const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR: Self = Self(1 << 8);
        pub const CONTENTS_INLINE_EXT: Self = Self::CONTENTS_INLINE_KHR;
        pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR: Self =
            Self::CONTENTS_SECONDARY_COMMAND_BUFFERS;
        pub const RESUMING_KHR: Self = Self::RESUMING;
        pub const SUSPENDING_KHR: Self = Self::SUSPENDING;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ToolPurposeFlags(Flags);
    vk_bitflags_wrapped!(ToolPurposeFlags, Flags);
    impl ToolPurposeFlags {
        pub const VALIDATION: Self = Self(ToolPurposeFlagBits::VALIDATION.0);
        pub const PROFILING: Self = Self(ToolPurposeFlagBits::PROFILING.0);
        pub const TRACING: Self = Self(ToolPurposeFlagBits::TRACING.0);
        pub const ADDITIONAL_FEATURES: Self = Self(ToolPurposeFlagBits::ADDITIONAL_FEATURES.0);
        pub const MODIFYING_FEATURES: Self = Self(ToolPurposeFlagBits::MODIFYING_FEATURES.0);
        pub const DEBUG_REPORTING_EXT: Self = Self(ToolPurposeFlagBits::DEBUG_REPORTING_EXT.0);
        pub const DEBUG_MARKERS_EXT: Self = Self(ToolPurposeFlagBits::DEBUG_MARKERS_EXT.0);
        pub const ADDITIONAL_FEATURES_EXT: Self = Self::ADDITIONAL_FEATURES;
        pub const MODIFYING_FEATURES_EXT: Self = Self::MODIFYING_FEATURES;
        pub const PROFILING_EXT: Self = Self::PROFILING;
        pub const TRACING_EXT: Self = Self::TRACING;
        pub const VALIDATION_EXT: Self = Self::VALIDATION;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ToolPurposeFlagBits(u32);
    impl ToolPurposeFlagBits {
        pub const VALIDATION: Self = Self(1 << 0);
        pub const PROFILING: Self = Self(1 << 1);
        pub const TRACING: Self = Self(1 << 2);
        pub const ADDITIONAL_FEATURES: Self = Self(1 << 3);
        pub const MODIFYING_FEATURES: Self = Self(1 << 4);
        pub const DEBUG_REPORTING_EXT: Self = Self(1 << 5);
        pub const DEBUG_MARKERS_EXT: Self = Self(1 << 6);
        pub const ADDITIONAL_FEATURES_EXT: Self = Self::ADDITIONAL_FEATURES;
        pub const MODIFYING_FEATURES_EXT: Self = Self::MODIFYING_FEATURES;
        pub const PROFILING_EXT: Self = Self::PROFILING;
        pub const TRACING_EXT: Self = Self::TRACING;
        pub const VALIDATION_EXT: Self = Self::VALIDATION;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SubmitFlags(Flags);
    vk_bitflags_wrapped!(SubmitFlags, Flags);
    impl SubmitFlags {
        pub const PROTECTED: Self = Self(SubmitFlagBits::PROTECTED.0);
        pub const PROTECTED_KHR: Self = Self::PROTECTED;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SubmitFlagBits(u32);
    impl SubmitFlagBits {
        pub const PROTECTED: Self = Self(1 << 0);
        pub const PROTECTED_KHR: Self = Self::PROTECTED;
    }
    pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceBufferMemoryRequirements<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceImageMemoryRequirements<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    pub type PFN_vkGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceImageMemoryRequirements<'_>,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
    );
    pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_tool_count: *mut u32,
        p_tool_properties: *mut PhysicalDeviceToolProperties<'_>,
    ) -> vk::Result;
    pub type PFN_vkCmdSetCullMode =
        unsafe extern "system" fn(command_buffer: CommandBuffer, cull_mode: CullModeFlags);
    pub type PFN_vkCmdSetFrontFace =
        unsafe extern "system" fn(command_buffer: CommandBuffer, front_face: FrontFace);
    pub type PFN_vkCmdSetPrimitiveTopology = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    );
    pub type PFN_vkCmdSetViewportWithCount = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        viewport_count: u32,
        p_viewports: *const Viewport,
    );
    pub type PFN_vkCmdSetScissorWithCount = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    );
    pub type PFN_vkCmdBindVertexBuffers2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
        p_sizes: *const DeviceSize,
        p_strides: *const DeviceSize,
    );
    pub type PFN_vkCmdSetDepthTestEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, depth_test_enable: Bool32);
    pub type PFN_vkCmdSetDepthWriteEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, depth_write_enable: Bool32);
    pub type PFN_vkCmdSetDepthCompareOp =
        unsafe extern "system" fn(command_buffer: CommandBuffer, depth_compare_op: CompareOp);
    pub type PFN_vkCmdSetDepthBoundsTestEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bounds_test_enable: Bool32);
    pub type PFN_vkCmdSetStencilTestEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, stencil_test_enable: Bool32);
    pub type PFN_vkCmdSetStencilOp = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    );
    pub type PFN_vkCmdSetRasterizerDiscardEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, rasterizer_discard_enable: Bool32);
    pub type PFN_vkCmdSetDepthBiasEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bias_enable: Bool32);
    pub type PFN_vkCmdSetPrimitiveRestartEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_enable: Bool32);
    pub type PFN_vkCreatePrivateDataSlot = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const PrivateDataSlotCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_private_data_slot: *mut PrivateDataSlot,
    ) -> vk::Result;
    pub type PFN_vkDestroyPrivateDataSlot = unsafe extern "system" fn(
        device: Device,
        private_data_slot: PrivateDataSlot,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkSetPrivateData = unsafe extern "system" fn(
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> vk::Result;
    pub type PFN_vkGetPrivateData = unsafe extern "system" fn(
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        p_data: *mut u64,
    );
    pub type PFN_vkCmdCopyBuffer2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_buffer_info: *const CopyBufferInfo2<'_>,
    );
    pub type PFN_vkCmdCopyImage2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_image_info: *const CopyImageInfo2<'_>,
    );
    pub type PFN_vkCmdBlitImage2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_blit_image_info: *const BlitImageInfo2<'_>,
    );
    pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2<'_>,
    );
    pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2<'_>,
    );
    pub type PFN_vkCmdResolveImage2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_resolve_image_info: *const ResolveImageInfo2<'_>,
    );
    pub type PFN_vkCmdSetEvent2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        p_dependency_info: *const DependencyInfo<'_>,
    );
    pub type PFN_vkCmdResetEvent2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    );
    pub type PFN_vkCmdWaitEvents2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event_count: u32,
        p_events: *const Event,
        p_dependency_infos: *const DependencyInfo<'_>,
    );
    pub type PFN_vkCmdPipelineBarrier2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_dependency_info: *const DependencyInfo<'_>,
    );
    pub type PFN_vkQueueSubmit2 = unsafe extern "system" fn(
        queue: Queue,
        submit_count: u32,
        p_submits: *const SubmitInfo2<'_>,
        fence: Fence,
    ) -> vk::Result;
    pub type PFN_vkCmdWriteTimestamp2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    );
    pub type PFN_vkCmdBeginRendering = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_rendering_info: *const RenderingInfo<'_>,
    );
    pub type PFN_vkCmdEndRendering = unsafe extern "system" fn(command_buffer: CommandBuffer);
}
pub struct InstanceFn {
    get_physical_device_tool_properties: PFN_vkGetPhysicalDeviceToolProperties,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_tool_properties: transmute(
                    load(c"vkGetPhysicalDeviceToolProperties").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_tool_properties<'a>(
        &self,
        physical_device: PhysicalDevice,
        tool_properties: impl ExtendUninit<PhysicalDeviceToolProperties<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(tool_properties, |tool_count, tool_properties| {
                let result = (self.get_physical_device_tool_properties)(
                    physical_device,
                    tool_count,
                    tool_properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
}
pub struct DeviceFn {
    create_private_data_slot: PFN_vkCreatePrivateDataSlot,
    destroy_private_data_slot: PFN_vkDestroyPrivateDataSlot,
    set_private_data: PFN_vkSetPrivateData,
    get_private_data: PFN_vkGetPrivateData,
    cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2,
    cmd_write_timestamp2: PFN_vkCmdWriteTimestamp2,
    queue_submit2: PFN_vkQueueSubmit2,
    cmd_copy_buffer2: PFN_vkCmdCopyBuffer2,
    cmd_copy_image2: PFN_vkCmdCopyImage2,
    cmd_copy_buffer_to_image2: PFN_vkCmdCopyBufferToImage2,
    cmd_copy_image_to_buffer2: PFN_vkCmdCopyImageToBuffer2,
    get_device_buffer_memory_requirements: PFN_vkGetDeviceBufferMemoryRequirements,
    get_device_image_memory_requirements: PFN_vkGetDeviceImageMemoryRequirements,
    get_device_image_sparse_memory_requirements: PFN_vkGetDeviceImageSparseMemoryRequirements,
    cmd_set_event2: PFN_vkCmdSetEvent2,
    cmd_reset_event2: PFN_vkCmdResetEvent2,
    cmd_wait_events2: PFN_vkCmdWaitEvents2,
    cmd_blit_image2: PFN_vkCmdBlitImage2,
    cmd_resolve_image2: PFN_vkCmdResolveImage2,
    cmd_begin_rendering: PFN_vkCmdBeginRendering,
    cmd_end_rendering: PFN_vkCmdEndRendering,
    cmd_set_cull_mode: PFN_vkCmdSetCullMode,
    cmd_set_front_face: PFN_vkCmdSetFrontFace,
    cmd_set_primitive_topology: PFN_vkCmdSetPrimitiveTopology,
    cmd_set_viewport_with_count: PFN_vkCmdSetViewportWithCount,
    cmd_set_scissor_with_count: PFN_vkCmdSetScissorWithCount,
    cmd_bind_vertex_buffers2: PFN_vkCmdBindVertexBuffers2,
    cmd_set_depth_test_enable: PFN_vkCmdSetDepthTestEnable,
    cmd_set_depth_write_enable: PFN_vkCmdSetDepthWriteEnable,
    cmd_set_depth_compare_op: PFN_vkCmdSetDepthCompareOp,
    cmd_set_depth_bounds_test_enable: PFN_vkCmdSetDepthBoundsTestEnable,
    cmd_set_stencil_test_enable: PFN_vkCmdSetStencilTestEnable,
    cmd_set_stencil_op: PFN_vkCmdSetStencilOp,
    cmd_set_rasterizer_discard_enable: PFN_vkCmdSetRasterizerDiscardEnable,
    cmd_set_depth_bias_enable: PFN_vkCmdSetDepthBiasEnable,
    cmd_set_primitive_restart_enable: PFN_vkCmdSetPrimitiveRestartEnable,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_private_data_slot: transmute(
                    load(c"vkCreatePrivateDataSlot").ok_or(MissingEntryPointError)?,
                ),
                destroy_private_data_slot: transmute(
                    load(c"vkDestroyPrivateDataSlot").ok_or(MissingEntryPointError)?,
                ),
                set_private_data: transmute(
                    load(c"vkSetPrivateData").ok_or(MissingEntryPointError)?,
                ),
                get_private_data: transmute(
                    load(c"vkGetPrivateData").ok_or(MissingEntryPointError)?,
                ),
                cmd_pipeline_barrier2: transmute(
                    load(c"vkCmdPipelineBarrier2").ok_or(MissingEntryPointError)?,
                ),
                cmd_write_timestamp2: transmute(
                    load(c"vkCmdWriteTimestamp2").ok_or(MissingEntryPointError)?,
                ),
                queue_submit2: transmute(load(c"vkQueueSubmit2").ok_or(MissingEntryPointError)?),
                cmd_copy_buffer2: transmute(
                    load(c"vkCmdCopyBuffer2").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_image2: transmute(load(c"vkCmdCopyImage2").ok_or(MissingEntryPointError)?),
                cmd_copy_buffer_to_image2: transmute(
                    load(c"vkCmdCopyBufferToImage2").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_image_to_buffer2: transmute(
                    load(c"vkCmdCopyImageToBuffer2").ok_or(MissingEntryPointError)?,
                ),
                get_device_buffer_memory_requirements: transmute(
                    load(c"vkGetDeviceBufferMemoryRequirements").ok_or(MissingEntryPointError)?,
                ),
                get_device_image_memory_requirements: transmute(
                    load(c"vkGetDeviceImageMemoryRequirements").ok_or(MissingEntryPointError)?,
                ),
                get_device_image_sparse_memory_requirements: transmute(
                    load(c"vkGetDeviceImageSparseMemoryRequirements")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_set_event2: transmute(load(c"vkCmdSetEvent2").ok_or(MissingEntryPointError)?),
                cmd_reset_event2: transmute(
                    load(c"vkCmdResetEvent2").ok_or(MissingEntryPointError)?,
                ),
                cmd_wait_events2: transmute(
                    load(c"vkCmdWaitEvents2").ok_or(MissingEntryPointError)?,
                ),
                cmd_blit_image2: transmute(load(c"vkCmdBlitImage2").ok_or(MissingEntryPointError)?),
                cmd_resolve_image2: transmute(
                    load(c"vkCmdResolveImage2").ok_or(MissingEntryPointError)?,
                ),
                cmd_begin_rendering: transmute(
                    load(c"vkCmdBeginRendering").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_rendering: transmute(
                    load(c"vkCmdEndRendering").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_cull_mode: transmute(
                    load(c"vkCmdSetCullMode").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_front_face: transmute(
                    load(c"vkCmdSetFrontFace").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_primitive_topology: transmute(
                    load(c"vkCmdSetPrimitiveTopology").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_viewport_with_count: transmute(
                    load(c"vkCmdSetViewportWithCount").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_scissor_with_count: transmute(
                    load(c"vkCmdSetScissorWithCount").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_vertex_buffers2: transmute(
                    load(c"vkCmdBindVertexBuffers2").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_test_enable: transmute(
                    load(c"vkCmdSetDepthTestEnable").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_write_enable: transmute(
                    load(c"vkCmdSetDepthWriteEnable").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_compare_op: transmute(
                    load(c"vkCmdSetDepthCompareOp").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_bounds_test_enable: transmute(
                    load(c"vkCmdSetDepthBoundsTestEnable").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_stencil_test_enable: transmute(
                    load(c"vkCmdSetStencilTestEnable").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_stencil_op: transmute(
                    load(c"vkCmdSetStencilOp").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_rasterizer_discard_enable: transmute(
                    load(c"vkCmdSetRasterizerDiscardEnable").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_bias_enable: transmute(
                    load(c"vkCmdSetDepthBiasEnable").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_primitive_restart_enable: transmute(
                    load(c"vkCmdSetPrimitiveRestartEnable").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_private_data_slot(
        &self,
        device: Device,
        create_info: &PrivateDataSlotCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<PrivateDataSlot> {
        unsafe {
            let mut private_data_slot = core::mem::MaybeUninit::uninit();
            let result = (self.create_private_data_slot)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                private_data_slot.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(private_data_slot.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_private_data_slot(
        &self,
        device: Device,
        private_data_slot: PrivateDataSlot,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_private_data_slot)(device, private_data_slot, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn set_private_data(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.set_private_data)(
                device,
                object_type,
                object_handle,
                private_data_slot,
                data,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_private_data(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
    ) -> u64 {
        unsafe {
            let mut data = core::mem::MaybeUninit::uninit();
            (self.get_private_data)(
                device,
                object_type,
                object_handle,
                private_data_slot,
                data.as_mut_ptr(),
            );
            data.assume_init()
        }
    }
    pub unsafe fn cmd_pipeline_barrier2(
        &self,
        command_buffer: CommandBuffer,
        dependency_info: &DependencyInfo<'_>,
    ) {
        unsafe { (self.cmd_pipeline_barrier2)(command_buffer, dependency_info) }
    }
    pub unsafe fn cmd_write_timestamp2(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.cmd_write_timestamp2)(command_buffer, stage, query_pool, query) }
    }
    pub unsafe fn queue_submit2(
        &self,
        queue: Queue,
        submits: &[SubmitInfo2<'_>],
        fence: Fence,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_submit2)(
                queue,
                submits.len().try_into().unwrap(),
                submits.as_ptr() as _,
                fence,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_copy_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_info: &CopyBufferInfo2<'_>,
    ) {
        unsafe { (self.cmd_copy_buffer2)(command_buffer, copy_buffer_info) }
    }
    pub unsafe fn cmd_copy_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_info: &CopyImageInfo2<'_>,
    ) {
        unsafe { (self.cmd_copy_image2)(command_buffer, copy_image_info) }
    }
    pub unsafe fn cmd_copy_buffer_to_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2<'_>,
    ) {
        unsafe { (self.cmd_copy_buffer_to_image2)(command_buffer, copy_buffer_to_image_info) }
    }
    pub unsafe fn cmd_copy_image_to_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2<'_>,
    ) {
        unsafe { (self.cmd_copy_image_to_buffer2)(command_buffer, copy_image_to_buffer_info) }
    }
    pub unsafe fn get_device_buffer_memory_requirements(
        &self,
        device: Device,
        info: &DeviceBufferMemoryRequirements<'_>,
    ) -> MemoryRequirements2<'_> {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_device_buffer_memory_requirements)(
                device,
                info,
                memory_requirements.as_mut_ptr(),
            );
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn get_device_image_memory_requirements(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements<'_>,
    ) -> MemoryRequirements2<'_> {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_device_image_memory_requirements)(
                device,
                info,
                memory_requirements.as_mut_ptr(),
            );
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn get_device_image_sparse_memory_requirements<'a>(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements<'a>,
        sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements2<'a>>,
    ) {
        unsafe {
            extend_uninit(
                sparse_memory_requirements,
                |sparse_memory_requirement_count, sparse_memory_requirements| {
                    (self.get_device_image_sparse_memory_requirements)(
                        device,
                        info,
                        sparse_memory_requirement_count,
                        sparse_memory_requirements as _,
                    )
                },
            )
        }
    }
    pub unsafe fn cmd_set_event2(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        dependency_info: &DependencyInfo<'_>,
    ) {
        unsafe { (self.cmd_set_event2)(command_buffer, event, dependency_info) }
    }
    pub unsafe fn cmd_reset_event2(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        unsafe { (self.cmd_reset_event2)(command_buffer, event, stage_mask) }
    }
    pub unsafe fn cmd_wait_events2(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        dependency_infos: &[DependencyInfo<'_>],
    ) {
        unsafe {
            (self.cmd_wait_events2)(
                command_buffer,
                events.len().try_into().unwrap(),
                events.as_ptr() as _,
                dependency_infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_blit_image2(
        &self,
        command_buffer: CommandBuffer,
        blit_image_info: &BlitImageInfo2<'_>,
    ) {
        unsafe { (self.cmd_blit_image2)(command_buffer, blit_image_info) }
    }
    pub unsafe fn cmd_resolve_image2(
        &self,
        command_buffer: CommandBuffer,
        resolve_image_info: &ResolveImageInfo2<'_>,
    ) {
        unsafe { (self.cmd_resolve_image2)(command_buffer, resolve_image_info) }
    }
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: CommandBuffer,
        rendering_info: &RenderingInfo<'_>,
    ) {
        unsafe { (self.cmd_begin_rendering)(command_buffer, rendering_info) }
    }
    pub unsafe fn cmd_end_rendering(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_rendering)(command_buffer) }
    }
    pub unsafe fn cmd_set_cull_mode(
        &self,
        command_buffer: CommandBuffer,
        cull_mode: CullModeFlags,
    ) {
        unsafe { (self.cmd_set_cull_mode)(command_buffer, cull_mode) }
    }
    pub unsafe fn cmd_set_front_face(&self, command_buffer: CommandBuffer, front_face: FrontFace) {
        unsafe { (self.cmd_set_front_face)(command_buffer, front_face) }
    }
    pub unsafe fn cmd_set_primitive_topology(
        &self,
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) {
        unsafe { (self.cmd_set_primitive_topology)(command_buffer, primitive_topology) }
    }
    pub unsafe fn cmd_set_viewport_with_count(
        &self,
        command_buffer: CommandBuffer,
        viewports: &[Viewport],
    ) {
        unsafe {
            (self.cmd_set_viewport_with_count)(
                command_buffer,
                viewports.len().try_into().unwrap(),
                viewports.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_scissor_with_count(
        &self,
        command_buffer: CommandBuffer,
        scissors: &[Rect2D],
    ) {
        unsafe {
            (self.cmd_set_scissor_with_count)(
                command_buffer,
                scissors.len().try_into().unwrap(),
                scissors.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_bind_vertex_buffers2(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
        strides: Option<&[DeviceSize]>,
    ) {
        unsafe {
            (self.cmd_bind_vertex_buffers2)(
                command_buffer,
                first_binding,
                buffers.len().try_into().unwrap(),
                buffers.as_ptr() as _,
                offsets.as_ptr() as _,
                sizes.to_raw_ptr(),
                strides.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn cmd_set_depth_test_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_test_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_test_enable)(command_buffer, depth_test_enable) }
    }
    pub unsafe fn cmd_set_depth_write_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_write_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_write_enable)(command_buffer, depth_write_enable) }
    }
    pub unsafe fn cmd_set_depth_compare_op(
        &self,
        command_buffer: CommandBuffer,
        depth_compare_op: CompareOp,
    ) {
        unsafe { (self.cmd_set_depth_compare_op)(command_buffer, depth_compare_op) }
    }
    pub unsafe fn cmd_set_depth_bounds_test_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_bounds_test_enable)(command_buffer, depth_bounds_test_enable) }
    }
    pub unsafe fn cmd_set_stencil_test_enable(
        &self,
        command_buffer: CommandBuffer,
        stencil_test_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_stencil_test_enable)(command_buffer, stencil_test_enable) }
    }
    pub unsafe fn cmd_set_stencil_op(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        unsafe {
            (self.cmd_set_stencil_op)(
                command_buffer,
                face_mask,
                fail_op,
                pass_op,
                depth_fail_op,
                compare_op,
            )
        }
    }
    pub unsafe fn cmd_set_rasterizer_discard_enable(
        &self,
        command_buffer: CommandBuffer,
        rasterizer_discard_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_rasterizer_discard_enable)(command_buffer, rasterizer_discard_enable)
        }
    }
    pub unsafe fn cmd_set_depth_bias_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_bias_enable)(command_buffer, depth_bias_enable) }
    }
    pub unsafe fn cmd_set_primitive_restart_enable(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_primitive_restart_enable)(command_buffer, primitive_restart_enable) }
    }
}
