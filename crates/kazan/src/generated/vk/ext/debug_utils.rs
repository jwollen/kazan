#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    create_debug_utils_messenger_ext: PFN_vkCreateDebugUtilsMessengerEXT,
    destroy_debug_utils_messenger_ext: PFN_vkDestroyDebugUtilsMessengerEXT,
    submit_debug_utils_message_ext: PFN_vkSubmitDebugUtilsMessageEXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_debug_utils_messenger_ext: transmute(
                    load(c"vkCreateDebugUtilsMessengerEXT").ok_or(LoadingError)?,
                ),
                destroy_debug_utils_messenger_ext: transmute(
                    load(c"vkDestroyDebugUtilsMessengerEXT").ok_or(LoadingError)?,
                ),
                submit_debug_utils_message_ext: transmute(
                    load(c"vkSubmitDebugUtilsMessageEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        instance: Instance,
        create_info: &DebugUtilsMessengerCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<DebugUtilsMessengerEXT> {
        unsafe {
            let mut messenger = core::mem::MaybeUninit::uninit();
            let result = (self.create_debug_utils_messenger_ext)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                messenger.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(messenger.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        instance: Instance,
        messenger: DebugUtilsMessengerEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_debug_utils_messenger_ext)(instance, messenger, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn submit_debug_utils_message_ext(
        &self,
        instance: Instance,
        message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: &DebugUtilsMessengerCallbackDataEXT,
    ) {
        unsafe {
            (self.submit_debug_utils_message_ext)(
                instance,
                message_severity,
                message_types,
                callback_data,
            )
        }
    }
}
pub struct DeviceFn {
    set_debug_utils_object_name_ext: PFN_vkSetDebugUtilsObjectNameEXT,
    set_debug_utils_object_tag_ext: PFN_vkSetDebugUtilsObjectTagEXT,
    queue_begin_debug_utils_label_ext: PFN_vkQueueBeginDebugUtilsLabelEXT,
    queue_end_debug_utils_label_ext: PFN_vkQueueEndDebugUtilsLabelEXT,
    queue_insert_debug_utils_label_ext: PFN_vkQueueInsertDebugUtilsLabelEXT,
    cmd_begin_debug_utils_label_ext: PFN_vkCmdBeginDebugUtilsLabelEXT,
    cmd_end_debug_utils_label_ext: PFN_vkCmdEndDebugUtilsLabelEXT,
    cmd_insert_debug_utils_label_ext: PFN_vkCmdInsertDebugUtilsLabelEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                set_debug_utils_object_name_ext: transmute(
                    load(c"vkSetDebugUtilsObjectNameEXT").ok_or(LoadingError)?,
                ),
                set_debug_utils_object_tag_ext: transmute(
                    load(c"vkSetDebugUtilsObjectTagEXT").ok_or(LoadingError)?,
                ),
                queue_begin_debug_utils_label_ext: transmute(
                    load(c"vkQueueBeginDebugUtilsLabelEXT").ok_or(LoadingError)?,
                ),
                queue_end_debug_utils_label_ext: transmute(
                    load(c"vkQueueEndDebugUtilsLabelEXT").ok_or(LoadingError)?,
                ),
                queue_insert_debug_utils_label_ext: transmute(
                    load(c"vkQueueInsertDebugUtilsLabelEXT").ok_or(LoadingError)?,
                ),
                cmd_begin_debug_utils_label_ext: transmute(
                    load(c"vkCmdBeginDebugUtilsLabelEXT").ok_or(LoadingError)?,
                ),
                cmd_end_debug_utils_label_ext: transmute(
                    load(c"vkCmdEndDebugUtilsLabelEXT").ok_or(LoadingError)?,
                ),
                cmd_insert_debug_utils_label_ext: transmute(
                    load(c"vkCmdInsertDebugUtilsLabelEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn set_debug_utils_object_name_ext(
        &self,
        device: Device,
        name_info: &DebugUtilsObjectNameInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.set_debug_utils_object_name_ext)(device, name_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn set_debug_utils_object_tag_ext(
        &self,
        device: Device,
        tag_info: &DebugUtilsObjectTagInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.set_debug_utils_object_tag_ext)(device, tag_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn queue_begin_debug_utils_label_ext(
        &self,
        queue: Queue,
        label_info: &DebugUtilsLabelEXT,
    ) {
        unsafe { (self.queue_begin_debug_utils_label_ext)(queue, label_info) }
    }
    pub unsafe fn queue_end_debug_utils_label_ext(&self, queue: Queue) {
        unsafe { (self.queue_end_debug_utils_label_ext)(queue) }
    }
    pub unsafe fn queue_insert_debug_utils_label_ext(
        &self,
        queue: Queue,
        label_info: &DebugUtilsLabelEXT,
    ) {
        unsafe { (self.queue_insert_debug_utils_label_ext)(queue, label_info) }
    }
    pub unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: CommandBuffer,
        label_info: &DebugUtilsLabelEXT,
    ) {
        unsafe { (self.cmd_begin_debug_utils_label_ext)(command_buffer, label_info) }
    }
    pub unsafe fn cmd_end_debug_utils_label_ext(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_debug_utils_label_ext)(command_buffer) }
    }
    pub unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: CommandBuffer,
        label_info: &DebugUtilsLabelEXT,
    ) {
        unsafe { (self.cmd_insert_debug_utils_label_ext)(command_buffer, label_info) }
    }
}
