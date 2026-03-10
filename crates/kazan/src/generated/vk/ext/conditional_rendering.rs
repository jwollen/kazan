//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_conditional_rendering.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_conditional_rendering";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkConditionalRenderingBeginInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ConditionalRenderingBeginInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: Buffer,
        pub offset: DeviceSize,
        pub flags: ConditionalRenderingFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ConditionalRenderingBeginInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ConditionalRenderingBeginInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("buffer", &self.buffer)
                .field("offset", &self.offset)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ConditionalRenderingBeginInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CONDITIONAL_RENDERING_BEGIN_INFO_EXT;
    }

    impl Default for ConditionalRenderingBeginInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                buffer: Default::default(),
                offset: Default::default(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ConditionalRenderingBeginInfoEXT<'a> {
        #[inline]
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }

        #[inline]
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: ConditionalRenderingFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CommandBufferInheritanceConditionalRenderingInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub conditional_rendering_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CommandBufferInheritanceConditionalRenderingInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CommandBufferInheritanceConditionalRenderingInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "conditional_rendering_enable",
                    &self.conditional_rendering_enable,
                )
                .finish()
        }
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
                p_next: ptr::null(),
                conditional_rendering_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CommandBufferInheritanceConditionalRenderingInfoEXT<'a> {
        #[inline]
        pub fn conditional_rendering_enable(mut self, conditional_rendering_enable: bool) -> Self {
            self.conditional_rendering_enable = conditional_rendering_enable.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceConditionalRenderingFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub conditional_rendering: Bool32,
        pub inherited_conditional_rendering: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceConditionalRenderingFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceConditionalRenderingFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("conditional_rendering", &self.conditional_rendering)
                .field(
                    "inherited_conditional_rendering",
                    &self.inherited_conditional_rendering,
                )
                .finish()
        }
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
                p_next: ptr::null_mut(),
                conditional_rendering: Default::default(),
                inherited_conditional_rendering: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceConditionalRenderingFeaturesEXT<'a> {
        #[inline]
        pub fn conditional_rendering(mut self, conditional_rendering: bool) -> Self {
            self.conditional_rendering = conditional_rendering.into();
            self
        }

        #[inline]
        pub fn inherited_conditional_rendering(
            mut self,
            inherited_conditional_rendering: bool,
        ) -> Self {
            self.inherited_conditional_rendering = inherited_conditional_rendering.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkConditionalRenderingFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ConditionalRenderingFlagsEXT(Flags);
    vk_bitflags_wrapped!(ConditionalRenderingFlagsEXT, Flags);

    impl ConditionalRenderingFlagsEXT {
        pub const INVERTED_EXT: Self = Self(ConditionalRenderingFlagBitsEXT::INVERTED_EXT.0);
    }

    impl fmt::Debug for ConditionalRenderingFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] =
                &[(ConditionalRenderingFlagsEXT::INVERTED_EXT.0, "INVERTED_EXT")];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkConditionalRenderingFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ConditionalRenderingFlagBitsEXT(u32);

    impl ConditionalRenderingFlagBitsEXT {
        pub const INVERTED_EXT: Self = Self(1 << 0);
    }

    impl fmt::Debug for ConditionalRenderingFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INVERTED_EXT => Some("INVERTED_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginConditionalRenderingEXT.html>
    pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndConditionalRenderingEXT.html>
    pub type PFN_vkCmdEndConditionalRenderingEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer);
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkConditionalRenderingBeginInfoEXT = ConditionalRenderingBeginInfoEXT<'static>;
    pub type VkCommandBufferInheritanceConditionalRenderingInfoEXT =
        CommandBufferInheritanceConditionalRenderingInfoEXT<'static>;
    pub type VkPhysicalDeviceConditionalRenderingFeaturesEXT =
        PhysicalDeviceConditionalRenderingFeaturesEXT<'static>;
    pub type VkConditionalRenderingFlagsEXT = ConditionalRenderingFlagsEXT;
    pub type VkConditionalRenderingFlagBitsEXT = ConditionalRenderingFlagBitsEXT;
    impl ConditionalRenderingBeginInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkConditionalRenderingBeginInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CommandBufferInheritanceConditionalRenderingInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkCommandBufferInheritanceConditionalRenderingInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceConditionalRenderingFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceConditionalRenderingFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_begin_conditional_rendering_ext: PFN_vkCmdBeginConditionalRenderingEXT,
    cmd_end_conditional_rendering_ext: PFN_vkCmdEndConditionalRenderingEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_begin_conditional_rendering_ext: transmute(
                    load(c"vkCmdBeginConditionalRenderingEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_conditional_rendering_ext: transmute(
                    load(c"vkCmdEndConditionalRenderingEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginConditionalRenderingEXT.html>
    #[inline]
    pub unsafe fn cmd_begin_conditional_rendering_ext(
        &self,
        command_buffer: CommandBuffer,
        conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT<'_>,
    ) {
        unsafe {
            (self.cmd_begin_conditional_rendering_ext)(command_buffer, conditional_rendering_begin)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndConditionalRenderingEXT.html>
    #[inline]
    pub unsafe fn cmd_end_conditional_rendering_ext(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_conditional_rendering_ext)(command_buffer) }
    }
}
