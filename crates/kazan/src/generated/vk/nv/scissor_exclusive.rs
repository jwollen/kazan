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
    pub struct PhysicalDeviceExclusiveScissorFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub exclusive_scissor: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExclusiveScissorFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceExclusiveScissorFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceExclusiveScissorFeaturesNV<'a> {}
    impl Default for PhysicalDeviceExclusiveScissorFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                exclusive_scissor: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceExclusiveScissorFeaturesNV<'a> {
        pub fn exclusive_scissor(mut self, exclusive_scissor: Bool32) -> Self {
            self.exclusive_scissor = exclusive_scissor;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineViewportExclusiveScissorStateCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub exclusive_scissor_count: u32,
        pub p_exclusive_scissors: *const Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineViewportExclusiveScissorStateCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV;
    }
    unsafe impl<'a> Extends<PipelineViewportStateCreateInfo<'a>>
        for PipelineViewportExclusiveScissorStateCreateInfoNV<'a>
    {
    }
    impl Default for PipelineViewportExclusiveScissorStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                exclusive_scissor_count: Default::default(),
                p_exclusive_scissors: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineViewportExclusiveScissorStateCreateInfoNV<'a> {
        pub fn exclusive_scissors(mut self, exclusive_scissors: &'a [Rect2D]) -> Self {
            self.exclusive_scissor_count = exclusive_scissors.len().try_into().unwrap();
            self.p_exclusive_scissors = exclusive_scissors.as_ptr();
            self
        }
    }
    pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissors: *const Rect2D,
    );
    pub type PFN_vkCmdSetExclusiveScissorEnableNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissor_enables: *const Bool32,
    );
}
pub struct DeviceFn {
    cmd_set_exclusive_scissor_enable_nv: PFN_vkCmdSetExclusiveScissorEnableNV,
    cmd_set_exclusive_scissor_nv: PFN_vkCmdSetExclusiveScissorNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_exclusive_scissor_enable_nv: transmute(
                    load(c"vkCmdSetExclusiveScissorEnableNV").ok_or(LoadingError)?,
                ),
                cmd_set_exclusive_scissor_nv: transmute(
                    load(c"vkCmdSetExclusiveScissorNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
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
