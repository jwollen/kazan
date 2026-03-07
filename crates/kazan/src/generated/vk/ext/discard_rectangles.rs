//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_discard_rectangles.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_discard_rectangles";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDiscardRectanglePropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_discard_rectangles: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDiscardRectanglePropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDiscardRectanglePropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_discard_rectangles", &self.max_discard_rectangles)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDiscardRectanglePropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceDiscardRectanglePropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceDiscardRectanglePropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_discard_rectangles: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDiscardRectanglePropertiesEXT<'a> {
        #[inline]
        pub fn max_discard_rectangles(mut self, max_discard_rectangles: u32) -> Self {
            self.max_discard_rectangles = max_discard_rectangles;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineDiscardRectangleStateCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineDiscardRectangleStateCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineDiscardRectangleStateCreateFlagsEXT,
        pub discard_rectangle_mode: DiscardRectangleModeEXT,
        pub discard_rectangle_count: u32,
        pub p_discard_rectangles: *const Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineDiscardRectangleStateCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineDiscardRectangleStateCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("discard_rectangle_mode", &self.discard_rectangle_mode)
                .field("discard_rectangle_count", &self.discard_rectangle_count)
                .field("p_discard_rectangles", &self.p_discard_rectangles)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineDiscardRectangleStateCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>>
        for PipelineDiscardRectangleStateCreateInfoEXT<'a>
    {
    }

    impl Default for PipelineDiscardRectangleStateCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                discard_rectangle_mode: Default::default(),
                discard_rectangle_count: Default::default(),
                p_discard_rectangles: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineDiscardRectangleStateCreateInfoEXT<'a> {
        #[inline]
        pub fn flags(mut self, flags: PipelineDiscardRectangleStateCreateFlagsEXT) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn discard_rectangle_mode(
            mut self,
            discard_rectangle_mode: DiscardRectangleModeEXT,
        ) -> Self {
            self.discard_rectangle_mode = discard_rectangle_mode;
            self
        }

        #[inline]
        pub fn discard_rectangles(mut self, discard_rectangles: &'a [Rect2D]) -> Self {
            self.discard_rectangle_count = discard_rectangles.len().try_into().unwrap();
            self.p_discard_rectangles = discard_rectangles.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDiscardRectangleModeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DiscardRectangleModeEXT(i32);

    impl DiscardRectangleModeEXT {
        pub const INCLUSIVE_EXT: Self = Self(0);
        pub const EXCLUSIVE_EXT: Self = Self(1);
    }

    impl fmt::Debug for DiscardRectangleModeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INCLUSIVE_EXT => Some("INCLUSIVE_EXT"),
                Self::EXCLUSIVE_EXT => Some("EXCLUSIVE_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineDiscardRectangleStateCreateFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineDiscardRectangleStateCreateFlagsEXT(Flags);
    vk_bitflags_wrapped!(PipelineDiscardRectangleStateCreateFlagsEXT, Flags);

    impl fmt::Debug for PipelineDiscardRectangleStateCreateFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDiscardRectangleEXT.html>
    pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangle_count: u32,
        p_discard_rectangles: *const Rect2D,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDiscardRectangleEnableEXT.html>
    pub type PFN_vkCmdSetDiscardRectangleEnableEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, discard_rectangle_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDiscardRectangleModeEXT.html>
    pub type PFN_vkCmdSetDiscardRectangleModeEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        discard_rectangle_mode: DiscardRectangleModeEXT,
    );
}

pub struct DeviceFn {
    cmd_set_discard_rectangle_ext: PFN_vkCmdSetDiscardRectangleEXT,
    cmd_set_discard_rectangle_enable_ext: PFN_vkCmdSetDiscardRectangleEnableEXT,
    cmd_set_discard_rectangle_mode_ext: PFN_vkCmdSetDiscardRectangleModeEXT,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_discard_rectangle_ext: transmute(
                    load(c"vkCmdSetDiscardRectangleEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_discard_rectangle_enable_ext: transmute(
                    load(c"vkCmdSetDiscardRectangleEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_discard_rectangle_mode_ext: transmute(
                    load(c"vkCmdSetDiscardRectangleModeEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDiscardRectangleEXT.html>
    #[inline]
    pub unsafe fn cmd_set_discard_rectangle_ext(
        &self,
        command_buffer: CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangles: &[Rect2D],
    ) {
        unsafe {
            (self.cmd_set_discard_rectangle_ext)(
                command_buffer,
                first_discard_rectangle,
                discard_rectangles.len().try_into().unwrap(),
                discard_rectangles.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDiscardRectangleEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_discard_rectangle_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        discard_rectangle_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_discard_rectangle_enable_ext)(
                command_buffer,
                discard_rectangle_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDiscardRectangleModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_discard_rectangle_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        discard_rectangle_mode: DiscardRectangleModeEXT,
    ) {
        unsafe { (self.cmd_set_discard_rectangle_mode_ext)(command_buffer, discard_rectangle_mode) }
    }
}
