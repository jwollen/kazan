#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryImageCreateInfoKHR.html>
    pub type ExternalMemoryImageCreateInfoKHR<'a> = ExternalMemoryImageCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryBufferCreateInfoKHR.html>
    pub type ExternalMemoryBufferCreateInfoKHR<'a> = ExternalMemoryBufferCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMemoryAllocateInfoKHR.html>
    pub type ExportMemoryAllocateInfoKHR<'a> = ExportMemoryAllocateInfo<'a>;
}
