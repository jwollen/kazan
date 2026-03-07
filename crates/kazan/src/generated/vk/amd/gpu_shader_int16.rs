//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_AMD_gpu_shader_int16.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_gpu_shader_int16";

pub(super) mod defs {}
