#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DebugUtilsMessengerEXT(u64);
#[repr(C)]
pub struct DebugUtilsObjectNameInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub p_object_name: *const c_char,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DebugUtilsObjectTagInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DebugUtilsLabelEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_label_name: *const c_char,
    pub color: [f32; 4],
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DebugUtilsMessengerCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugUtilsMessengerCreateFlagsEXT,
    pub message_severity: DebugUtilsMessageSeverityFlagsEXT,
    pub message_type: DebugUtilsMessageTypeFlagsEXT,
    pub pfn_user_callback: Option<PFN_vkDebugUtilsMessengerCallbackEXT>,
    pub p_user_data: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DebugUtilsMessengerCallbackDataEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    pub p_message_id_name: *const c_char,
    pub message_id_number: i32,
    pub p_message: *const c_char,
    pub queue_label_count: u32,
    pub p_queue_labels: *const DebugUtilsLabelEXT<'a>,
    pub cmd_buf_label_count: u32,
    pub p_cmd_buf_labels: *const DebugUtilsLabelEXT<'a>,
    pub object_count: u32,
    pub p_objects: *const DebugUtilsObjectNameInfoEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DebugUtilsMessageSeverityFlagsEXT: Flags {
        const VERBOSE_EXT = DebugUtilsMessageSeverityFlagBitsEXT::VERBOSE_EXT.0;
        const INFO_EXT = DebugUtilsMessageSeverityFlagBitsEXT::INFO_EXT.0;
        const WARNING_EXT = DebugUtilsMessageSeverityFlagBitsEXT::WARNING_EXT.0;
        const ERROR_EXT = DebugUtilsMessageSeverityFlagBitsEXT::ERROR_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessageSeverityFlagBitsEXT(u32);
impl DebugUtilsMessageSeverityFlagBitsEXT {
    pub const VERBOSE_EXT: Self = Self(1 << 0);
    pub const INFO_EXT: Self = Self(1 << 4);
    pub const WARNING_EXT: Self = Self(1 << 8);
    pub const ERROR_EXT: Self = Self(1 << 12);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DebugUtilsMessageTypeFlagsEXT: Flags {
        const GENERAL_EXT = DebugUtilsMessageTypeFlagBitsEXT::GENERAL_EXT.0;
        const VALIDATION_EXT = DebugUtilsMessageTypeFlagBitsEXT::VALIDATION_EXT.0;
        const PERFORMANCE_EXT = DebugUtilsMessageTypeFlagBitsEXT::PERFORMANCE_EXT.0;
        const DEVICE_ADDRESS_BINDING_EXT = DebugUtilsMessageTypeFlagBitsEXT::DEVICE_ADDRESS_BINDING_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessageTypeFlagBitsEXT(u32);
impl DebugUtilsMessageTypeFlagBitsEXT {
    pub const GENERAL_EXT: Self = Self(1 << 0);
    pub const VALIDATION_EXT: Self = Self(1 << 1);
    pub const PERFORMANCE_EXT: Self = Self(1 << 2);
    pub const DEVICE_ADDRESS_BINDING_EXT: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DebugUtilsMessengerCreateFlagsEXT: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DebugUtilsMessengerCallbackDataFlagsEXT: Flags {
    }
}
pub type PFN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
    message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
    p_user_data: *mut c_void,
) -> Bool32;
pub type PFN_vkSetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
    device: Device,
    p_name_info: *const DebugUtilsObjectNameInfoEXT<'_>,
) -> Result;
pub type PFN_vkSetDebugUtilsObjectTagEXT = unsafe extern "system" fn(
    device: Device,
    p_tag_info: *const DebugUtilsObjectTagInfoEXT<'_>,
) -> Result;
pub type PFN_vkQueueBeginDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT<'_>);
pub type PFN_vkQueueEndDebugUtilsLabelEXT = unsafe extern "system" fn(queue: Queue);
pub type PFN_vkQueueInsertDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT<'_>);
pub type PFN_vkCmdBeginDebugUtilsLabelEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_label_info: *const DebugUtilsLabelEXT<'_>,
);
pub type PFN_vkCmdEndDebugUtilsLabelEXT = unsafe extern "system" fn(command_buffer: CommandBuffer);
pub type PFN_vkCmdInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_label_info: *const DebugUtilsLabelEXT<'_>,
);
pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DebugUtilsMessengerCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_messenger: *mut DebugUtilsMessengerEXT,
) -> Result;
pub type PFN_vkDestroyDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: Instance,
    messenger: DebugUtilsMessengerEXT,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkSubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
    instance: Instance,
    message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
);
