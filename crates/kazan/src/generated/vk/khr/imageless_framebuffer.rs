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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImagelessFramebufferFeaturesKHR.html>
    pub type PhysicalDeviceImagelessFramebufferFeaturesKHR<'a> =
        PhysicalDeviceImagelessFramebufferFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFramebufferAttachmentsCreateInfoKHR.html>
    pub type FramebufferAttachmentsCreateInfoKHR<'a> = FramebufferAttachmentsCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFramebufferAttachmentImageInfoKHR.html>
    pub type FramebufferAttachmentImageInfoKHR<'a> = FramebufferAttachmentImageInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassAttachmentBeginInfoKHR.html>
    pub type RenderPassAttachmentBeginInfoKHR<'a> = RenderPassAttachmentBeginInfo<'a>;
}
