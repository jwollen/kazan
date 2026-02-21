#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderEXT(u64);
pub type ShaderRequiredSubgroupSizeCreateInfoEXT =
    PipelineShaderStageRequiredSubgroupSizeCreateInfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderObjectFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_object: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderObjectPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_binary_uuid: [u8; UUID_SIZE as usize],
    pub shader_binary_version: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ShaderCreateFlagsEXT,
    pub stage: ShaderStageFlags,
    pub next_stage: ShaderStageFlags,
    pub code_type: ShaderCodeTypeEXT,
    pub code_size: usize,
    pub p_code: *const c_void,
    pub p_name: *const c_char,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
    pub p_specialization_info: *const SpecializationInfo,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderCodeTypeEXT(i32);
impl ShaderCodeTypeEXT {
    pub const BINARY_EXT: Self = Self(0);
    pub const SPIRV_EXT: Self = Self(1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ShaderCreateFlagsEXT: Flags {
        const LINK_STAGE_EXT = 1 << 0;
    }
}
pub type PFN_vkCreateShadersEXT = unsafe extern "system" fn(
    device: Device,
    create_info_count: u32,
    p_create_infos: *const ShaderCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_shaders: *mut ShaderEXT,
) -> Result;
pub type PFN_vkDestroyShaderEXT = unsafe extern "system" fn(
    device: Device,
    shader: ShaderEXT,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetShaderBinaryDataEXT = unsafe extern "system" fn(
    device: Device,
    shader: ShaderEXT,
    p_data_size: *mut usize,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkCmdBindShadersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage_count: u32,
    p_stages: *const ShaderStageFlags,
    p_shaders: *const ShaderEXT,
);
pub type PFN_vkCmdSetDepthClampRangeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    depth_clamp_mode: DepthClampModeEXT,
    p_depth_clamp_range: *const DepthClampRangeEXT,
);
