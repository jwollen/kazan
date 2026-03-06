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
    handle_nondispatchable!(
        IndirectCommandsLayoutEXT,
        INDIRECT_COMMANDS_LAYOUT_EXT,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutEXT.html>"
    );
    handle_nondispatchable!(
        IndirectExecutionSetEXT,
        INDIRECT_EXECUTION_SET_EXT,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectExecutionSetEXT.html>"
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_generated_commands: Bool32,
        pub dynamic_generated_pipeline_layout: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                device_generated_commands: Default::default(),
                dynamic_generated_pipeline_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'a> {
        pub fn device_generated_commands(mut self, device_generated_commands: bool) -> Self {
            self.device_generated_commands = device_generated_commands.into();
            self
        }
        pub fn dynamic_generated_pipeline_layout(
            mut self,
            dynamic_generated_pipeline_layout: bool,
        ) -> Self {
            self.dynamic_generated_pipeline_layout = dynamic_generated_pipeline_layout.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_indirect_pipeline_count: u32,
        pub max_indirect_shader_object_count: u32,
        pub max_indirect_sequence_count: u32,
        pub max_indirect_commands_token_count: u32,
        pub max_indirect_commands_token_offset: u32,
        pub max_indirect_commands_indirect_stride: u32,
        pub supported_indirect_commands_input_modes: IndirectCommandsInputModeFlagsEXT,
        pub supported_indirect_commands_shader_stages: ShaderStageFlags,
        pub supported_indirect_commands_shader_stages_pipeline_binding: ShaderStageFlags,
        pub supported_indirect_commands_shader_stages_shader_binding: ShaderStageFlags,
        pub device_generated_commands_transform_feedback: Bool32,
        pub device_generated_commands_multi_draw_indirect_count: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDeviceGeneratedCommandsPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceDeviceGeneratedCommandsPropertiesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceDeviceGeneratedCommandsPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_indirect_pipeline_count: Default::default(),
                max_indirect_shader_object_count: Default::default(),
                max_indirect_sequence_count: Default::default(),
                max_indirect_commands_token_count: Default::default(),
                max_indirect_commands_token_offset: Default::default(),
                max_indirect_commands_indirect_stride: Default::default(),
                supported_indirect_commands_input_modes: Default::default(),
                supported_indirect_commands_shader_stages: Default::default(),
                supported_indirect_commands_shader_stages_pipeline_binding: Default::default(),
                supported_indirect_commands_shader_stages_shader_binding: Default::default(),
                device_generated_commands_transform_feedback: Default::default(),
                device_generated_commands_multi_draw_indirect_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDeviceGeneratedCommandsPropertiesEXT<'a> {
        pub fn max_indirect_pipeline_count(mut self, max_indirect_pipeline_count: u32) -> Self {
            self.max_indirect_pipeline_count = max_indirect_pipeline_count;
            self
        }
        pub fn max_indirect_shader_object_count(
            mut self,
            max_indirect_shader_object_count: u32,
        ) -> Self {
            self.max_indirect_shader_object_count = max_indirect_shader_object_count;
            self
        }
        pub fn max_indirect_sequence_count(mut self, max_indirect_sequence_count: u32) -> Self {
            self.max_indirect_sequence_count = max_indirect_sequence_count;
            self
        }
        pub fn max_indirect_commands_token_count(
            mut self,
            max_indirect_commands_token_count: u32,
        ) -> Self {
            self.max_indirect_commands_token_count = max_indirect_commands_token_count;
            self
        }
        pub fn max_indirect_commands_token_offset(
            mut self,
            max_indirect_commands_token_offset: u32,
        ) -> Self {
            self.max_indirect_commands_token_offset = max_indirect_commands_token_offset;
            self
        }
        pub fn max_indirect_commands_indirect_stride(
            mut self,
            max_indirect_commands_indirect_stride: u32,
        ) -> Self {
            self.max_indirect_commands_indirect_stride = max_indirect_commands_indirect_stride;
            self
        }
        pub fn supported_indirect_commands_input_modes(
            mut self,
            supported_indirect_commands_input_modes: IndirectCommandsInputModeFlagsEXT,
        ) -> Self {
            self.supported_indirect_commands_input_modes = supported_indirect_commands_input_modes;
            self
        }
        pub fn supported_indirect_commands_shader_stages(
            mut self,
            supported_indirect_commands_shader_stages: ShaderStageFlags,
        ) -> Self {
            self.supported_indirect_commands_shader_stages =
                supported_indirect_commands_shader_stages;
            self
        }
        pub fn supported_indirect_commands_shader_stages_pipeline_binding(
            mut self,
            supported_indirect_commands_shader_stages_pipeline_binding: ShaderStageFlags,
        ) -> Self {
            self.supported_indirect_commands_shader_stages_pipeline_binding =
                supported_indirect_commands_shader_stages_pipeline_binding;
            self
        }
        pub fn supported_indirect_commands_shader_stages_shader_binding(
            mut self,
            supported_indirect_commands_shader_stages_shader_binding: ShaderStageFlags,
        ) -> Self {
            self.supported_indirect_commands_shader_stages_shader_binding =
                supported_indirect_commands_shader_stages_shader_binding;
            self
        }
        pub fn device_generated_commands_transform_feedback(
            mut self,
            device_generated_commands_transform_feedback: bool,
        ) -> Self {
            self.device_generated_commands_transform_feedback =
                device_generated_commands_transform_feedback.into();
            self
        }
        pub fn device_generated_commands_multi_draw_indirect_count(
            mut self,
            device_generated_commands_multi_draw_indirect_count: bool,
        ) -> Self {
            self.device_generated_commands_multi_draw_indirect_count =
                device_generated_commands_multi_draw_indirect_count.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeneratedCommandsPipelineInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GeneratedCommandsPipelineInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline: Pipeline,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for GeneratedCommandsPipelineInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GENERATED_COMMANDS_PIPELINE_INFO_EXT;
    }
    unsafe impl<'a> Extends<GeneratedCommandsInfoEXT<'a>> for GeneratedCommandsPipelineInfoEXT<'a> {}
    unsafe impl<'a> Extends<GeneratedCommandsMemoryRequirementsInfoEXT<'a>>
        for GeneratedCommandsPipelineInfoEXT<'a>
    {
    }
    impl Default for GeneratedCommandsPipelineInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                pipeline: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> GeneratedCommandsPipelineInfoEXT<'a> {
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeneratedCommandsShaderInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GeneratedCommandsShaderInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_count: u32,
        pub p_shaders: *const ShaderEXT,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for GeneratedCommandsShaderInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GENERATED_COMMANDS_SHADER_INFO_EXT;
    }
    unsafe impl<'a> Extends<GeneratedCommandsInfoEXT<'a>> for GeneratedCommandsShaderInfoEXT<'a> {}
    unsafe impl<'a> Extends<GeneratedCommandsMemoryRequirementsInfoEXT<'a>>
        for GeneratedCommandsShaderInfoEXT<'a>
    {
    }
    impl Default for GeneratedCommandsShaderInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_count: Default::default(),
                p_shaders: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> GeneratedCommandsShaderInfoEXT<'a> {
        pub fn shaders(mut self, shaders: &'a [ShaderEXT]) -> Self {
            self.shader_count = shaders.len().try_into().unwrap();
            self.p_shaders = shaders.as_ptr();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeneratedCommandsMemoryRequirementsInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GeneratedCommandsMemoryRequirementsInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub indirect_execution_set: IndirectExecutionSetEXT,
        pub indirect_commands_layout: IndirectCommandsLayoutEXT,
        pub max_sequence_count: u32,
        pub max_draw_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for GeneratedCommandsMemoryRequirementsInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT;
    }
    impl Default for GeneratedCommandsMemoryRequirementsInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                indirect_execution_set: Default::default(),
                indirect_commands_layout: Default::default(),
                max_sequence_count: Default::default(),
                max_draw_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> GeneratedCommandsMemoryRequirementsInfoEXT<'a> {
        pub fn indirect_execution_set(
            mut self,
            indirect_execution_set: IndirectExecutionSetEXT,
        ) -> Self {
            self.indirect_execution_set = indirect_execution_set;
            self
        }
        pub fn indirect_commands_layout(
            mut self,
            indirect_commands_layout: IndirectCommandsLayoutEXT,
        ) -> Self {
            self.indirect_commands_layout = indirect_commands_layout;
            self
        }
        pub fn max_sequence_count(mut self, max_sequence_count: u32) -> Self {
            self.max_sequence_count = max_sequence_count;
            self
        }
        pub fn max_draw_count(mut self, max_draw_count: u32) -> Self {
            self.max_draw_count = max_draw_count;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectExecutionSetPipelineInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IndirectExecutionSetPipelineInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub initial_pipeline: Pipeline,
        pub max_pipeline_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for IndirectExecutionSetPipelineInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT;
    }
    impl Default for IndirectExecutionSetPipelineInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                initial_pipeline: Default::default(),
                max_pipeline_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> IndirectExecutionSetPipelineInfoEXT<'a> {
        pub fn initial_pipeline(mut self, initial_pipeline: Pipeline) -> Self {
            self.initial_pipeline = initial_pipeline;
            self
        }
        pub fn max_pipeline_count(mut self, max_pipeline_count: u32) -> Self {
            self.max_pipeline_count = max_pipeline_count;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectExecutionSetShaderLayoutInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IndirectExecutionSetShaderLayoutInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub set_layout_count: u32,
        pub p_set_layouts: *const DescriptorSetLayout,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for IndirectExecutionSetShaderLayoutInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT;
    }
    impl Default for IndirectExecutionSetShaderLayoutInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                set_layout_count: Default::default(),
                p_set_layouts: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> IndirectExecutionSetShaderLayoutInfoEXT<'a> {
        pub fn set_layouts(mut self, set_layouts: &'a [DescriptorSetLayout]) -> Self {
            self.set_layout_count = set_layouts.len().try_into().unwrap();
            self.p_set_layouts = set_layouts.as_ptr();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectExecutionSetShaderInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IndirectExecutionSetShaderInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub shader_count: u32,
        pub p_initial_shaders: *const ShaderEXT,
        pub p_set_layout_infos: *const IndirectExecutionSetShaderLayoutInfoEXT<'a>,
        pub max_shader_count: u32,
        pub push_constant_range_count: u32,
        pub p_push_constant_ranges: *const PushConstantRange,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for IndirectExecutionSetShaderInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::INDIRECT_EXECUTION_SET_SHADER_INFO_EXT;
    }
    impl Default for IndirectExecutionSetShaderInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                shader_count: Default::default(),
                p_initial_shaders: core::ptr::null(),
                p_set_layout_infos: core::ptr::null(),
                max_shader_count: Default::default(),
                push_constant_range_count: Default::default(),
                p_push_constant_ranges: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> IndirectExecutionSetShaderInfoEXT<'a> {
        pub fn initial_shaders(mut self, initial_shaders: &'a [ShaderEXT]) -> Self {
            self.shader_count = initial_shaders.len().try_into().unwrap();
            self.p_initial_shaders = initial_shaders.as_ptr();
            self
        }
        pub fn set_layout_infos(
            mut self,
            set_layout_infos: &'a [IndirectExecutionSetShaderLayoutInfoEXT<'a>],
        ) -> Self {
            self.shader_count = set_layout_infos.len().try_into().unwrap();
            self.p_set_layout_infos = set_layout_infos.as_ptr();
            self
        }
        pub fn max_shader_count(mut self, max_shader_count: u32) -> Self {
            self.max_shader_count = max_shader_count;
            self
        }
        pub fn push_constant_ranges(
            mut self,
            push_constant_ranges: &'a [PushConstantRange],
        ) -> Self {
            self.push_constant_range_count = push_constant_ranges.len().try_into().unwrap();
            self.p_push_constant_ranges = push_constant_ranges.as_ptr();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectExecutionSetCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IndirectExecutionSetCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: IndirectExecutionSetInfoTypeEXT,
        pub info: IndirectExecutionSetInfoEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for IndirectExecutionSetCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::INDIRECT_EXECUTION_SET_CREATE_INFO_EXT;
    }
    impl Default for IndirectExecutionSetCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                ty: Default::default(),
                info: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> IndirectExecutionSetCreateInfoEXT<'a> {
        pub fn ty(mut self, ty: IndirectExecutionSetInfoTypeEXT) -> Self {
            self.ty = ty;
            self
        }
        pub fn info(mut self, info: IndirectExecutionSetInfoEXT<'a>) -> Self {
            self.info = info;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeneratedCommandsInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GeneratedCommandsInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub shader_stages: ShaderStageFlags,
        pub indirect_execution_set: IndirectExecutionSetEXT,
        pub indirect_commands_layout: IndirectCommandsLayoutEXT,
        pub indirect_address: DeviceAddress,
        pub indirect_address_size: DeviceSize,
        pub preprocess_address: DeviceAddress,
        pub preprocess_size: DeviceSize,
        pub max_sequence_count: u32,
        pub sequence_count_address: DeviceAddress,
        pub max_draw_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for GeneratedCommandsInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GENERATED_COMMANDS_INFO_EXT;
    }
    impl Default for GeneratedCommandsInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                shader_stages: Default::default(),
                indirect_execution_set: Default::default(),
                indirect_commands_layout: Default::default(),
                indirect_address: Default::default(),
                indirect_address_size: Default::default(),
                preprocess_address: Default::default(),
                preprocess_size: Default::default(),
                max_sequence_count: Default::default(),
                sequence_count_address: Default::default(),
                max_draw_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> GeneratedCommandsInfoEXT<'a> {
        pub fn shader_stages(mut self, shader_stages: ShaderStageFlags) -> Self {
            self.shader_stages = shader_stages;
            self
        }
        pub fn indirect_execution_set(
            mut self,
            indirect_execution_set: IndirectExecutionSetEXT,
        ) -> Self {
            self.indirect_execution_set = indirect_execution_set;
            self
        }
        pub fn indirect_commands_layout(
            mut self,
            indirect_commands_layout: IndirectCommandsLayoutEXT,
        ) -> Self {
            self.indirect_commands_layout = indirect_commands_layout;
            self
        }
        pub fn indirect_address(mut self, indirect_address: DeviceAddress) -> Self {
            self.indirect_address = indirect_address;
            self
        }
        pub fn indirect_address_size(mut self, indirect_address_size: DeviceSize) -> Self {
            self.indirect_address_size = indirect_address_size;
            self
        }
        pub fn preprocess_address(mut self, preprocess_address: DeviceAddress) -> Self {
            self.preprocess_address = preprocess_address;
            self
        }
        pub fn preprocess_size(mut self, preprocess_size: DeviceSize) -> Self {
            self.preprocess_size = preprocess_size;
            self
        }
        pub fn max_sequence_count(mut self, max_sequence_count: u32) -> Self {
            self.max_sequence_count = max_sequence_count;
            self
        }
        pub fn sequence_count_address(mut self, sequence_count_address: DeviceAddress) -> Self {
            self.sequence_count_address = sequence_count_address;
            self
        }
        pub fn max_draw_count(mut self, max_draw_count: u32) -> Self {
            self.max_draw_count = max_draw_count;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWriteIndirectExecutionSetPipelineEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct WriteIndirectExecutionSetPipelineEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub index: u32,
        pub pipeline: Pipeline,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for WriteIndirectExecutionSetPipelineEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT;
    }
    impl Default for WriteIndirectExecutionSetPipelineEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                index: Default::default(),
                pipeline: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> WriteIndirectExecutionSetPipelineEXT<'a> {
        pub fn index(mut self, index: u32) -> Self {
            self.index = index;
            self
        }
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWriteIndirectExecutionSetShaderEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct WriteIndirectExecutionSetShaderEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub index: u32,
        pub shader: ShaderEXT,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for WriteIndirectExecutionSetShaderEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT;
    }
    impl Default for WriteIndirectExecutionSetShaderEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                index: Default::default(),
                shader: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> WriteIndirectExecutionSetShaderEXT<'a> {
        pub fn index(mut self, index: u32) -> Self {
            self.index = index;
            self
        }
        pub fn shader(mut self, shader: ShaderEXT) -> Self {
            self.shader = shader;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IndirectCommandsLayoutCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: IndirectCommandsLayoutUsageFlagsEXT,
        pub shader_stages: ShaderStageFlags,
        pub indirect_stride: u32,
        pub pipeline_layout: PipelineLayout,
        pub token_count: u32,
        pub p_tokens: *const IndirectCommandsLayoutTokenEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for IndirectCommandsLayoutCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT;
    }
    impl Default for IndirectCommandsLayoutCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                shader_stages: Default::default(),
                indirect_stride: Default::default(),
                pipeline_layout: Default::default(),
                token_count: Default::default(),
                p_tokens: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> IndirectCommandsLayoutCreateInfoEXT<'a> {
        pub fn flags(mut self, flags: IndirectCommandsLayoutUsageFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
        pub fn shader_stages(mut self, shader_stages: ShaderStageFlags) -> Self {
            self.shader_stages = shader_stages;
            self
        }
        pub fn indirect_stride(mut self, indirect_stride: u32) -> Self {
            self.indirect_stride = indirect_stride;
            self
        }
        pub fn pipeline_layout(mut self, pipeline_layout: PipelineLayout) -> Self {
            self.pipeline_layout = pipeline_layout;
            self
        }
        pub fn tokens(mut self, tokens: &'a [IndirectCommandsLayoutTokenEXT<'a>]) -> Self {
            self.token_count = tokens.len().try_into().unwrap();
            self.p_tokens = tokens.as_ptr();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutTokenEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IndirectCommandsLayoutTokenEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: IndirectCommandsTokenTypeEXT,
        pub data: IndirectCommandsTokenDataEXT<'a>,
        pub offset: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for IndirectCommandsLayoutTokenEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT;
    }
    impl Default for IndirectCommandsLayoutTokenEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                ty: Default::default(),
                data: Default::default(),
                offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> IndirectCommandsLayoutTokenEXT<'a> {
        pub fn ty(mut self, ty: IndirectCommandsTokenTypeEXT) -> Self {
            self.ty = ty;
            self
        }
        pub fn data(mut self, data: IndirectCommandsTokenDataEXT<'a>) -> Self {
            self.data = data;
            self
        }
        pub fn offset(mut self, offset: u32) -> Self {
            self.offset = offset;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDrawIndirectCountIndirectCommandEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct DrawIndirectCountIndirectCommandEXT {
        pub buffer_address: DeviceAddress,
        pub stride: u32,
        pub command_count: u32,
    }
    impl DrawIndirectCountIndirectCommandEXT {
        pub fn buffer_address(mut self, buffer_address: DeviceAddress) -> Self {
            self.buffer_address = buffer_address;
            self
        }
        pub fn stride(mut self, stride: u32) -> Self {
            self.stride = stride;
            self
        }
        pub fn command_count(mut self, command_count: u32) -> Self {
            self.command_count = command_count;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsVertexBufferTokenEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct IndirectCommandsVertexBufferTokenEXT {
        pub vertex_binding_unit: u32,
    }
    impl IndirectCommandsVertexBufferTokenEXT {
        pub fn vertex_binding_unit(mut self, vertex_binding_unit: u32) -> Self {
            self.vertex_binding_unit = vertex_binding_unit;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindVertexBufferIndirectCommandEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct BindVertexBufferIndirectCommandEXT {
        pub buffer_address: DeviceAddress,
        pub size: u32,
        pub stride: u32,
    }
    impl BindVertexBufferIndirectCommandEXT {
        pub fn buffer_address(mut self, buffer_address: DeviceAddress) -> Self {
            self.buffer_address = buffer_address;
            self
        }
        pub fn size(mut self, size: u32) -> Self {
            self.size = size;
            self
        }
        pub fn stride(mut self, stride: u32) -> Self {
            self.stride = stride;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsIndexBufferTokenEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct IndirectCommandsIndexBufferTokenEXT {
        pub mode: IndirectCommandsInputModeFlagBitsEXT,
    }
    impl IndirectCommandsIndexBufferTokenEXT {
        pub fn mode(mut self, mode: IndirectCommandsInputModeFlagBitsEXT) -> Self {
            self.mode = mode;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindIndexBufferIndirectCommandEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct BindIndexBufferIndirectCommandEXT {
        pub buffer_address: DeviceAddress,
        pub size: u32,
        pub index_type: IndexType,
    }
    impl BindIndexBufferIndirectCommandEXT {
        pub fn buffer_address(mut self, buffer_address: DeviceAddress) -> Self {
            self.buffer_address = buffer_address;
            self
        }
        pub fn size(mut self, size: u32) -> Self {
            self.size = size;
            self
        }
        pub fn index_type(mut self, index_type: IndexType) -> Self {
            self.index_type = index_type;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsPushConstantTokenEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct IndirectCommandsPushConstantTokenEXT {
        pub update_range: PushConstantRange,
    }
    impl IndirectCommandsPushConstantTokenEXT {
        pub fn update_range(mut self, update_range: PushConstantRange) -> Self {
            self.update_range = update_range;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsExecutionSetTokenEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct IndirectCommandsExecutionSetTokenEXT {
        pub ty: IndirectExecutionSetInfoTypeEXT,
        pub shader_stages: ShaderStageFlags,
    }
    impl IndirectCommandsExecutionSetTokenEXT {
        pub fn ty(mut self, ty: IndirectExecutionSetInfoTypeEXT) -> Self {
            self.ty = ty;
            self
        }
        pub fn shader_stages(mut self, shader_stages: ShaderStageFlags) -> Self {
            self.shader_stages = shader_stages;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectExecutionSetInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union IndirectExecutionSetInfoEXT<'a> {
        pub p_pipeline_info: *const IndirectExecutionSetPipelineInfoEXT<'a>,
        pub p_shader_info: *const IndirectExecutionSetShaderInfoEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for IndirectExecutionSetInfoEXT<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsTokenDataEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union IndirectCommandsTokenDataEXT<'a> {
        pub p_push_constant: *const IndirectCommandsPushConstantTokenEXT,
        pub p_vertex_buffer: *const IndirectCommandsVertexBufferTokenEXT,
        pub p_index_buffer: *const IndirectCommandsIndexBufferTokenEXT,
        pub p_execution_set: *const IndirectCommandsExecutionSetTokenEXT,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for IndirectCommandsTokenDataEXT<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectExecutionSetInfoTypeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct IndirectExecutionSetInfoTypeEXT(i32);
    impl IndirectExecutionSetInfoTypeEXT {
        pub const PIPELINES_EXT: Self = Self(0);
        pub const SHADER_OBJECTS_EXT: Self = Self(1);
    }
    impl fmt::Debug for IndirectExecutionSetInfoTypeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PIPELINES_EXT => Some("PIPELINES_EXT"),
                Self::SHADER_OBJECTS_EXT => Some("SHADER_OBJECTS_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsTokenTypeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct IndirectCommandsTokenTypeEXT(i32);
    impl IndirectCommandsTokenTypeEXT {
        pub const EXECUTION_SET_EXT: Self = Self(0);
        pub const PUSH_CONSTANT_EXT: Self = Self(1);
        pub const SEQUENCE_INDEX_EXT: Self = Self(2);
        pub const INDEX_BUFFER_EXT: Self = Self(3);
        pub const VERTEX_BUFFER_EXT: Self = Self(4);
        pub const DRAW_INDEXED_EXT: Self = Self(5);
        pub const DRAW_EXT: Self = Self(6);
        pub const DRAW_INDEXED_COUNT_EXT: Self = Self(7);
        pub const DRAW_COUNT_EXT: Self = Self(8);
        pub const DISPATCH_EXT: Self = Self(9);
        // VK_EXT_descriptor_heap
        pub const PUSH_DATA_EXT: Self = Self(1000135000);
        pub const PUSH_DATA_SEQUENCE_INDEX_EXT: Self = Self(1000135001);
        // VK_EXT_mesh_shader
        pub const DRAW_MESH_TASKS_EXT: Self = Self(1000328000);
        pub const DRAW_MESH_TASKS_COUNT_EXT: Self = Self(1000328001);
        // VK_KHR_ray_tracing_maintenance1
        pub const TRACE_RAYS2_EXT: Self = Self(1000386004);
        // VK_NV_mesh_shader
        pub const DRAW_MESH_TASKS_NV_EXT: Self = Self(1000202002);
        pub const DRAW_MESH_TASKS_COUNT_NV_EXT: Self = Self(1000202003);
    }
    impl fmt::Debug for IndirectCommandsTokenTypeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::EXECUTION_SET_EXT => Some("EXECUTION_SET_EXT"),
                Self::PUSH_CONSTANT_EXT => Some("PUSH_CONSTANT_EXT"),
                Self::SEQUENCE_INDEX_EXT => Some("SEQUENCE_INDEX_EXT"),
                Self::INDEX_BUFFER_EXT => Some("INDEX_BUFFER_EXT"),
                Self::VERTEX_BUFFER_EXT => Some("VERTEX_BUFFER_EXT"),
                Self::DRAW_INDEXED_EXT => Some("DRAW_INDEXED_EXT"),
                Self::DRAW_EXT => Some("DRAW_EXT"),
                Self::DRAW_INDEXED_COUNT_EXT => Some("DRAW_INDEXED_COUNT_EXT"),
                Self::DRAW_COUNT_EXT => Some("DRAW_COUNT_EXT"),
                Self::DISPATCH_EXT => Some("DISPATCH_EXT"),
                Self::PUSH_DATA_EXT => Some("PUSH_DATA_EXT"),
                Self::PUSH_DATA_SEQUENCE_INDEX_EXT => Some("PUSH_DATA_SEQUENCE_INDEX_EXT"),
                Self::DRAW_MESH_TASKS_EXT => Some("DRAW_MESH_TASKS_EXT"),
                Self::DRAW_MESH_TASKS_COUNT_EXT => Some("DRAW_MESH_TASKS_COUNT_EXT"),
                Self::TRACE_RAYS2_EXT => Some("TRACE_RAYS2_EXT"),
                Self::DRAW_MESH_TASKS_NV_EXT => Some("DRAW_MESH_TASKS_NV_EXT"),
                Self::DRAW_MESH_TASKS_COUNT_NV_EXT => Some("DRAW_MESH_TASKS_COUNT_NV_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutUsageFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct IndirectCommandsLayoutUsageFlagsEXT(Flags);
    vk_bitflags_wrapped!(IndirectCommandsLayoutUsageFlagsEXT, Flags);
    impl IndirectCommandsLayoutUsageFlagsEXT {
        pub const EXPLICIT_PREPROCESS_EXT: Self =
            Self(IndirectCommandsLayoutUsageFlagBitsEXT::EXPLICIT_PREPROCESS_EXT.0);
        pub const UNORDERED_SEQUENCES_EXT: Self =
            Self(IndirectCommandsLayoutUsageFlagBitsEXT::UNORDERED_SEQUENCES_EXT.0);
    }
    impl fmt::Debug for IndirectCommandsLayoutUsageFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    IndirectCommandsLayoutUsageFlagsEXT::EXPLICIT_PREPROCESS_EXT.0,
                    "EXPLICIT_PREPROCESS_EXT",
                ),
                (
                    IndirectCommandsLayoutUsageFlagsEXT::UNORDERED_SEQUENCES_EXT.0,
                    "UNORDERED_SEQUENCES_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutUsageFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct IndirectCommandsLayoutUsageFlagBitsEXT(u32);
    impl IndirectCommandsLayoutUsageFlagBitsEXT {
        pub const EXPLICIT_PREPROCESS_EXT: Self = Self(1 << 0);
        pub const UNORDERED_SEQUENCES_EXT: Self = Self(1 << 1);
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsInputModeFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct IndirectCommandsInputModeFlagsEXT(Flags);
    vk_bitflags_wrapped!(IndirectCommandsInputModeFlagsEXT, Flags);
    impl IndirectCommandsInputModeFlagsEXT {
        pub const VULKAN_INDEX_BUFFER_EXT: Self =
            Self(IndirectCommandsInputModeFlagBitsEXT::VULKAN_INDEX_BUFFER_EXT.0);
        pub const DXGI_INDEX_BUFFER_EXT: Self =
            Self(IndirectCommandsInputModeFlagBitsEXT::DXGI_INDEX_BUFFER_EXT.0);
    }
    impl fmt::Debug for IndirectCommandsInputModeFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    IndirectCommandsInputModeFlagsEXT::VULKAN_INDEX_BUFFER_EXT.0,
                    "VULKAN_INDEX_BUFFER_EXT",
                ),
                (
                    IndirectCommandsInputModeFlagsEXT::DXGI_INDEX_BUFFER_EXT.0,
                    "DXGI_INDEX_BUFFER_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsInputModeFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct IndirectCommandsInputModeFlagBitsEXT(u32);
    impl IndirectCommandsInputModeFlagBitsEXT {
        pub const VULKAN_INDEX_BUFFER_EXT: Self = Self(1 << 0);
        pub const DXGI_INDEX_BUFFER_EXT: Self = Self(1 << 1);
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdExecuteGeneratedCommandsEXT.html>
    pub type PFN_vkCmdExecuteGeneratedCommandsEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        p_generated_commands_info: *const GeneratedCommandsInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPreprocessGeneratedCommandsEXT.html>
    pub type PFN_vkCmdPreprocessGeneratedCommandsEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_generated_commands_info: *const GeneratedCommandsInfoEXT<'_>,
        state_command_buffer: CommandBuffer,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetGeneratedCommandsMemoryRequirementsEXT.html>
    pub type PFN_vkGetGeneratedCommandsMemoryRequirementsEXT = unsafe extern "system" fn(
        device: Device,
        p_info: *const GeneratedCommandsMemoryRequirementsInfoEXT<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateIndirectCommandsLayoutEXT.html>
    pub type PFN_vkCreateIndirectCommandsLayoutEXT = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const IndirectCommandsLayoutCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_indirect_commands_layout: *mut IndirectCommandsLayoutEXT,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyIndirectCommandsLayoutEXT.html>
    pub type PFN_vkDestroyIndirectCommandsLayoutEXT = unsafe extern "system" fn(
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateIndirectExecutionSetEXT.html>
    pub type PFN_vkCreateIndirectExecutionSetEXT = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const IndirectExecutionSetCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_indirect_execution_set: *mut IndirectExecutionSetEXT,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyIndirectExecutionSetEXT.html>
    pub type PFN_vkDestroyIndirectExecutionSetEXT = unsafe extern "system" fn(
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUpdateIndirectExecutionSetPipelineEXT.html>
    pub type PFN_vkUpdateIndirectExecutionSetPipelineEXT = unsafe extern "system" fn(
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_write_count: u32,
        p_execution_set_writes: *const WriteIndirectExecutionSetPipelineEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUpdateIndirectExecutionSetShaderEXT.html>
    pub type PFN_vkUpdateIndirectExecutionSetShaderEXT = unsafe extern "system" fn(
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_write_count: u32,
        p_execution_set_writes: *const WriteIndirectExecutionSetShaderEXT<'_>,
    );
}
pub struct DeviceFn {
    get_generated_commands_memory_requirements_ext: PFN_vkGetGeneratedCommandsMemoryRequirementsEXT,
    cmd_preprocess_generated_commands_ext: PFN_vkCmdPreprocessGeneratedCommandsEXT,
    cmd_execute_generated_commands_ext: PFN_vkCmdExecuteGeneratedCommandsEXT,
    create_indirect_commands_layout_ext: PFN_vkCreateIndirectCommandsLayoutEXT,
    destroy_indirect_commands_layout_ext: PFN_vkDestroyIndirectCommandsLayoutEXT,
    create_indirect_execution_set_ext: PFN_vkCreateIndirectExecutionSetEXT,
    destroy_indirect_execution_set_ext: PFN_vkDestroyIndirectExecutionSetEXT,
    update_indirect_execution_set_pipeline_ext: PFN_vkUpdateIndirectExecutionSetPipelineEXT,
    update_indirect_execution_set_shader_ext: PFN_vkUpdateIndirectExecutionSetShaderEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_generated_commands_memory_requirements_ext: transmute(
                    load(c"vkGetGeneratedCommandsMemoryRequirementsEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_preprocess_generated_commands_ext: transmute(
                    load(c"vkCmdPreprocessGeneratedCommandsEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_execute_generated_commands_ext: transmute(
                    load(c"vkCmdExecuteGeneratedCommandsEXT").ok_or(MissingEntryPointError)?,
                ),
                create_indirect_commands_layout_ext: transmute(
                    load(c"vkCreateIndirectCommandsLayoutEXT").ok_or(MissingEntryPointError)?,
                ),
                destroy_indirect_commands_layout_ext: transmute(
                    load(c"vkDestroyIndirectCommandsLayoutEXT").ok_or(MissingEntryPointError)?,
                ),
                create_indirect_execution_set_ext: transmute(
                    load(c"vkCreateIndirectExecutionSetEXT").ok_or(MissingEntryPointError)?,
                ),
                destroy_indirect_execution_set_ext: transmute(
                    load(c"vkDestroyIndirectExecutionSetEXT").ok_or(MissingEntryPointError)?,
                ),
                update_indirect_execution_set_pipeline_ext: transmute(
                    load(c"vkUpdateIndirectExecutionSetPipelineEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
                update_indirect_execution_set_shader_ext: transmute(
                    load(c"vkUpdateIndirectExecutionSetShaderEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetGeneratedCommandsMemoryRequirementsEXT.html>
    pub unsafe fn get_generated_commands_memory_requirements_ext(
        &self,
        device: Device,
        info: &GeneratedCommandsMemoryRequirementsInfoEXT<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe {
            (self.get_generated_commands_memory_requirements_ext)(device, info, memory_requirements)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPreprocessGeneratedCommandsEXT.html>
    pub unsafe fn cmd_preprocess_generated_commands_ext(
        &self,
        command_buffer: CommandBuffer,
        generated_commands_info: &GeneratedCommandsInfoEXT<'_>,
        state_command_buffer: CommandBuffer,
    ) {
        unsafe {
            (self.cmd_preprocess_generated_commands_ext)(
                command_buffer,
                generated_commands_info,
                state_command_buffer,
            )
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdExecuteGeneratedCommandsEXT.html>
    pub unsafe fn cmd_execute_generated_commands_ext(
        &self,
        command_buffer: CommandBuffer,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoEXT<'_>,
    ) {
        unsafe {
            (self.cmd_execute_generated_commands_ext)(
                command_buffer,
                is_preprocessed.into(),
                generated_commands_info,
            )
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateIndirectCommandsLayoutEXT.html>
    pub unsafe fn create_indirect_commands_layout_ext(
        &self,
        device: Device,
        create_info: &IndirectCommandsLayoutCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<IndirectCommandsLayoutEXT> {
        unsafe {
            let mut indirect_commands_layout = core::mem::MaybeUninit::uninit();
            let result = (self.create_indirect_commands_layout_ext)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                indirect_commands_layout.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(indirect_commands_layout.assume_init()),
                err => Err(err),
            }
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyIndirectCommandsLayoutEXT.html>
    pub unsafe fn destroy_indirect_commands_layout_ext(
        &self,
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_indirect_commands_layout_ext)(
                device,
                indirect_commands_layout,
                allocator.to_raw_ptr(),
            )
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateIndirectExecutionSetEXT.html>
    pub unsafe fn create_indirect_execution_set_ext(
        &self,
        device: Device,
        create_info: &IndirectExecutionSetCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<IndirectExecutionSetEXT> {
        unsafe {
            let mut indirect_execution_set = core::mem::MaybeUninit::uninit();
            let result = (self.create_indirect_execution_set_ext)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                indirect_execution_set.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(indirect_execution_set.assume_init()),
                err => Err(err),
            }
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyIndirectExecutionSetEXT.html>
    pub unsafe fn destroy_indirect_execution_set_ext(
        &self,
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_indirect_execution_set_ext)(
                device,
                indirect_execution_set,
                allocator.to_raw_ptr(),
            )
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUpdateIndirectExecutionSetPipelineEXT.html>
    pub unsafe fn update_indirect_execution_set_pipeline_ext(
        &self,
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetPipelineEXT<'_>],
    ) {
        unsafe {
            (self.update_indirect_execution_set_pipeline_ext)(
                device,
                indirect_execution_set,
                execution_set_writes.len().try_into().unwrap(),
                execution_set_writes.as_ptr() as _,
            )
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUpdateIndirectExecutionSetShaderEXT.html>
    pub unsafe fn update_indirect_execution_set_shader_ext(
        &self,
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetShaderEXT<'_>],
    ) {
        unsafe {
            (self.update_indirect_execution_set_shader_ext)(
                device,
                indirect_execution_set,
                execution_set_writes.len().try_into().unwrap(),
                execution_set_writes.as_ptr() as _,
            )
        }
    }
}
