#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    reset_query_pool_ext: PFN_vkResetQueryPool,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                reset_query_pool_ext: transmute(load(c"vkResetQueryPoolEXT").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
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
