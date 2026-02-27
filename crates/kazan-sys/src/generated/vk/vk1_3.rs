#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type Flags64 = u64;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PrivateDataSlot(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DevicePrivateDataCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub private_data_slot_request_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DevicePrivateDataCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_PRIVATE_DATA_CREATE_INFO,
            p_next: core::ptr::null(),
            private_data_slot_request_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DevicePrivateDataCreateInfo<'a> {
    pub fn private_data_slot_request_count(mut self, private_data_slot_request_count: u32) -> Self {
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
impl Default for PrivateDataSlotCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PRIVATE_DATA_SLOT_CREATE_INFO,
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
impl Default for PhysicalDevicePrivateDataFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES,
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
impl Default for DeviceBufferMemoryRequirements<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_BUFFER_MEMORY_REQUIREMENTS,
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
impl Default for DeviceImageMemoryRequirements<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_IMAGE_MEMORY_REQUIREMENTS,
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
impl Default for PhysicalDeviceInlineUniformBlockFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES,
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
impl Default for PhysicalDeviceInlineUniformBlockProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES,
            p_next: core::ptr::null_mut(),
            max_inline_uniform_block_size: Default::default(),
            max_per_stage_descriptor_inline_uniform_blocks: Default::default(),
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: Default::default(),
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
        self.max_descriptor_set_inline_uniform_blocks = max_descriptor_set_inline_uniform_blocks;
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
impl Default for WriteDescriptorSetInlineUniformBlock<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK,
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
impl Default for DescriptorPoolInlineUniformBlockCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO,
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
impl Default for PhysicalDeviceMaintenance4Features<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES,
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
impl Default for PhysicalDeviceMaintenance4Properties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES,
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
impl Default for PhysicalDeviceTextureCompressionASTCHDRFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES,
            p_next: core::ptr::null_mut(),
            texture_compression_astc_hdr: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceTextureCompressionASTCHDRFeatures<'a> {
    pub fn texture_compression_astc_hdr(mut self, texture_compression_astc_hdr: Bool32) -> Self {
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
impl Default for PipelineCreationFeedbackCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_CREATION_FEEDBACK_CREATE_INFO,
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
        self.p_pipeline_stage_creation_feedbacks = pipeline_stage_creation_feedbacks.as_mut_ptr();
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
impl Default for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES,
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
impl Default for PhysicalDeviceTexelBufferAlignmentProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES,
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
impl Default for PhysicalDeviceSubgroupSizeControlFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES,
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
impl Default for PhysicalDeviceSubgroupSizeControlProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES,
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
    pub fn max_compute_workgroup_subgroups(mut self, max_compute_workgroup_subgroups: u32) -> Self {
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
impl Default for PipelineShaderStageRequiredSubgroupSizeCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO,
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
impl Default for PhysicalDevicePipelineCreationCacheControlFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES,
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
impl Default for PhysicalDeviceVulkan13Features<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
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
    pub fn texture_compression_astc_hdr(mut self, texture_compression_astc_hdr: Bool32) -> Self {
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
impl Default for PhysicalDeviceVulkan13Properties<'_> {
    fn default() -> Self {
        Self {
s_type: StructureType::PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES
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
    pub fn max_compute_workgroup_subgroups(mut self, max_compute_workgroup_subgroups: u32) -> Self {
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
        self.max_descriptor_set_inline_uniform_blocks = max_descriptor_set_inline_uniform_blocks;
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
impl Default for PhysicalDeviceToolProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TOOL_PROPERTIES,
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
impl Default for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES,
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
impl Default for PhysicalDeviceImageRobustnessFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES,
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
impl Default for BufferCopy2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_COPY_2,
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
impl Default for ImageCopy2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_COPY_2,
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
impl Default for ImageBlit2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_BLIT_2,
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
impl Default for BufferImageCopy2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_IMAGE_COPY_2,
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
impl Default for ImageResolve2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_RESOLVE_2,
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
impl Default for CopyBufferInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COPY_BUFFER_INFO_2,
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
impl Default for CopyImageInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COPY_IMAGE_INFO_2,
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
impl Default for BlitImageInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BLIT_IMAGE_INFO_2,
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
impl Default for CopyBufferToImageInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COPY_BUFFER_TO_IMAGE_INFO_2,
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
impl Default for CopyImageToBufferInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COPY_IMAGE_TO_BUFFER_INFO_2,
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
impl Default for ResolveImageInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RESOLVE_IMAGE_INFO_2,
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
impl Default for PhysicalDeviceShaderTerminateInvocationFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES,
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
impl Default for MemoryBarrier2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_BARRIER_2,
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
impl Default for ImageMemoryBarrier2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_MEMORY_BARRIER_2,
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
impl Default for BufferMemoryBarrier2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_MEMORY_BARRIER_2,
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
impl Default for DependencyInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEPENDENCY_INFO,
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
impl Default for SemaphoreSubmitInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SEMAPHORE_SUBMIT_INFO,
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
impl Default for CommandBufferSubmitInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COMMAND_BUFFER_SUBMIT_INFO,
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
impl Default for SubmitInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SUBMIT_INFO_2,
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
impl Default for PhysicalDeviceSynchronization2Features<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES,
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
impl Default for PhysicalDeviceShaderIntegerDotProductFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES,
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
impl Default for PhysicalDeviceShaderIntegerDotProductProperties<'_> {
    fn default() -> Self {
        Self {
s_type: StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES
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
impl Default for FormatProperties3<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::FORMAT_PROPERTIES_3,
            p_next: core::ptr::null_mut(),
            linear_tiling_features: Default::default(),
            optimal_tiling_features: Default::default(),
            buffer_features: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> FormatProperties3<'a> {
    pub fn linear_tiling_features(mut self, linear_tiling_features: FormatFeatureFlags2) -> Self {
        self.linear_tiling_features = linear_tiling_features;
        self
    }
    pub fn optimal_tiling_features(mut self, optimal_tiling_features: FormatFeatureFlags2) -> Self {
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
impl Default for PipelineRenderingCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_RENDERING_CREATE_INFO,
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
impl Default for RenderingInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDERING_INFO,
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
    pub fn depth_attachment(mut self, depth_attachment: &'a RenderingAttachmentInfo<'a>) -> Self {
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
impl Default for RenderingAttachmentInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDERING_ATTACHMENT_INFO,
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
impl Default for PhysicalDeviceDynamicRenderingFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES,
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
impl Default for CommandBufferInheritanceRenderingInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO,
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
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PrivateDataSlotCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineCreationFeedbackFlags: Flags {
        const VALID = PipelineCreationFeedbackFlagBits::VALID.0;
        const APPLICATION_PIPELINE_CACHE_HIT = PipelineCreationFeedbackFlagBits::APPLICATION_PIPELINE_CACHE_HIT.0;
        const BASE_PIPELINE_ACCELERATION = PipelineCreationFeedbackFlagBits::BASE_PIPELINE_ACCELERATION.0;
        const APPLICATION_PIPELINE_CACHE_HIT_EXT = Self::APPLICATION_PIPELINE_CACHE_HIT.bits();
        const BASE_PIPELINE_ACCELERATION_EXT = Self::BASE_PIPELINE_ACCELERATION.bits();
        const VALID_EXT = Self::VALID.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCreationFeedbackFlagBits(u32);
impl PipelineCreationFeedbackFlagBits {
    pub const VALID: Self = Self(1 << 0);
    pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(1 << 1);
    pub const BASE_PIPELINE_ACCELERATION: Self = Self(1 << 2);
    pub const APPLICATION_PIPELINE_CACHE_HIT_EXT: Self = Self::APPLICATION_PIPELINE_CACHE_HIT;
    pub const BASE_PIPELINE_ACCELERATION_EXT: Self = Self::BASE_PIPELINE_ACCELERATION;
    pub const VALID_EXT: Self = Self::VALID;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccessFlags2: Flags64 {
        const INDIRECT_COMMAND_READ = AccessFlagBits2::INDIRECT_COMMAND_READ.0;
        const INDEX_READ = AccessFlagBits2::INDEX_READ.0;
        const VERTEX_ATTRIBUTE_READ = AccessFlagBits2::VERTEX_ATTRIBUTE_READ.0;
        const UNIFORM_READ = AccessFlagBits2::UNIFORM_READ.0;
        const INPUT_ATTACHMENT_READ = AccessFlagBits2::INPUT_ATTACHMENT_READ.0;
        const SHADER_READ = AccessFlagBits2::SHADER_READ.0;
        const SHADER_WRITE = AccessFlagBits2::SHADER_WRITE.0;
        const COLOR_ATTACHMENT_READ = AccessFlagBits2::COLOR_ATTACHMENT_READ.0;
        const COLOR_ATTACHMENT_WRITE = AccessFlagBits2::COLOR_ATTACHMENT_WRITE.0;
        const DEPTH_STENCIL_ATTACHMENT_READ = AccessFlagBits2::DEPTH_STENCIL_ATTACHMENT_READ.0;
        const DEPTH_STENCIL_ATTACHMENT_WRITE = AccessFlagBits2::DEPTH_STENCIL_ATTACHMENT_WRITE.0;
        const TRANSFER_READ = AccessFlagBits2::TRANSFER_READ.0;
        const TRANSFER_WRITE = AccessFlagBits2::TRANSFER_WRITE.0;
        const HOST_READ = AccessFlagBits2::HOST_READ.0;
        const HOST_WRITE = AccessFlagBits2::HOST_WRITE.0;
        const MEMORY_READ = AccessFlagBits2::MEMORY_READ.0;
        const MEMORY_WRITE = AccessFlagBits2::MEMORY_WRITE.0;
        const COMMAND_PREPROCESS_READ_EXT = AccessFlagBits2::COMMAND_PREPROCESS_READ_EXT.0;
        const COMMAND_PREPROCESS_WRITE_EXT = AccessFlagBits2::COMMAND_PREPROCESS_WRITE_EXT.0;
        const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT = AccessFlagBits2::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT.0;
        const CONDITIONAL_RENDERING_READ_EXT = AccessFlagBits2::CONDITIONAL_RENDERING_READ_EXT.0;
        const ACCELERATION_STRUCTURE_READ_KHR = AccessFlagBits2::ACCELERATION_STRUCTURE_READ_KHR.0;
        const ACCELERATION_STRUCTURE_WRITE_KHR = AccessFlagBits2::ACCELERATION_STRUCTURE_WRITE_KHR.0;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR = AccessFlagBits2::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR.0;
        const FRAGMENT_DENSITY_MAP_READ_EXT = AccessFlagBits2::FRAGMENT_DENSITY_MAP_READ_EXT.0;
        const TRANSFORM_FEEDBACK_WRITE_EXT = AccessFlagBits2::TRANSFORM_FEEDBACK_WRITE_EXT.0;
        const TRANSFORM_FEEDBACK_COUNTER_READ_EXT = AccessFlagBits2::TRANSFORM_FEEDBACK_COUNTER_READ_EXT.0;
        const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT = AccessFlagBits2::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT.0;
        const SHADER_SAMPLED_READ = AccessFlagBits2::SHADER_SAMPLED_READ.0;
        const SHADER_STORAGE_READ = AccessFlagBits2::SHADER_STORAGE_READ.0;
        const SHADER_STORAGE_WRITE = AccessFlagBits2::SHADER_STORAGE_WRITE.0;
        const VIDEO_DECODE_READ_KHR = AccessFlagBits2::VIDEO_DECODE_READ_KHR.0;
        const VIDEO_DECODE_WRITE_KHR = AccessFlagBits2::VIDEO_DECODE_WRITE_KHR.0;
        const VIDEO_ENCODE_READ_KHR = AccessFlagBits2::VIDEO_ENCODE_READ_KHR.0;
        const VIDEO_ENCODE_WRITE_KHR = AccessFlagBits2::VIDEO_ENCODE_WRITE_KHR.0;
        const INVOCATION_MASK_READ_HUAWEI = AccessFlagBits2::INVOCATION_MASK_READ_HUAWEI.0;
        const SHADER_BINDING_TABLE_READ_KHR = AccessFlagBits2::SHADER_BINDING_TABLE_READ_KHR.0;
        const DESCRIPTOR_BUFFER_READ_EXT = AccessFlagBits2::DESCRIPTOR_BUFFER_READ_EXT.0;
        const OPTICAL_FLOW_READ_NV = AccessFlagBits2::OPTICAL_FLOW_READ_NV.0;
        const OPTICAL_FLOW_WRITE_NV = AccessFlagBits2::OPTICAL_FLOW_WRITE_NV.0;
        const MICROMAP_READ_EXT = AccessFlagBits2::MICROMAP_READ_EXT.0;
        const MICROMAP_WRITE_EXT = AccessFlagBits2::MICROMAP_WRITE_EXT.0;
        const DATA_GRAPH_READ_ARM = AccessFlagBits2::DATA_GRAPH_READ_ARM.0;
        const DATA_GRAPH_WRITE_ARM = AccessFlagBits2::DATA_GRAPH_WRITE_ARM.0;
        const SHADER_TILE_ATTACHMENT_READ_QCOM = AccessFlagBits2::SHADER_TILE_ATTACHMENT_READ_QCOM.0;
        const SHADER_TILE_ATTACHMENT_WRITE_QCOM = AccessFlagBits2::SHADER_TILE_ATTACHMENT_WRITE_QCOM.0;
        const MEMORY_DECOMPRESSION_READ_EXT = AccessFlagBits2::MEMORY_DECOMPRESSION_READ_EXT.0;
        const MEMORY_DECOMPRESSION_WRITE_EXT = AccessFlagBits2::MEMORY_DECOMPRESSION_WRITE_EXT.0;
        const SAMPLER_HEAP_READ_EXT = AccessFlagBits2::SAMPLER_HEAP_READ_EXT.0;
        const RESOURCE_HEAP_READ_EXT = AccessFlagBits2::RESOURCE_HEAP_READ_EXT.0;
        const ACCELERATION_STRUCTURE_READ_NV = Self::ACCELERATION_STRUCTURE_READ_KHR.bits();
        const ACCELERATION_STRUCTURE_WRITE_NV = Self::ACCELERATION_STRUCTURE_WRITE_KHR.bits();
        const COLOR_ATTACHMENT_READ_KHR = Self::COLOR_ATTACHMENT_READ.bits();
        const COLOR_ATTACHMENT_WRITE_KHR = Self::COLOR_ATTACHMENT_WRITE.bits();
        const COMMAND_PREPROCESS_READ_NV = Self::COMMAND_PREPROCESS_READ_EXT.bits();
        const COMMAND_PREPROCESS_WRITE_NV = Self::COMMAND_PREPROCESS_WRITE_EXT.bits();
        const DEPTH_STENCIL_ATTACHMENT_READ_KHR = Self::DEPTH_STENCIL_ATTACHMENT_READ.bits();
        const DEPTH_STENCIL_ATTACHMENT_WRITE_KHR = Self::DEPTH_STENCIL_ATTACHMENT_WRITE.bits();
        const HOST_READ_KHR = Self::HOST_READ.bits();
        const HOST_WRITE_KHR = Self::HOST_WRITE.bits();
        const INDEX_READ_KHR = Self::INDEX_READ.bits();
        const INDIRECT_COMMAND_READ_KHR = Self::INDIRECT_COMMAND_READ.bits();
        const INPUT_ATTACHMENT_READ_KHR = Self::INPUT_ATTACHMENT_READ.bits();
        const MEMORY_READ_KHR = Self::MEMORY_READ.bits();
        const MEMORY_WRITE_KHR = Self::MEMORY_WRITE.bits();
        const NONE_KHR = Self::NONE.bits();
        const SHADER_READ_KHR = Self::SHADER_READ.bits();
        const SHADER_SAMPLED_READ_KHR = Self::SHADER_SAMPLED_READ.bits();
        const SHADER_STORAGE_READ_KHR = Self::SHADER_STORAGE_READ.bits();
        const SHADER_STORAGE_WRITE_KHR = Self::SHADER_STORAGE_WRITE.bits();
        const SHADER_WRITE_KHR = Self::SHADER_WRITE.bits();
        const SHADING_RATE_IMAGE_READ_NV = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR.bits();
        const TRANSFER_READ_KHR = Self::TRANSFER_READ.bits();
        const TRANSFER_WRITE_KHR = Self::TRANSFER_WRITE.bits();
        const UNIFORM_READ_KHR = Self::UNIFORM_READ.bits();
        const VERTEX_ATTRIBUTE_READ_KHR = Self::VERTEX_ATTRIBUTE_READ.bits();
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const SHADING_RATE_IMAGE_READ_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
    pub const TRANSFER_READ_KHR: Self = Self::TRANSFER_READ;
    pub const TRANSFER_WRITE_KHR: Self = Self::TRANSFER_WRITE;
    pub const UNIFORM_READ_KHR: Self = Self::UNIFORM_READ;
    pub const VERTEX_ATTRIBUTE_READ_KHR: Self = Self::VERTEX_ATTRIBUTE_READ;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineStageFlags2: Flags64 {
        const TOP_OF_PIPE = PipelineStageFlagBits2::TOP_OF_PIPE.0;
        const DRAW_INDIRECT = PipelineStageFlagBits2::DRAW_INDIRECT.0;
        const VERTEX_INPUT = PipelineStageFlagBits2::VERTEX_INPUT.0;
        const VERTEX_SHADER = PipelineStageFlagBits2::VERTEX_SHADER.0;
        const TESSELLATION_CONTROL_SHADER = PipelineStageFlagBits2::TESSELLATION_CONTROL_SHADER.0;
        const TESSELLATION_EVALUATION_SHADER = PipelineStageFlagBits2::TESSELLATION_EVALUATION_SHADER.0;
        const GEOMETRY_SHADER = PipelineStageFlagBits2::GEOMETRY_SHADER.0;
        const FRAGMENT_SHADER = PipelineStageFlagBits2::FRAGMENT_SHADER.0;
        const EARLY_FRAGMENT_TESTS = PipelineStageFlagBits2::EARLY_FRAGMENT_TESTS.0;
        const LATE_FRAGMENT_TESTS = PipelineStageFlagBits2::LATE_FRAGMENT_TESTS.0;
        const COLOR_ATTACHMENT_OUTPUT = PipelineStageFlagBits2::COLOR_ATTACHMENT_OUTPUT.0;
        const COMPUTE_SHADER = PipelineStageFlagBits2::COMPUTE_SHADER.0;
        const ALL_TRANSFER = PipelineStageFlagBits2::ALL_TRANSFER.0;
        const BOTTOM_OF_PIPE = PipelineStageFlagBits2::BOTTOM_OF_PIPE.0;
        const HOST = PipelineStageFlagBits2::HOST.0;
        const ALL_GRAPHICS = PipelineStageFlagBits2::ALL_GRAPHICS.0;
        const ALL_COMMANDS = PipelineStageFlagBits2::ALL_COMMANDS.0;
        const COMMAND_PREPROCESS_EXT = PipelineStageFlagBits2::COMMAND_PREPROCESS_EXT.0;
        const CONDITIONAL_RENDERING_EXT = PipelineStageFlagBits2::CONDITIONAL_RENDERING_EXT.0;
        const TASK_SHADER_EXT = PipelineStageFlagBits2::TASK_SHADER_EXT.0;
        const MESH_SHADER_EXT = PipelineStageFlagBits2::MESH_SHADER_EXT.0;
        const RAY_TRACING_SHADER_KHR = PipelineStageFlagBits2::RAY_TRACING_SHADER_KHR.0;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = PipelineStageFlagBits2::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0;
        const FRAGMENT_DENSITY_PROCESS_EXT = PipelineStageFlagBits2::FRAGMENT_DENSITY_PROCESS_EXT.0;
        const TRANSFORM_FEEDBACK_EXT = PipelineStageFlagBits2::TRANSFORM_FEEDBACK_EXT.0;
        const ACCELERATION_STRUCTURE_BUILD_KHR = PipelineStageFlagBits2::ACCELERATION_STRUCTURE_BUILD_KHR.0;
        const VIDEO_DECODE_KHR = PipelineStageFlagBits2::VIDEO_DECODE_KHR.0;
        const VIDEO_ENCODE_KHR = PipelineStageFlagBits2::VIDEO_ENCODE_KHR.0;
        const ACCELERATION_STRUCTURE_COPY_KHR = PipelineStageFlagBits2::ACCELERATION_STRUCTURE_COPY_KHR.0;
        const OPTICAL_FLOW_NV = PipelineStageFlagBits2::OPTICAL_FLOW_NV.0;
        const MICROMAP_BUILD_EXT = PipelineStageFlagBits2::MICROMAP_BUILD_EXT.0;
        const COPY = PipelineStageFlagBits2::COPY.0;
        const RESOLVE = PipelineStageFlagBits2::RESOLVE.0;
        const BLIT = PipelineStageFlagBits2::BLIT.0;
        const CLEAR = PipelineStageFlagBits2::CLEAR.0;
        const INDEX_INPUT = PipelineStageFlagBits2::INDEX_INPUT.0;
        const VERTEX_ATTRIBUTE_INPUT = PipelineStageFlagBits2::VERTEX_ATTRIBUTE_INPUT.0;
        const PRE_RASTERIZATION_SHADERS = PipelineStageFlagBits2::PRE_RASTERIZATION_SHADERS.0;
        const SUBPASS_SHADER_HUAWEI = PipelineStageFlagBits2::SUBPASS_SHADER_HUAWEI.0;
        const INVOCATION_MASK_HUAWEI = PipelineStageFlagBits2::INVOCATION_MASK_HUAWEI.0;
        const CLUSTER_CULLING_SHADER_HUAWEI = PipelineStageFlagBits2::CLUSTER_CULLING_SHADER_HUAWEI.0;
        const DATA_GRAPH_ARM = PipelineStageFlagBits2::DATA_GRAPH_ARM.0;
        const CONVERT_COOPERATIVE_VECTOR_MATRIX_NV = PipelineStageFlagBits2::CONVERT_COOPERATIVE_VECTOR_MATRIX_NV.0;
        const MEMORY_DECOMPRESSION_EXT = PipelineStageFlagBits2::MEMORY_DECOMPRESSION_EXT.0;
        const COPY_INDIRECT_KHR = PipelineStageFlagBits2::COPY_INDIRECT_KHR.0;
        const TRANSFER = Self::ALL_TRANSFER.bits();
        const ACCELERATION_STRUCTURE_BUILD_NV = Self::ACCELERATION_STRUCTURE_BUILD_KHR.bits();
        const ALL_COMMANDS_KHR = Self::ALL_COMMANDS.bits();
        const ALL_GRAPHICS_KHR = Self::ALL_GRAPHICS.bits();
        const ALL_TRANSFER_KHR = Self::ALL_TRANSFER.bits();
        const BLIT_KHR = Self::BLIT.bits();
        const BOTTOM_OF_PIPE_KHR = Self::BOTTOM_OF_PIPE.bits();
        const CLEAR_KHR = Self::CLEAR.bits();
        const COLOR_ATTACHMENT_OUTPUT_KHR = Self::COLOR_ATTACHMENT_OUTPUT.bits();
        const COMMAND_PREPROCESS_NV = Self::COMMAND_PREPROCESS_EXT.bits();
        const COMPUTE_SHADER_KHR = Self::COMPUTE_SHADER.bits();
        const COPY_KHR = Self::COPY.bits();
        const DRAW_INDIRECT_KHR = Self::DRAW_INDIRECT.bits();
        const EARLY_FRAGMENT_TESTS_KHR = Self::EARLY_FRAGMENT_TESTS.bits();
        const FRAGMENT_SHADER_KHR = Self::FRAGMENT_SHADER.bits();
        const GEOMETRY_SHADER_KHR = Self::GEOMETRY_SHADER.bits();
        const HOST_KHR = Self::HOST.bits();
        const INDEX_INPUT_KHR = Self::INDEX_INPUT.bits();
        const LATE_FRAGMENT_TESTS_KHR = Self::LATE_FRAGMENT_TESTS.bits();
        const MESH_SHADER_NV = Self::MESH_SHADER_EXT.bits();
        const NONE_KHR = Self::NONE.bits();
        const PRE_RASTERIZATION_SHADERS_KHR = Self::PRE_RASTERIZATION_SHADERS.bits();
        const RAY_TRACING_SHADER_NV = Self::RAY_TRACING_SHADER_KHR.bits();
        const RESOLVE_KHR = Self::RESOLVE.bits();
        const SHADING_RATE_IMAGE_NV = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.bits();
        const SUBPASS_SHADING_HUAWEI = Self::SUBPASS_SHADER_HUAWEI.bits();
        const TASK_SHADER_NV = Self::TASK_SHADER_EXT.bits();
        const TESSELLATION_CONTROL_SHADER_KHR = Self::TESSELLATION_CONTROL_SHADER.bits();
        const TESSELLATION_EVALUATION_SHADER_KHR = Self::TESSELLATION_EVALUATION_SHADER.bits();
        const TOP_OF_PIPE_KHR = Self::TOP_OF_PIPE.bits();
        const TRANSFER_KHR = Self::ALL_TRANSFER.bits();
        const VERTEX_ATTRIBUTE_INPUT_KHR = Self::VERTEX_ATTRIBUTE_INPUT.bits();
        const VERTEX_INPUT_KHR = Self::VERTEX_INPUT.bits();
        const VERTEX_SHADER_KHR = Self::VERTEX_SHADER.bits();
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FormatFeatureFlags2: Flags64 {
        const SAMPLED_IMAGE = FormatFeatureFlagBits2::SAMPLED_IMAGE.0;
        const STORAGE_IMAGE = FormatFeatureFlagBits2::STORAGE_IMAGE.0;
        const STORAGE_IMAGE_ATOMIC = FormatFeatureFlagBits2::STORAGE_IMAGE_ATOMIC.0;
        const UNIFORM_TEXEL_BUFFER = FormatFeatureFlagBits2::UNIFORM_TEXEL_BUFFER.0;
        const STORAGE_TEXEL_BUFFER = FormatFeatureFlagBits2::STORAGE_TEXEL_BUFFER.0;
        const STORAGE_TEXEL_BUFFER_ATOMIC = FormatFeatureFlagBits2::STORAGE_TEXEL_BUFFER_ATOMIC.0;
        const VERTEX_BUFFER = FormatFeatureFlagBits2::VERTEX_BUFFER.0;
        const COLOR_ATTACHMENT = FormatFeatureFlagBits2::COLOR_ATTACHMENT.0;
        const COLOR_ATTACHMENT_BLEND = FormatFeatureFlagBits2::COLOR_ATTACHMENT_BLEND.0;
        const DEPTH_STENCIL_ATTACHMENT = FormatFeatureFlagBits2::DEPTH_STENCIL_ATTACHMENT.0;
        const BLIT_SRC = FormatFeatureFlagBits2::BLIT_SRC.0;
        const BLIT_DST = FormatFeatureFlagBits2::BLIT_DST.0;
        const SAMPLED_IMAGE_FILTER_LINEAR = FormatFeatureFlagBits2::SAMPLED_IMAGE_FILTER_LINEAR.0;
        const SAMPLED_IMAGE_FILTER_CUBIC = FormatFeatureFlagBits2::SAMPLED_IMAGE_FILTER_CUBIC.0;
        const TRANSFER_SRC = FormatFeatureFlagBits2::TRANSFER_SRC.0;
        const TRANSFER_DST = FormatFeatureFlagBits2::TRANSFER_DST.0;
        const SAMPLED_IMAGE_FILTER_MINMAX = FormatFeatureFlagBits2::SAMPLED_IMAGE_FILTER_MINMAX.0;
        const MIDPOINT_CHROMA_SAMPLES = FormatFeatureFlagBits2::MIDPOINT_CHROMA_SAMPLES.0;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER = FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER.0;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER = FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER.0;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT = FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT.0;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE = FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE.0;
        const DISJOINT = FormatFeatureFlagBits2::DISJOINT.0;
        const COSITED_CHROMA_SAMPLES = FormatFeatureFlagBits2::COSITED_CHROMA_SAMPLES.0;
        const FRAGMENT_DENSITY_MAP_EXT = FormatFeatureFlagBits2::FRAGMENT_DENSITY_MAP_EXT.0;
        const VIDEO_DECODE_OUTPUT_KHR = FormatFeatureFlagBits2::VIDEO_DECODE_OUTPUT_KHR.0;
        const VIDEO_DECODE_DPB_KHR = FormatFeatureFlagBits2::VIDEO_DECODE_DPB_KHR.0;
        const VIDEO_ENCODE_INPUT_KHR = FormatFeatureFlagBits2::VIDEO_ENCODE_INPUT_KHR.0;
        const VIDEO_ENCODE_DPB_KHR = FormatFeatureFlagBits2::VIDEO_ENCODE_DPB_KHR.0;
        const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR = FormatFeatureFlagBits2::ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR.0;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = FormatFeatureFlagBits2::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0;
        const STORAGE_READ_WITHOUT_FORMAT = FormatFeatureFlagBits2::STORAGE_READ_WITHOUT_FORMAT.0;
        const STORAGE_WRITE_WITHOUT_FORMAT = FormatFeatureFlagBits2::STORAGE_WRITE_WITHOUT_FORMAT.0;
        const SAMPLED_IMAGE_DEPTH_COMPARISON = FormatFeatureFlagBits2::SAMPLED_IMAGE_DEPTH_COMPARISON.0;
        const WEIGHT_IMAGE_QCOM = FormatFeatureFlagBits2::WEIGHT_IMAGE_QCOM.0;
        const WEIGHT_SAMPLED_IMAGE_QCOM = FormatFeatureFlagBits2::WEIGHT_SAMPLED_IMAGE_QCOM.0;
        const BLOCK_MATCHING_QCOM = FormatFeatureFlagBits2::BLOCK_MATCHING_QCOM.0;
        const BOX_FILTER_SAMPLED_QCOM = FormatFeatureFlagBits2::BOX_FILTER_SAMPLED_QCOM.0;
        const LINEAR_COLOR_ATTACHMENT_NV = FormatFeatureFlagBits2::LINEAR_COLOR_ATTACHMENT_NV.0;
        const TENSOR_SHADER_ARM = FormatFeatureFlagBits2::TENSOR_SHADER_ARM.0;
        const OPTICAL_FLOW_IMAGE_NV = FormatFeatureFlagBits2::OPTICAL_FLOW_IMAGE_NV.0;
        const OPTICAL_FLOW_VECTOR_NV = FormatFeatureFlagBits2::OPTICAL_FLOW_VECTOR_NV.0;
        const OPTICAL_FLOW_COST_NV = FormatFeatureFlagBits2::OPTICAL_FLOW_COST_NV.0;
        const TENSOR_IMAGE_ALIASING_ARM = FormatFeatureFlagBits2::TENSOR_IMAGE_ALIASING_ARM.0;
        const HOST_IMAGE_TRANSFER = FormatFeatureFlagBits2::HOST_IMAGE_TRANSFER.0;
        const TENSOR_DATA_GRAPH_ARM = FormatFeatureFlagBits2::TENSOR_DATA_GRAPH_ARM.0;
        const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR = FormatFeatureFlagBits2::VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR.0;
        const VIDEO_ENCODE_EMPHASIS_MAP_KHR = FormatFeatureFlagBits2::VIDEO_ENCODE_EMPHASIS_MAP_KHR.0;
        const ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV = FormatFeatureFlagBits2::ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV.0;
        const DEPTH_COPY_ON_COMPUTE_QUEUE_KHR = FormatFeatureFlagBits2::DEPTH_COPY_ON_COMPUTE_QUEUE_KHR.0;
        const DEPTH_COPY_ON_TRANSFER_QUEUE_KHR = FormatFeatureFlagBits2::DEPTH_COPY_ON_TRANSFER_QUEUE_KHR.0;
        const STENCIL_COPY_ON_COMPUTE_QUEUE_KHR = FormatFeatureFlagBits2::STENCIL_COPY_ON_COMPUTE_QUEUE_KHR.0;
        const STENCIL_COPY_ON_TRANSFER_QUEUE_KHR = FormatFeatureFlagBits2::STENCIL_COPY_ON_TRANSFER_QUEUE_KHR.0;
        const COPY_IMAGE_INDIRECT_DST_KHR = FormatFeatureFlagBits2::COPY_IMAGE_INDIRECT_DST_KHR.0;
        const BLIT_DST_KHR = Self::BLIT_DST.bits();
        const BLIT_SRC_KHR = Self::BLIT_SRC.bits();
        const COLOR_ATTACHMENT_KHR = Self::COLOR_ATTACHMENT.bits();
        const COLOR_ATTACHMENT_BLEND_KHR = Self::COLOR_ATTACHMENT_BLEND.bits();
        const COSITED_CHROMA_SAMPLES_KHR = Self::COSITED_CHROMA_SAMPLES.bits();
        const DEPTH_STENCIL_ATTACHMENT_KHR = Self::DEPTH_STENCIL_ATTACHMENT.bits();
        const DISJOINT_KHR = Self::DISJOINT.bits();
        const HOST_IMAGE_TRANSFER_EXT = Self::HOST_IMAGE_TRANSFER.bits();
        const MIDPOINT_CHROMA_SAMPLES_KHR = Self::MIDPOINT_CHROMA_SAMPLES.bits();
        const SAMPLED_IMAGE_KHR = Self::SAMPLED_IMAGE.bits();
        const SAMPLED_IMAGE_DEPTH_COMPARISON_KHR = Self::SAMPLED_IMAGE_DEPTH_COMPARISON.bits();
        const SAMPLED_IMAGE_FILTER_CUBIC_EXT = Self::SAMPLED_IMAGE_FILTER_CUBIC.bits();
        const SAMPLED_IMAGE_FILTER_LINEAR_KHR = Self::SAMPLED_IMAGE_FILTER_LINEAR.bits();
        const SAMPLED_IMAGE_FILTER_MINMAX_KHR = Self::SAMPLED_IMAGE_FILTER_MINMAX.bits();
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT.bits();
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE.bits();
        const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER.bits();
        const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER.bits();
        const STORAGE_IMAGE_ATOMIC_KHR = Self::STORAGE_IMAGE_ATOMIC.bits();
        const STORAGE_IMAGE_KHR = Self::STORAGE_IMAGE.bits();
        const STORAGE_READ_WITHOUT_FORMAT_KHR = Self::STORAGE_READ_WITHOUT_FORMAT.bits();
        const STORAGE_TEXEL_BUFFER_ATOMIC_KHR = Self::STORAGE_TEXEL_BUFFER_ATOMIC.bits();
        const STORAGE_TEXEL_BUFFER_KHR = Self::STORAGE_TEXEL_BUFFER.bits();
        const STORAGE_WRITE_WITHOUT_FORMAT_KHR = Self::STORAGE_WRITE_WITHOUT_FORMAT.bits();
        const TRANSFER_DST_KHR = Self::TRANSFER_DST.bits();
        const TRANSFER_SRC_KHR = Self::TRANSFER_SRC.bits();
        const UNIFORM_TEXEL_BUFFER_KHR = Self::UNIFORM_TEXEL_BUFFER.bits();
        const VERTEX_BUFFER_KHR = Self::VERTEX_BUFFER.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self(1 << 19);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self = Self(1 << 20);
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
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
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
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RenderingFlags: Flags {
        const CONTENTS_SECONDARY_COMMAND_BUFFERS = RenderingFlagBits::CONTENTS_SECONDARY_COMMAND_BUFFERS.0;
        const SUSPENDING = RenderingFlagBits::SUSPENDING.0;
        const RESUMING = RenderingFlagBits::RESUMING.0;
        const ENABLE_LEGACY_DITHERING_EXT = RenderingFlagBits::ENABLE_LEGACY_DITHERING_EXT.0;
        const CONTENTS_INLINE_KHR = RenderingFlagBits::CONTENTS_INLINE_KHR.0;
        const PER_LAYER_FRAGMENT_DENSITY_VALVE = RenderingFlagBits::PER_LAYER_FRAGMENT_DENSITY_VALVE.0;
        const FRAGMENT_REGION_EXT = RenderingFlagBits::FRAGMENT_REGION_EXT.0;
        const CUSTOM_RESOLVE_EXT = RenderingFlagBits::CUSTOM_RESOLVE_EXT.0;
        const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR = RenderingFlagBits::LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR.0;
        const CONTENTS_INLINE_EXT = Self::CONTENTS_INLINE_KHR.bits();
        const CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR = Self::CONTENTS_SECONDARY_COMMAND_BUFFERS.bits();
        const RESUMING_KHR = Self::RESUMING.bits();
        const SUSPENDING_KHR = Self::SUSPENDING.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ToolPurposeFlags: Flags {
        const VALIDATION = ToolPurposeFlagBits::VALIDATION.0;
        const PROFILING = ToolPurposeFlagBits::PROFILING.0;
        const TRACING = ToolPurposeFlagBits::TRACING.0;
        const ADDITIONAL_FEATURES = ToolPurposeFlagBits::ADDITIONAL_FEATURES.0;
        const MODIFYING_FEATURES = ToolPurposeFlagBits::MODIFYING_FEATURES.0;
        const DEBUG_REPORTING_EXT = ToolPurposeFlagBits::DEBUG_REPORTING_EXT.0;
        const DEBUG_MARKERS_EXT = ToolPurposeFlagBits::DEBUG_MARKERS_EXT.0;
        const ADDITIONAL_FEATURES_EXT = Self::ADDITIONAL_FEATURES.bits();
        const MODIFYING_FEATURES_EXT = Self::MODIFYING_FEATURES.bits();
        const PROFILING_EXT = Self::PROFILING.bits();
        const TRACING_EXT = Self::TRACING.bits();
        const VALIDATION_EXT = Self::VALIDATION.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SubmitFlags: Flags {
        const PROTECTED = SubmitFlagBits::PROTECTED.0;
        const PROTECTED_KHR = Self::PROTECTED.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
) -> Result;
pub type PFN_vkCmdSetCullMode =
    unsafe extern "system" fn(command_buffer: CommandBuffer, cull_mode: CullModeFlags);
pub type PFN_vkCmdSetFrontFace =
    unsafe extern "system" fn(command_buffer: CommandBuffer, front_face: FrontFace);
pub type PFN_vkCmdSetPrimitiveTopology =
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_topology: PrimitiveTopology);
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
) -> Result;
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
) -> Result;
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
) -> Result;
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
