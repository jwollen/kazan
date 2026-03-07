//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_depth_range_unrestricted.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_depth_range_unrestricted";

pub(super) mod defs {}
