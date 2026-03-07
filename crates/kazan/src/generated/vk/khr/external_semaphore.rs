//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_external_semaphore.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_external_semaphore";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportSemaphoreCreateInfoKHR.html>
    pub type ExportSemaphoreCreateInfoKHR<'a> = ExportSemaphoreCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSemaphoreImportFlagsKHR.html>
    pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;
}
