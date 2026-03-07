#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_external_fence";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportFenceCreateInfoKHR.html>
    pub type ExportFenceCreateInfoKHR<'a> = ExportFenceCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFenceImportFlagsKHR.html>
    pub type FenceImportFlagsKHR = FenceImportFlags;
}
