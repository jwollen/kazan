//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_external_memory.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_external_memory";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryImageCreateInfoKHR.html>
    pub type ExternalMemoryImageCreateInfoKHR<'a> = ExternalMemoryImageCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryBufferCreateInfoKHR.html>
    pub type ExternalMemoryBufferCreateInfoKHR<'a> = ExternalMemoryBufferCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMemoryAllocateInfoKHR.html>
    pub type ExportMemoryAllocateInfoKHR<'a> = ExportMemoryAllocateInfo<'a>;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkExternalMemoryImageCreateInfoKHR = ExternalMemoryImageCreateInfoKHR<'static>;
    pub type VkExternalMemoryBufferCreateInfoKHR = ExternalMemoryBufferCreateInfoKHR<'static>;
    pub type VkExportMemoryAllocateInfoKHR = ExportMemoryAllocateInfoKHR<'static>;
}
