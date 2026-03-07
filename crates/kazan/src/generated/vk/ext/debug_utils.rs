#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_debug_utils";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    handle_nondispatchable!(
        DebugUtilsMessengerEXT,
        DEBUG_UTILS_MESSENGER_EXT,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugUtilsMessengerEXT.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugUtilsObjectNameInfoEXT.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for DebugUtilsObjectNameInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DebugUtilsObjectNameInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("object_type", &self.object_type)
                .field("object_handle", &self.object_handle)
                .field("p_object_name", &unsafe { as_c_str(self.p_object_name) })
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DebugUtilsObjectNameInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT;
    }

    unsafe impl<'a> Extends<PipelineShaderStageCreateInfo<'a>> for DebugUtilsObjectNameInfoEXT<'a> {}
    unsafe impl<'a> Extends<ResourceDescriptorInfoEXT<'a>> for DebugUtilsObjectNameInfoEXT<'a> {}
    unsafe impl<'a> Extends<SamplerCreateInfo<'a>> for DebugUtilsObjectNameInfoEXT<'a> {}

    impl Default for DebugUtilsObjectNameInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

        pub fn object_name(mut self, object_name: &'a CStr) -> Self {
            self.p_object_name = object_name.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugUtilsObjectTagInfoEXT.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for DebugUtilsObjectTagInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DebugUtilsObjectTagInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("object_type", &self.object_type)
                .field("object_handle", &self.object_handle)
                .field("tag_name", &self.tag_name)
                .field("tag_size", &self.tag_size)
                .field("p_tag", &self.p_tag)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DebugUtilsObjectTagInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEBUG_UTILS_OBJECT_TAG_INFO_EXT;
    }

    impl Default for DebugUtilsObjectTagInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugUtilsLabelEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DebugUtilsLabelEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_label_name: *const c_char,
        pub color: [f32; 4],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DebugUtilsLabelEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DebugUtilsLabelEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_label_name", &unsafe { as_c_str(self.p_label_name) })
                .field("color", &self.color)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DebugUtilsLabelEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEBUG_UTILS_LABEL_EXT;
    }

    impl Default for DebugUtilsLabelEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_label_name: core::ptr::null(),
                color: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DebugUtilsLabelEXT<'a> {
        pub fn label_name(mut self, label_name: &'a CStr) -> Self {
            self.p_label_name = label_name.as_ptr();
            self
        }

        pub fn color(mut self, color: [f32; 4]) -> Self {
            self.color = color;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugUtilsMessengerCreateInfoEXT.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for DebugUtilsMessengerCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DebugUtilsMessengerCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("message_severity", &self.message_severity)
                .field("message_type", &self.message_type)
                .field(
                    "pfn_user_callback",
                    &self.pfn_user_callback.map(|f| f as *const ()),
                )
                .field("p_user_data", &self.p_user_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DebugUtilsMessengerCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<InstanceCreateInfo<'a>> for DebugUtilsMessengerCreateInfoEXT<'a> {}

    impl Default for DebugUtilsMessengerCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

        pub fn message_severity(
            mut self,
            message_severity: DebugUtilsMessageSeverityFlagsEXT,
        ) -> Self {
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

        pub fn user_data(mut self, user_data: *mut c_void) -> Self {
            self.p_user_data = user_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugUtilsMessengerCallbackDataEXT.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for DebugUtilsMessengerCallbackDataEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DebugUtilsMessengerCallbackDataEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("p_message_id_name", &unsafe {
                    as_c_str(self.p_message_id_name)
                })
                .field("message_id_number", &self.message_id_number)
                .field("p_message", &unsafe { as_c_str(self.p_message) })
                .field("queue_label_count", &self.queue_label_count)
                .field("p_queue_labels", &self.p_queue_labels)
                .field("cmd_buf_label_count", &self.cmd_buf_label_count)
                .field("p_cmd_buf_labels", &self.p_cmd_buf_labels)
                .field("object_count", &self.object_count)
                .field("p_objects", &self.p_objects)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DebugUtilsMessengerCallbackDataEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT;
    }

    impl Default for DebugUtilsMessengerCallbackDataEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

        pub fn message_id_name(mut self, message_id_name: &'a CStr) -> Self {
            self.p_message_id_name = message_id_name.as_ptr();
            self
        }

        pub fn message_id_number(mut self, message_id_number: i32) -> Self {
            self.message_id_number = message_id_number;
            self
        }

        pub fn message(mut self, message: &'a CStr) -> Self {
            self.p_message = message.as_ptr();
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugUtilsMessageSeverityFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DebugUtilsMessageSeverityFlagsEXT(Flags);
    vk_bitflags_wrapped!(DebugUtilsMessageSeverityFlagsEXT, Flags);

    impl DebugUtilsMessageSeverityFlagsEXT {
        pub const VERBOSE_EXT: Self = Self(DebugUtilsMessageSeverityFlagBitsEXT::VERBOSE_EXT.0);
        pub const INFO_EXT: Self = Self(DebugUtilsMessageSeverityFlagBitsEXT::INFO_EXT.0);
        pub const WARNING_EXT: Self = Self(DebugUtilsMessageSeverityFlagBitsEXT::WARNING_EXT.0);
        pub const ERROR_EXT: Self = Self(DebugUtilsMessageSeverityFlagBitsEXT::ERROR_EXT.0);
    }

    impl fmt::Debug for DebugUtilsMessageSeverityFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    DebugUtilsMessageSeverityFlagsEXT::VERBOSE_EXT.0,
                    "VERBOSE_EXT",
                ),
                (DebugUtilsMessageSeverityFlagsEXT::INFO_EXT.0, "INFO_EXT"),
                (
                    DebugUtilsMessageSeverityFlagsEXT::WARNING_EXT.0,
                    "WARNING_EXT",
                ),
                (DebugUtilsMessageSeverityFlagsEXT::ERROR_EXT.0, "ERROR_EXT"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DebugUtilsMessageSeverityFlagBitsEXT(u32);

    impl DebugUtilsMessageSeverityFlagBitsEXT {
        pub const VERBOSE_EXT: Self = Self(1 << 0);
        pub const INFO_EXT: Self = Self(1 << 4);
        pub const WARNING_EXT: Self = Self(1 << 8);
        pub const ERROR_EXT: Self = Self(1 << 12);
    }

    impl fmt::Debug for DebugUtilsMessageSeverityFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::VERBOSE_EXT => Some("VERBOSE_EXT"),
                Self::INFO_EXT => Some("INFO_EXT"),
                Self::WARNING_EXT => Some("WARNING_EXT"),
                Self::ERROR_EXT => Some("ERROR_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugUtilsMessageTypeFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DebugUtilsMessageTypeFlagsEXT(Flags);
    vk_bitflags_wrapped!(DebugUtilsMessageTypeFlagsEXT, Flags);

    impl DebugUtilsMessageTypeFlagsEXT {
        pub const GENERAL_EXT: Self = Self(DebugUtilsMessageTypeFlagBitsEXT::GENERAL_EXT.0);
        pub const VALIDATION_EXT: Self = Self(DebugUtilsMessageTypeFlagBitsEXT::VALIDATION_EXT.0);
        pub const PERFORMANCE_EXT: Self = Self(DebugUtilsMessageTypeFlagBitsEXT::PERFORMANCE_EXT.0);
        // VK_EXT_device_address_binding_report
        pub const DEVICE_ADDRESS_BINDING_EXT: Self =
            Self(DebugUtilsMessageTypeFlagBitsEXT::DEVICE_ADDRESS_BINDING_EXT.0);
    }

    impl fmt::Debug for DebugUtilsMessageTypeFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (DebugUtilsMessageTypeFlagsEXT::GENERAL_EXT.0, "GENERAL_EXT"),
                (
                    DebugUtilsMessageTypeFlagsEXT::VALIDATION_EXT.0,
                    "VALIDATION_EXT",
                ),
                (
                    DebugUtilsMessageTypeFlagsEXT::PERFORMANCE_EXT.0,
                    "PERFORMANCE_EXT",
                ),
                (
                    DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING_EXT.0,
                    "DEVICE_ADDRESS_BINDING_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DebugUtilsMessageTypeFlagBitsEXT(u32);

    impl DebugUtilsMessageTypeFlagBitsEXT {
        pub const GENERAL_EXT: Self = Self(1 << 0);
        pub const VALIDATION_EXT: Self = Self(1 << 1);
        pub const PERFORMANCE_EXT: Self = Self(1 << 2);
        // VK_EXT_device_address_binding_report
        pub const DEVICE_ADDRESS_BINDING_EXT: Self = Self(1 << 3);
    }

    impl fmt::Debug for DebugUtilsMessageTypeFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::GENERAL_EXT => Some("GENERAL_EXT"),
                Self::VALIDATION_EXT => Some("VALIDATION_EXT"),
                Self::PERFORMANCE_EXT => Some("PERFORMANCE_EXT"),
                Self::DEVICE_ADDRESS_BINDING_EXT => Some("DEVICE_ADDRESS_BINDING_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugUtilsMessengerCreateFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DebugUtilsMessengerCreateFlagsEXT(Flags);
    vk_bitflags_wrapped!(DebugUtilsMessengerCreateFlagsEXT, Flags);

    impl fmt::Debug for DebugUtilsMessengerCreateFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugUtilsMessengerCallbackDataFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DebugUtilsMessengerCallbackDataFlagsEXT(Flags);
    vk_bitflags_wrapped!(DebugUtilsMessengerCallbackDataFlagsEXT, Flags);

    impl fmt::Debug for DebugUtilsMessengerCallbackDataFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html>
    pub type PFN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
        message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
        p_user_data: *mut c_void,
    ) -> Bool32;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetDebugUtilsObjectNameEXT.html>
    pub type PFN_vkSetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
        device: Device,
        p_name_info: *const DebugUtilsObjectNameInfoEXT<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetDebugUtilsObjectTagEXT.html>
    pub type PFN_vkSetDebugUtilsObjectTagEXT = unsafe extern "system" fn(
        device: Device,
        p_tag_info: *const DebugUtilsObjectTagInfoEXT<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueBeginDebugUtilsLabelEXT.html>
    pub type PFN_vkQueueBeginDebugUtilsLabelEXT =
        unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT<'_>);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueEndDebugUtilsLabelEXT.html>
    pub type PFN_vkQueueEndDebugUtilsLabelEXT = unsafe extern "system" fn(queue: Queue);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueInsertDebugUtilsLabelEXT.html>
    pub type PFN_vkQueueInsertDebugUtilsLabelEXT =
        unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT<'_>);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginDebugUtilsLabelEXT.html>
    pub type PFN_vkCmdBeginDebugUtilsLabelEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_label_info: *const DebugUtilsLabelEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndDebugUtilsLabelEXT.html>
    pub type PFN_vkCmdEndDebugUtilsLabelEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdInsertDebugUtilsLabelEXT.html>
    pub type PFN_vkCmdInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_label_info: *const DebugUtilsLabelEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDebugUtilsMessengerEXT.html>
    pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const DebugUtilsMessengerCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_messenger: *mut DebugUtilsMessengerEXT,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyDebugUtilsMessengerEXT.html>
    pub type PFN_vkDestroyDebugUtilsMessengerEXT = unsafe extern "system" fn(
        instance: Instance,
        messenger: DebugUtilsMessengerEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSubmitDebugUtilsMessageEXT.html>
    pub type PFN_vkSubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
        instance: Instance,
        message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
    );
}

pub struct InstanceFn {
    create_debug_utils_messenger_ext: PFN_vkCreateDebugUtilsMessengerEXT,
    destroy_debug_utils_messenger_ext: PFN_vkDestroyDebugUtilsMessengerEXT,
    submit_debug_utils_message_ext: PFN_vkSubmitDebugUtilsMessageEXT,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_debug_utils_messenger_ext: transmute(
                    load(c"vkCreateDebugUtilsMessengerEXT").ok_or(MissingEntryPointError)?,
                ),
                destroy_debug_utils_messenger_ext: transmute(
                    load(c"vkDestroyDebugUtilsMessengerEXT").ok_or(MissingEntryPointError)?,
                ),
                submit_debug_utils_message_ext: transmute(
                    load(c"vkSubmitDebugUtilsMessageEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDebugUtilsMessengerEXT.html>
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        instance: Instance,
        create_info: &DebugUtilsMessengerCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyDebugUtilsMessengerEXT.html>
    pub unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        instance: Instance,
        messenger: DebugUtilsMessengerEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_debug_utils_messenger_ext)(instance, messenger, allocator.to_raw_ptr())
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSubmitDebugUtilsMessageEXT.html>
    pub unsafe fn submit_debug_utils_message_ext(
        &self,
        instance: Instance,
        message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: &DebugUtilsMessengerCallbackDataEXT<'_>,
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
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                set_debug_utils_object_name_ext: transmute(
                    load(c"vkSetDebugUtilsObjectNameEXT").ok_or(MissingEntryPointError)?,
                ),
                set_debug_utils_object_tag_ext: transmute(
                    load(c"vkSetDebugUtilsObjectTagEXT").ok_or(MissingEntryPointError)?,
                ),
                queue_begin_debug_utils_label_ext: transmute(
                    load(c"vkQueueBeginDebugUtilsLabelEXT").ok_or(MissingEntryPointError)?,
                ),
                queue_end_debug_utils_label_ext: transmute(
                    load(c"vkQueueEndDebugUtilsLabelEXT").ok_or(MissingEntryPointError)?,
                ),
                queue_insert_debug_utils_label_ext: transmute(
                    load(c"vkQueueInsertDebugUtilsLabelEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_begin_debug_utils_label_ext: transmute(
                    load(c"vkCmdBeginDebugUtilsLabelEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_debug_utils_label_ext: transmute(
                    load(c"vkCmdEndDebugUtilsLabelEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_insert_debug_utils_label_ext: transmute(
                    load(c"vkCmdInsertDebugUtilsLabelEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetDebugUtilsObjectNameEXT.html>
    pub unsafe fn set_debug_utils_object_name_ext(
        &self,
        device: Device,
        name_info: &DebugUtilsObjectNameInfoEXT<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.set_debug_utils_object_name_ext)(device, name_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetDebugUtilsObjectTagEXT.html>
    pub unsafe fn set_debug_utils_object_tag_ext(
        &self,
        device: Device,
        tag_info: &DebugUtilsObjectTagInfoEXT<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.set_debug_utils_object_tag_ext)(device, tag_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueBeginDebugUtilsLabelEXT.html>
    pub unsafe fn queue_begin_debug_utils_label_ext(
        &self,
        queue: Queue,
        label_info: &DebugUtilsLabelEXT<'_>,
    ) {
        unsafe { (self.queue_begin_debug_utils_label_ext)(queue, label_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueEndDebugUtilsLabelEXT.html>
    pub unsafe fn queue_end_debug_utils_label_ext(&self, queue: Queue) {
        unsafe { (self.queue_end_debug_utils_label_ext)(queue) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueInsertDebugUtilsLabelEXT.html>
    pub unsafe fn queue_insert_debug_utils_label_ext(
        &self,
        queue: Queue,
        label_info: &DebugUtilsLabelEXT<'_>,
    ) {
        unsafe { (self.queue_insert_debug_utils_label_ext)(queue, label_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginDebugUtilsLabelEXT.html>
    pub unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: CommandBuffer,
        label_info: &DebugUtilsLabelEXT<'_>,
    ) {
        unsafe { (self.cmd_begin_debug_utils_label_ext)(command_buffer, label_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndDebugUtilsLabelEXT.html>
    pub unsafe fn cmd_end_debug_utils_label_ext(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_debug_utils_label_ext)(command_buffer) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdInsertDebugUtilsLabelEXT.html>
    pub unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: CommandBuffer,
        label_info: &DebugUtilsLabelEXT<'_>,
    ) {
        unsafe { (self.cmd_insert_debug_utils_label_ext)(command_buffer, label_info) }
    }
}
