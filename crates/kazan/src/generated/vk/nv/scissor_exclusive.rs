//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_scissor_exclusive.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_scissor_exclusive";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExclusiveScissorFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExclusiveScissorFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub exclusive_scissor: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExclusiveScissorFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExclusiveScissorFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("exclusive_scissor", &self.exclusive_scissor)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExclusiveScissorFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceExclusiveScissorFeaturesNV<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceExclusiveScissorFeaturesNV<'_> {}

    impl Default for PhysicalDeviceExclusiveScissorFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                exclusive_scissor: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExclusiveScissorFeaturesNV<'a> {
        #[inline]
        pub fn exclusive_scissor(mut self, exclusive_scissor: bool) -> Self {
            self.exclusive_scissor = exclusive_scissor.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineViewportExclusiveScissorStateCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub exclusive_scissor_count: u32,
        pub p_exclusive_scissors: *const Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineViewportExclusiveScissorStateCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineViewportExclusiveScissorStateCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("exclusive_scissor_count", &self.exclusive_scissor_count)
                .field("p_exclusive_scissors", &self.p_exclusive_scissors)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineViewportExclusiveScissorStateCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV;
    }

    unsafe impl Extends<PipelineViewportStateCreateInfo<'_>>
        for PipelineViewportExclusiveScissorStateCreateInfoNV<'_>
    {
    }

    impl Default for PipelineViewportExclusiveScissorStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                exclusive_scissor_count: Default::default(),
                p_exclusive_scissors: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineViewportExclusiveScissorStateCreateInfoNV<'a> {
        #[inline]
        pub fn exclusive_scissors(mut self, exclusive_scissors: &'a [Rect2D]) -> Self {
            self.exclusive_scissor_count = exclusive_scissors.len().try_into().unwrap();
            self.p_exclusive_scissors = exclusive_scissors.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetExclusiveScissorNV.html>
    pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissors: *const Rect2D,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetExclusiveScissorEnableNV.html>
    pub type PFN_vkCmdSetExclusiveScissorEnableNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissor_enables: *const Bool32,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceExclusiveScissorFeaturesNV =
        PhysicalDeviceExclusiveScissorFeaturesNV<'static>;
    pub type VkPipelineViewportExclusiveScissorStateCreateInfoNV =
        PipelineViewportExclusiveScissorStateCreateInfoNV<'static>;
    impl PhysicalDeviceExclusiveScissorFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceExclusiveScissorFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineViewportExclusiveScissorStateCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPipelineViewportExclusiveScissorStateCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_set_exclusive_scissor_enable_nv: PFN_vkCmdSetExclusiveScissorEnableNV,
    cmd_set_exclusive_scissor_nv: PFN_vkCmdSetExclusiveScissorNV,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_exclusive_scissor_enable_nv: transmute(
                    load(c"vkCmdSetExclusiveScissorEnableNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_exclusive_scissor_nv: transmute(
                    load(c"vkCmdSetExclusiveScissorNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetExclusiveScissorEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_exclusive_scissor_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_enables: &[Bool32],
    ) {
        unsafe {
            (self.cmd_set_exclusive_scissor_enable_nv)(
                command_buffer,
                first_exclusive_scissor,
                exclusive_scissor_enables.len().try_into().unwrap(),
                exclusive_scissor_enables.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetExclusiveScissorNV.html>
    #[inline]
    pub unsafe fn cmd_set_exclusive_scissor_nv(
        &self,
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissors: &[Rect2D],
    ) {
        unsafe {
            (self.cmd_set_exclusive_scissor_nv)(
                command_buffer,
                first_exclusive_scissor,
                exclusive_scissors.len().try_into().unwrap(),
                exclusive_scissors.as_ptr() as _,
            )
        }
    }
}
