#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub type PhysicalDeviceMaintenance6FeaturesKHR<'a> = PhysicalDeviceMaintenance6Features<'a>;
    pub type PhysicalDeviceMaintenance6PropertiesKHR<'a> = PhysicalDeviceMaintenance6Properties<'a>;
    pub type BindMemoryStatusKHR<'a> = BindMemoryStatus<'a>;
    pub type BindDescriptorSetsInfoKHR<'a> = BindDescriptorSetsInfo<'a>;
    pub type PushConstantsInfoKHR<'a> = PushConstantsInfo<'a>;
    pub type PushDescriptorSetInfoKHR<'a> = PushDescriptorSetInfo<'a>;
    pub type PushDescriptorSetWithTemplateInfoKHR<'a> = PushDescriptorSetWithTemplateInfo<'a>;
    pub type PFN_vkCmdBindDescriptorSets2KHR = PFN_vkCmdBindDescriptorSets2;
    pub type PFN_vkCmdPushConstants2KHR = PFN_vkCmdPushConstants2;
    pub type PFN_vkCmdPushDescriptorSet2KHR = PFN_vkCmdPushDescriptorSet2;
    pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = PFN_vkCmdPushDescriptorSetWithTemplate2;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SetDescriptorBufferOffsetsInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stage_flags: ShaderStageFlags,
        pub layout: PipelineLayout,
        pub first_set: u32,
        pub set_count: u32,
        pub p_buffer_indices: *const u32,
        pub p_offsets: *const DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SetDescriptorBufferOffsetsInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT,
                p_next: core::ptr::null(),
                stage_flags: Default::default(),
                layout: Default::default(),
                first_set: Default::default(),
                set_count: Default::default(),
                p_buffer_indices: core::ptr::null(),
                p_offsets: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SetDescriptorBufferOffsetsInfoEXT<'a> {
        pub fn stage_flags(mut self, stage_flags: ShaderStageFlags) -> Self {
            self.stage_flags = stage_flags;
            self
        }
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }
        pub fn first_set(mut self, first_set: u32) -> Self {
            self.first_set = first_set;
            self
        }
        pub fn buffer_indices(mut self, buffer_indices: &'a [u32]) -> Self {
            self.set_count = buffer_indices.len().try_into().unwrap();
            self.p_buffer_indices = buffer_indices.as_ptr();
            self
        }
        pub fn offsets(mut self, offsets: &'a [DeviceSize]) -> Self {
            self.set_count = offsets.len().try_into().unwrap();
            self.p_offsets = offsets.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindDescriptorBufferEmbeddedSamplersInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stage_flags: ShaderStageFlags,
        pub layout: PipelineLayout,
        pub set: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for BindDescriptorBufferEmbeddedSamplersInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT,
                p_next: core::ptr::null(),
                stage_flags: Default::default(),
                layout: Default::default(),
                set: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BindDescriptorBufferEmbeddedSamplersInfoEXT<'a> {
        pub fn stage_flags(mut self, stage_flags: ShaderStageFlags) -> Self {
            self.stage_flags = stage_flags;
            self
        }
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }
        pub fn set(mut self, set: u32) -> Self {
            self.set = set;
            self
        }
    }
    pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT<'_>,
    );
    pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_bind_descriptor_buffer_embedded_samplers_info: *const BindDescriptorBufferEmbeddedSamplersInfoEXT<'_>,
);
}
pub struct DeviceFn {
    cmd_bind_descriptor_sets2_khr: PFN_vkCmdBindDescriptorSets2,
    cmd_push_constants2_khr: PFN_vkCmdPushConstants2,
    cmd_push_descriptor_set2_khr: Option<PFN_vkCmdPushDescriptorSet2>,
    cmd_push_descriptor_set_with_template2_khr: Option<PFN_vkCmdPushDescriptorSetWithTemplate2>,
    cmd_set_descriptor_buffer_offsets2_ext: Option<PFN_vkCmdSetDescriptorBufferOffsets2EXT>,
    cmd_bind_descriptor_buffer_embedded_samplers2_ext:
        Option<PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_bind_descriptor_sets2_khr: transmute(
                    load(c"vkCmdBindDescriptorSets2KHR").ok_or(LoadingError)?,
                ),
                cmd_push_constants2_khr: transmute(
                    load(c"vkCmdPushConstants2KHR").ok_or(LoadingError)?,
                ),
                cmd_push_descriptor_set2_khr: transmute(load(c"vkCmdPushDescriptorSet2KHR")),
                cmd_push_descriptor_set_with_template2_khr: transmute(load(
                    c"vkCmdPushDescriptorSetWithTemplate2KHR",
                )),
                cmd_set_descriptor_buffer_offsets2_ext: transmute(load(
                    c"vkCmdSetDescriptorBufferOffsets2EXT",
                )),
                cmd_bind_descriptor_buffer_embedded_samplers2_ext: transmute(load(
                    c"vkCmdBindDescriptorBufferEmbeddedSamplers2EXT",
                )),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_bind_descriptor_sets2_khr(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo<'_>,
    ) {
        unsafe { (self.cmd_bind_descriptor_sets2_khr)(command_buffer, bind_descriptor_sets_info) }
    }
    pub unsafe fn cmd_push_constants2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_constants_info: &PushConstantsInfo<'_>,
    ) {
        unsafe { (self.cmd_push_constants2_khr)(command_buffer, push_constants_info) }
    }
    pub unsafe fn cmd_push_descriptor_set2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo<'_>,
    ) {
        unsafe {
            (self.cmd_push_descriptor_set2_khr.unwrap())(command_buffer, push_descriptor_set_info)
        }
    }
    pub unsafe fn cmd_push_descriptor_set_with_template2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo<'_>,
    ) {
        unsafe {
            (self.cmd_push_descriptor_set_with_template2_khr.unwrap())(
                command_buffer,
                push_descriptor_set_with_template_info,
            )
        }
    }
    pub unsafe fn cmd_set_descriptor_buffer_offsets2_ext(
        &self,
        command_buffer: CommandBuffer,
        set_descriptor_buffer_offsets_info: &SetDescriptorBufferOffsetsInfoEXT<'_>,
    ) {
        unsafe {
            (self.cmd_set_descriptor_buffer_offsets2_ext.unwrap())(
                command_buffer,
                set_descriptor_buffer_offsets_info,
            )
        }
    }
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers2_ext(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_buffer_embedded_samplers_info: &BindDescriptorBufferEmbeddedSamplersInfoEXT<
            '_,
        >,
    ) {
        unsafe {
            (self
                .cmd_bind_descriptor_buffer_embedded_samplers2_ext
                .unwrap())(
                command_buffer,
                bind_descriptor_buffer_embedded_samplers_info,
            )
        }
    }
}
