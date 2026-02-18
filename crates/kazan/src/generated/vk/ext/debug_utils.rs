#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    create_debug_utils_messenger_ext: PFN_vkCreateDebugUtilsMessengerEXT,
    destroy_debug_utils_messenger_ext: PFN_vkDestroyDebugUtilsMessengerEXT,
    submit_debug_utils_message_ext: PFN_vkSubmitDebugUtilsMessageEXT,
}
impl InstanceFn {
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        instance: Instance,
        create_info: &DebugUtilsMessengerCreateInfoEXT,
        allocator: &AllocationCallbacks,
        messenger: &mut DebugUtilsMessengerEXT,
    ) -> Result {
        unsafe {
            (self.create_debug_utils_messenger_ext)(instance, create_info, allocator, messenger)
        }
    }
    pub unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        instance: Instance,
        messenger: DebugUtilsMessengerEXT,
        allocator: &AllocationCallbacks,
    ) {
        unsafe { (self.destroy_debug_utils_messenger_ext)(instance, messenger, allocator) }
    }
    pub unsafe fn submit_debug_utils_message_ext(
        &self,
        instance: Instance,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
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
    pub unsafe fn set_debug_utils_object_name_ext(
        &self,
        device: Device,
        name_info: &DebugUtilsObjectNameInfoEXT,
    ) -> Result {
        unsafe { (self.set_debug_utils_object_name_ext)(device, name_info) }
    }
    pub unsafe fn set_debug_utils_object_tag_ext(
        &self,
        device: Device,
        tag_info: &DebugUtilsObjectTagInfoEXT,
    ) -> Result {
        unsafe { (self.set_debug_utils_object_tag_ext)(device, tag_info) }
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
