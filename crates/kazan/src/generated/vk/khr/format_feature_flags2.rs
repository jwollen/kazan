//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_format_feature_flags2.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_format_feature_flags2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFormatProperties3KHR.html>
    pub type FormatProperties3KHR<'a> = FormatProperties3<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFormatFeatureFlags2KHR.html>
    pub type FormatFeatureFlags2KHR = FormatFeatureFlags2;
}
