//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_image_format_list.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_image_format_list";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageFormatListCreateInfoKHR.html>
    pub type ImageFormatListCreateInfoKHR<'a> = ImageFormatListCreateInfo<'a>;
}
