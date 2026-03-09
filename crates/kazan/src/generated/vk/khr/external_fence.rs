//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_external_fence.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_external_fence";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportFenceCreateInfoKHR.html>
    pub type ExportFenceCreateInfoKHR<'a> = ExportFenceCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFenceImportFlagsKHR.html>
    pub type FenceImportFlagsKHR = FenceImportFlags;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkExportFenceCreateInfoKHR = ExportFenceCreateInfoKHR<'static>;
    pub type VkFenceImportFlagsKHR = FenceImportFlagsKHR;
}
