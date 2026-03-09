//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_dedicated_allocation.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_dedicated_allocation";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryDedicatedRequirementsKHR.html>
    pub type MemoryDedicatedRequirementsKHR<'a> = MemoryDedicatedRequirements<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryDedicatedAllocateInfoKHR.html>
    pub type MemoryDedicatedAllocateInfoKHR<'a> = MemoryDedicatedAllocateInfo<'a>;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkMemoryDedicatedRequirementsKHR = MemoryDedicatedRequirementsKHR<'static>;
    pub type VkMemoryDedicatedAllocateInfoKHR = MemoryDedicatedAllocateInfoKHR<'static>;
}
