#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    create_debug_report_callback_ext: PFN_vkCreateDebugReportCallbackEXT,
    destroy_debug_report_callback_ext: PFN_vkDestroyDebugReportCallbackEXT,
    debug_report_message_ext: PFN_vkDebugReportMessageEXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_debug_report_callback_ext: transmute(
                    load(c"vkCreateDebugReportCallbackEXT").ok_or(LoadingError)?,
                ),
                destroy_debug_report_callback_ext: transmute(
                    load(c"vkDestroyDebugReportCallbackEXT").ok_or(LoadingError)?,
                ),
                debug_report_message_ext: transmute(
                    load(c"vkDebugReportMessageEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_debug_report_callback_ext(
        &self,
        instance: Instance,
        create_info: &DebugReportCallbackCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<DebugReportCallbackEXT> {
        unsafe {
            let mut callback = core::mem::MaybeUninit::uninit();
            let result = (self.create_debug_report_callback_ext)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                callback.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(callback.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_debug_report_callback_ext(
        &self,
        instance: Instance,
        callback: DebugReportCallbackEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_debug_report_callback_ext)(instance, callback, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn debug_report_message_ext(
        &self,
        instance: Instance,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        layer_prefix: &CStr,
        message: &CStr,
    ) {
        unsafe {
            (self.debug_report_message_ext)(
                instance,
                flags,
                object_type,
                object,
                location,
                message_code,
                layer_prefix.as_ptr() as _,
                message.as_ptr() as _,
            )
        }
    }
}
