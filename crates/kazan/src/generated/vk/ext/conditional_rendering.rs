#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ConditionalRenderingBeginInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: Buffer,
        pub offset: DeviceSize,
        pub flags: ConditionalRenderingFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ConditionalRenderingBeginInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CONDITIONAL_RENDERING_BEGIN_INFO_EXT;
    }
    impl Default for ConditionalRenderingBeginInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                buffer: Default::default(),
                offset: Default::default(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ConditionalRenderingBeginInfoEXT<'a> {
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }
        pub fn flags(mut self, flags: ConditionalRenderingFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CommandBufferInheritanceConditionalRenderingInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub conditional_rendering_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CommandBufferInheritanceConditionalRenderingInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT;
    }
    unsafe impl<'a> Extends<CommandBufferInheritanceInfo<'a>>
        for CommandBufferInheritanceConditionalRenderingInfoEXT<'a>
    {
    }
    impl Default for CommandBufferInheritanceConditionalRenderingInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                conditional_rendering_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CommandBufferInheritanceConditionalRenderingInfoEXT<'a> {
        pub fn conditional_rendering_enable(
            mut self,
            conditional_rendering_enable: Bool32,
        ) -> Self {
            self.conditional_rendering_enable = conditional_rendering_enable;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceConditionalRenderingFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub conditional_rendering: Bool32,
        pub inherited_conditional_rendering: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceConditionalRenderingFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceConditionalRenderingFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceConditionalRenderingFeaturesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceConditionalRenderingFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                conditional_rendering: Default::default(),
                inherited_conditional_rendering: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceConditionalRenderingFeaturesEXT<'a> {
        pub fn conditional_rendering(mut self, conditional_rendering: Bool32) -> Self {
            self.conditional_rendering = conditional_rendering;
            self
        }
        pub fn inherited_conditional_rendering(
            mut self,
            inherited_conditional_rendering: Bool32,
        ) -> Self {
            self.inherited_conditional_rendering = inherited_conditional_rendering;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ConditionalRenderingFlagsEXT(Flags);
    vk_bitflags_wrapped!(ConditionalRenderingFlagsEXT, Flags);
    impl ConditionalRenderingFlagsEXT {
        pub const INVERTED_EXT: Self = Self(ConditionalRenderingFlagBitsEXT::INVERTED_EXT.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ConditionalRenderingFlagBitsEXT(u32);
    impl ConditionalRenderingFlagBitsEXT {
        pub const INVERTED_EXT: Self = Self(1 << 0);
    }
    pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT<'_>,
    );
    pub type PFN_vkCmdEndConditionalRenderingEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer);
}
pub struct DeviceFn {
    cmd_begin_conditional_rendering_ext: PFN_vkCmdBeginConditionalRenderingEXT,
    cmd_end_conditional_rendering_ext: PFN_vkCmdEndConditionalRenderingEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_begin_conditional_rendering_ext: transmute(
                    load(c"vkCmdBeginConditionalRenderingEXT").ok_or(LoadingError)?,
                ),
                cmd_end_conditional_rendering_ext: transmute(
                    load(c"vkCmdEndConditionalRenderingEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_begin_conditional_rendering_ext(
        &self,
        command_buffer: CommandBuffer,
        conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT<'_>,
    ) {
        unsafe {
            (self.cmd_begin_conditional_rendering_ext)(command_buffer, conditional_rendering_begin)
        }
    }
    pub unsafe fn cmd_end_conditional_rendering_ext(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_conditional_rendering_ext)(command_buffer) }
    }
}
