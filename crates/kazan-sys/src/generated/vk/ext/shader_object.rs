#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ShaderEXT(u64);
pub type ShaderRequiredSubgroupSizeCreateInfoEXT<'a> =
    PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderObjectFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_object: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderObjectFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            shader_object: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceShaderObjectFeaturesEXT<'a> {
    pub fn shader_object(mut self, shader_object: Bool32) -> Self {
        self.shader_object = shader_object;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderObjectPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_binary_uuid: [u8; UUID_SIZE as usize],
    pub shader_binary_version: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderObjectPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            shader_binary_uuid: [Default::default(); _],
            shader_binary_version: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceShaderObjectPropertiesEXT<'a> {
    pub fn shader_binary_uuid(mut self, shader_binary_uuid: [u8; UUID_SIZE as usize]) -> Self {
        self.shader_binary_uuid = shader_binary_uuid;
        self
    }
    pub fn shader_binary_version(mut self, shader_binary_version: u32) -> Self {
        self.shader_binary_version = shader_binary_version;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ShaderCreateFlagsEXT,
    pub stage: ShaderStageFlagBits,
    pub next_stage: ShaderStageFlags,
    pub code_type: ShaderCodeTypeEXT,
    pub code_size: usize,
    pub p_code: *const c_void,
    pub p_name: *const c_char,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
    pub p_specialization_info: *const SpecializationInfo<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ShaderCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SHADER_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            stage: Default::default(),
            next_stage: Default::default(),
            code_type: Default::default(),
            code_size: Default::default(),
            p_code: core::ptr::null(),
            p_name: core::ptr::null(),
            set_layout_count: Default::default(),
            p_set_layouts: core::ptr::null(),
            push_constant_range_count: Default::default(),
            p_push_constant_ranges: core::ptr::null(),
            p_specialization_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ShaderCreateInfoEXT<'a> {
    pub fn flags(mut self, flags: ShaderCreateFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
    pub fn stage(mut self, stage: ShaderStageFlagBits) -> Self {
        self.stage = stage;
        self
    }
    pub fn next_stage(mut self, next_stage: ShaderStageFlags) -> Self {
        self.next_stage = next_stage;
        self
    }
    pub fn code_type(mut self, code_type: ShaderCodeTypeEXT) -> Self {
        self.code_type = code_type;
        self
    }
    pub fn code(mut self, code: &'a [u8]) -> Self {
        self.code_size = code.len().try_into().unwrap();
        self.p_code = code.as_ptr() as _;
        self
    }
    pub fn set_layouts(mut self, set_layouts: &'a [DescriptorSetLayout]) -> Self {
        self.set_layout_count = set_layouts.len().try_into().unwrap();
        self.p_set_layouts = set_layouts.as_ptr();
        self
    }
    pub fn push_constant_ranges(mut self, push_constant_ranges: &'a [PushConstantRange]) -> Self {
        self.push_constant_range_count = push_constant_ranges.len().try_into().unwrap();
        self.p_push_constant_ranges = push_constant_ranges.as_ptr();
        self
    }
    pub fn specialization_info(mut self, specialization_info: &'a SpecializationInfo<'a>) -> Self {
        self.p_specialization_info = specialization_info;
        self
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderCodeTypeEXT(i32);
impl ShaderCodeTypeEXT {
    pub const BINARY_EXT: Self = Self(0);
    pub const SPIRV_EXT: Self = Self(1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShaderCreateFlagsEXT: Flags {
        const LINK_STAGE_EXT = ShaderCreateFlagBitsEXT::LINK_STAGE_EXT.0;
        const ALLOW_VARYING_SUBGROUP_SIZE_EXT = ShaderCreateFlagBitsEXT::ALLOW_VARYING_SUBGROUP_SIZE_EXT.0;
        const REQUIRE_FULL_SUBGROUPS_EXT = ShaderCreateFlagBitsEXT::REQUIRE_FULL_SUBGROUPS_EXT.0;
        const NO_TASK_SHADER_EXT = ShaderCreateFlagBitsEXT::NO_TASK_SHADER_EXT.0;
        const DISPATCH_BASE_EXT = ShaderCreateFlagBitsEXT::DISPATCH_BASE_EXT.0;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_EXT = ShaderCreateFlagBitsEXT::FRAGMENT_SHADING_RATE_ATTACHMENT_EXT.0;
        const FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT = ShaderCreateFlagBitsEXT::FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT.0;
        const INDIRECT_BINDABLE_EXT = ShaderCreateFlagBitsEXT::INDIRECT_BINDABLE_EXT.0;
        const DESCRIPTOR_HEAP_EXT = ShaderCreateFlagBitsEXT::DESCRIPTOR_HEAP_EXT.0;
        const _64_INDEXING_EXT = ShaderCreateFlagBitsEXT::_64_INDEXING_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderCreateFlagBitsEXT(u32);
impl ShaderCreateFlagBitsEXT {
    pub const LINK_STAGE_EXT: Self = Self(1 << 0);
    pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self(1 << 1);
    pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self(1 << 2);
    pub const NO_TASK_SHADER_EXT: Self = Self(1 << 3);
    pub const DISPATCH_BASE_EXT: Self = Self(1 << 4);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_EXT: Self = Self(1 << 5);
    pub const FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(1 << 6);
    pub const INDIRECT_BINDABLE_EXT: Self = Self(1 << 7);
    pub const DESCRIPTOR_HEAP_EXT: Self = Self(1 << 10);
    pub const _64_INDEXING_EXT: Self = Self(1 << 15);
}
pub type PFN_vkCreateShadersEXT = unsafe extern "system" fn(
    device: Device,
    create_info_count: u32,
    p_create_infos: *const ShaderCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_shaders: *mut ShaderEXT,
) -> Result;
pub type PFN_vkDestroyShaderEXT = unsafe extern "system" fn(
    device: Device,
    shader: ShaderEXT,
    p_allocator: *const AllocationCallbacks<'_>,
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
    p_stages: *const ShaderStageFlagBits,
    p_shaders: *const ShaderEXT,
);
pub type PFN_vkCmdSetDepthClampRangeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    depth_clamp_mode: DepthClampModeEXT,
    p_depth_clamp_range: *const DepthClampRangeEXT,
);
