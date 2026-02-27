#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DebugUtilsMessengerEXT(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsObjectNameInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub p_object_name: *const c_char,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DebugUtilsObjectNameInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
            p_next: core::ptr::null(),
            object_type: Default::default(),
            object_handle: Default::default(),
            p_object_name: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DebugUtilsObjectNameInfoEXT<'a> {
    pub fn object_type(mut self, object_type: ObjectType) -> Self {
        self.object_type = object_type;
        self
    }
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.object_handle = object_handle;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for DebugUtilsObjectTagInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEBUG_UTILS_OBJECT_TAG_INFO_EXT,
            p_next: core::ptr::null(),
            object_type: Default::default(),
            object_handle: Default::default(),
            tag_name: Default::default(),
            tag_size: Default::default(),
            p_tag: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DebugUtilsObjectTagInfoEXT<'a> {
    pub fn object_type(mut self, object_type: ObjectType) -> Self {
        self.object_type = object_type;
        self
    }
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.object_handle = object_handle;
        self
    }
    pub fn tag_name(mut self, tag_name: u64) -> Self {
        self.tag_name = tag_name;
        self
    }
    pub fn tag(mut self, tag: &'a [u8]) -> Self {
        self.tag_size = tag.len().try_into().unwrap();
        self.p_tag = tag.as_ptr() as _;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsLabelEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_label_name: *const c_char,
    pub color: [f32; 4],
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DebugUtilsLabelEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEBUG_UTILS_LABEL_EXT,
            p_next: core::ptr::null(),
            p_label_name: core::ptr::null(),
            color: [Default::default(); _],
            _marker: PhantomData,
        }
    }
}
impl<'a> DebugUtilsLabelEXT<'a> {
    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for DebugUtilsMessengerCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            message_severity: Default::default(),
            message_type: Default::default(),
            pfn_user_callback: Default::default(),
            p_user_data: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DebugUtilsMessengerCreateInfoEXT<'a> {
    pub fn flags(mut self, flags: DebugUtilsMessengerCreateFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
    pub fn message_severity(mut self, message_severity: DebugUtilsMessageSeverityFlagsEXT) -> Self {
        self.message_severity = message_severity;
        self
    }
    pub fn message_type(mut self, message_type: DebugUtilsMessageTypeFlagsEXT) -> Self {
        self.message_type = message_type;
        self
    }
    pub fn pfn_user_callback(
        mut self,
        pfn_user_callback: PFN_vkDebugUtilsMessengerCallbackEXT,
    ) -> Self {
        self.pfn_user_callback = Some(pfn_user_callback);
        self
    }
    pub fn user_data(mut self, user_data: &'a mut c_void) -> Self {
        self.p_user_data = user_data;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for DebugUtilsMessengerCallbackDataEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            p_message_id_name: core::ptr::null(),
            message_id_number: Default::default(),
            p_message: core::ptr::null(),
            queue_label_count: Default::default(),
            p_queue_labels: core::ptr::null(),
            cmd_buf_label_count: Default::default(),
            p_cmd_buf_labels: core::ptr::null(),
            object_count: Default::default(),
            p_objects: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DebugUtilsMessengerCallbackDataEXT<'a> {
    pub fn flags(mut self, flags: DebugUtilsMessengerCallbackDataFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
    pub fn message_id_number(mut self, message_id_number: i32) -> Self {
        self.message_id_number = message_id_number;
        self
    }
    pub fn queue_labels(mut self, queue_labels: &'a [DebugUtilsLabelEXT<'a>]) -> Self {
        self.queue_label_count = queue_labels.len().try_into().unwrap();
        self.p_queue_labels = queue_labels.as_ptr();
        self
    }
    pub fn cmd_buf_labels(mut self, cmd_buf_labels: &'a [DebugUtilsLabelEXT<'a>]) -> Self {
        self.cmd_buf_label_count = cmd_buf_labels.len().try_into().unwrap();
        self.p_cmd_buf_labels = cmd_buf_labels.as_ptr();
        self
    }
    pub fn objects(mut self, objects: &'a [DebugUtilsObjectNameInfoEXT<'a>]) -> Self {
        self.object_count = objects.len().try_into().unwrap();
        self.p_objects = objects.as_ptr();
        self
    }
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
