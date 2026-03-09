//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_host_query_reset.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_host_query_reset";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceHostQueryResetFeaturesEXT.html>
    pub type PhysicalDeviceHostQueryResetFeaturesEXT<'a> = PhysicalDeviceHostQueryResetFeatures<'a>;
    pub type PFN_vkResetQueryPoolEXT = PFN_vkResetQueryPool;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceHostQueryResetFeaturesEXT =
        PhysicalDeviceHostQueryResetFeaturesEXT<'static>;
}

pub struct DeviceFn {
    reset_query_pool_ext: PFN_vkResetQueryPool,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                reset_query_pool_ext: transmute(
                    load(c"vkResetQueryPoolEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkResetQueryPoolEXT.html>
    #[inline]
    pub unsafe fn reset_query_pool_ext(
        &self,
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        unsafe { (self.reset_query_pool_ext)(device, query_pool, first_query, query_count) }
    }
}
