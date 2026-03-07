//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_VALVE_mutable_descriptor_type.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_VALVE_mutable_descriptor_type";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE.html>
    pub type PhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'a> =
        PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMutableDescriptorTypeListVALVE.html>
    pub type MutableDescriptorTypeListVALVE<'a> = MutableDescriptorTypeListEXT<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMutableDescriptorTypeCreateInfoVALVE.html>
    pub type MutableDescriptorTypeCreateInfoVALVE<'a> = MutableDescriptorTypeCreateInfoEXT<'a>;
}
