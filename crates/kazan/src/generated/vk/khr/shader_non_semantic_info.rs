//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_shader_non_semantic_info.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_shader_non_semantic_info";

pub(super) mod defs {}
