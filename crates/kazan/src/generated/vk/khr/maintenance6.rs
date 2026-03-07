//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_maintenance6.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_maintenance6";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance6FeaturesKHR.html>
    pub type PhysicalDeviceMaintenance6FeaturesKHR<'a> = PhysicalDeviceMaintenance6Features<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance6PropertiesKHR.html>
    pub type PhysicalDeviceMaintenance6PropertiesKHR<'a> = PhysicalDeviceMaintenance6Properties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindMemoryStatusKHR.html>
    pub type BindMemoryStatusKHR<'a> = BindMemoryStatus<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindDescriptorSetsInfoKHR.html>
    pub type BindDescriptorSetsInfoKHR<'a> = BindDescriptorSetsInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPushConstantsInfoKHR.html>
    pub type PushConstantsInfoKHR<'a> = PushConstantsInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPushDescriptorSetInfoKHR.html>
    pub type PushDescriptorSetInfoKHR<'a> = PushDescriptorSetInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPushDescriptorSetWithTemplateInfoKHR.html>
    pub type PushDescriptorSetWithTemplateInfoKHR<'a> = PushDescriptorSetWithTemplateInfo<'a>;
    pub type PFN_vkCmdBindDescriptorSets2KHR = PFN_vkCmdBindDescriptorSets2;
    pub type PFN_vkCmdPushConstants2KHR = PFN_vkCmdPushConstants2;
    pub type PFN_vkCmdPushDescriptorSet2KHR = PFN_vkCmdPushDescriptorSet2;
    pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = PFN_vkCmdPushDescriptorSetWithTemplate2;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSetDescriptorBufferOffsetsInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for SetDescriptorBufferOffsetsInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SetDescriptorBufferOffsetsInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stage_flags", &self.stage_flags)
                .field("layout", &self.layout)
                .field("first_set", &self.first_set)
                .field("set_count", &self.set_count)
                .field("p_buffer_indices", &self.p_buffer_indices)
                .field("p_offsets", &self.p_offsets)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SetDescriptorBufferOffsetsInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT;
    }

    impl Default for SetDescriptorBufferOffsetsInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                stage_flags: Default::default(),
                layout: Default::default(),
                first_set: Default::default(),
                set_count: Default::default(),
                p_buffer_indices: ptr::null(),
                p_offsets: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SetDescriptorBufferOffsetsInfoEXT<'a> {
        #[inline]
        pub fn stage_flags(mut self, stage_flags: ShaderStageFlags) -> Self {
            self.stage_flags = stage_flags;
            self
        }

        #[inline]
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }

        #[inline]
        pub fn first_set(mut self, first_set: u32) -> Self {
            self.first_set = first_set;
            self
        }

        #[inline]
        pub fn sets(mut self, buffer_indices: &'a [u32], offsets: &'a [DeviceSize]) -> Self {
            self.set_count = buffer_indices.len().try_into().unwrap();
            assert_eq!(offsets.len(), self.set_count as usize);
            self.p_buffer_indices = buffer_indices.as_ptr();
            self.p_offsets = offsets.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindDescriptorBufferEmbeddedSamplersInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindDescriptorBufferEmbeddedSamplersInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stage_flags: ShaderStageFlags,
        pub layout: PipelineLayout,
        pub set: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindDescriptorBufferEmbeddedSamplersInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindDescriptorBufferEmbeddedSamplersInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stage_flags", &self.stage_flags)
                .field("layout", &self.layout)
                .field("set", &self.set)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindDescriptorBufferEmbeddedSamplersInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT;
    }

    impl Default for BindDescriptorBufferEmbeddedSamplersInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                stage_flags: Default::default(),
                layout: Default::default(),
                set: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindDescriptorBufferEmbeddedSamplersInfoEXT<'a> {
        #[inline]
        pub fn stage_flags(mut self, stage_flags: ShaderStageFlags) -> Self {
            self.stage_flags = stage_flags;
            self
        }

        #[inline]
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }

        #[inline]
        pub fn set(mut self, set: u32) -> Self {
            self.set = set;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDescriptorBufferOffsets2EXT.html>
    pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html>
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
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_bind_descriptor_sets2_khr: transmute(
                    load(c"vkCmdBindDescriptorSets2KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_push_constants2_khr: transmute(
                    load(c"vkCmdPushConstants2KHR").ok_or(MissingEntryPointError)?,
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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindDescriptorSets2KHR.html>
    #[inline]
    pub unsafe fn cmd_bind_descriptor_sets2_khr(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo<'_>,
    ) {
        unsafe { (self.cmd_bind_descriptor_sets2_khr)(command_buffer, bind_descriptor_sets_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushConstants2KHR.html>
    #[inline]
    pub unsafe fn cmd_push_constants2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_constants_info: &PushConstantsInfo<'_>,
    ) {
        unsafe { (self.cmd_push_constants2_khr)(command_buffer, push_constants_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSet2KHR.html>
    #[inline]
    pub unsafe fn cmd_push_descriptor_set2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo<'_>,
    ) {
        unsafe {
            (self.cmd_push_descriptor_set2_khr.unwrap())(command_buffer, push_descriptor_set_info)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSetWithTemplate2KHR.html>
    #[inline]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDescriptorBufferOffsets2EXT.html>
    #[inline]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html>
    #[inline]
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
