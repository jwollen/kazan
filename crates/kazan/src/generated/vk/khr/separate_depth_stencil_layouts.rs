//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_separate_depth_stencil_layouts.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_separate_depth_stencil_layouts";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR.html>
    pub type PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR<'a> =
        PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAttachmentReferenceStencilLayoutKHR.html>
    pub type AttachmentReferenceStencilLayoutKHR<'a> = AttachmentReferenceStencilLayout<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAttachmentDescriptionStencilLayoutKHR.html>
    pub type AttachmentDescriptionStencilLayoutKHR<'a> = AttachmentDescriptionStencilLayout<'a>;
}
