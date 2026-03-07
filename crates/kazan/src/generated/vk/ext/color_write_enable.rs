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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceColorWriteEnableFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceColorWriteEnableFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub color_write_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceColorWriteEnableFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceColorWriteEnableFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceColorWriteEnableFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceColorWriteEnableFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                color_write_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceColorWriteEnableFeaturesEXT<'a> {
        pub fn color_write_enable(mut self, color_write_enable: bool) -> Self {
            self.color_write_enable = color_write_enable.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineColorWriteCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineColorWriteCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub attachment_count: u32,
        pub p_color_write_enables: *const Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineColorWriteCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_COLOR_WRITE_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<PipelineColorBlendStateCreateInfo<'a>>
        for PipelineColorWriteCreateInfoEXT<'a>
    {
    }

    impl Default for PipelineColorWriteCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                attachment_count: Default::default(),
                p_color_write_enables: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineColorWriteCreateInfoEXT<'a> {
        pub fn color_write_enables(mut self, color_write_enables: &'a [Bool32]) -> Self {
            self.attachment_count = color_write_enables.len().try_into().unwrap();
            self.p_color_write_enables = color_write_enables.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorWriteEnableEXT.html>
    pub type PFN_vkCmdSetColorWriteEnableEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        attachment_count: u32,
        p_color_write_enables: *const Bool32,
    );
}

pub struct DeviceFn {
    cmd_set_color_write_enable_ext: PFN_vkCmdSetColorWriteEnableEXT,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_color_write_enable_ext: transmute(
                    load(c"vkCmdSetColorWriteEnableEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorWriteEnableEXT.html>
    pub unsafe fn cmd_set_color_write_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        color_write_enables: &[Bool32],
    ) {
        unsafe {
            (self.cmd_set_color_write_enable_ext)(
                command_buffer,
                color_write_enables.len().try_into().unwrap(),
                color_write_enables.as_ptr() as _,
            )
        }
    }
}
