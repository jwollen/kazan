//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_index_type_uint8.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_index_type_uint8";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesKHR.html>
    pub type PhysicalDeviceIndexTypeUint8FeaturesKHR<'a> = PhysicalDeviceIndexTypeUint8Features<'a>;
}
