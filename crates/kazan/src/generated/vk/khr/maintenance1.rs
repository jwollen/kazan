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
    pub type CommandPoolTrimFlagsKHR = CommandPoolTrimFlags;
    pub type PFN_vkTrimCommandPoolKHR = PFN_vkTrimCommandPool;
}
pub struct DeviceFn {
    trim_command_pool_khr: PFN_vkTrimCommandPool,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                trim_command_pool_khr: transmute(
                    load(c"vkTrimCommandPoolKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn trim_command_pool_khr(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        unsafe { (self.trim_command_pool_khr)(device, command_pool, flags) }
    }
}
