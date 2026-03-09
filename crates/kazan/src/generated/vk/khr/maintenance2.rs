//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_maintenance2.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_maintenance2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPointClippingBehaviorKHR.html>
    pub type PointClippingBehaviorKHR = PointClippingBehavior;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTessellationDomainOriginKHR.html>
    pub type TessellationDomainOriginKHR = TessellationDomainOrigin;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkInputAttachmentAspectReferenceKHR.html>
    pub type InputAttachmentAspectReferenceKHR = InputAttachmentAspectReference;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassInputAttachmentAspectCreateInfoKHR.html>
    pub type RenderPassInputAttachmentAspectCreateInfoKHR<'a> =
        RenderPassInputAttachmentAspectCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePointClippingPropertiesKHR.html>
    pub type PhysicalDevicePointClippingPropertiesKHR<'a> =
        PhysicalDevicePointClippingProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageViewUsageCreateInfoKHR.html>
    pub type ImageViewUsageCreateInfoKHR<'a> = ImageViewUsageCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineTessellationDomainOriginStateCreateInfoKHR.html>
    pub type PipelineTessellationDomainOriginStateCreateInfoKHR<'a> =
        PipelineTessellationDomainOriginStateCreateInfo<'a>;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPointClippingBehaviorKHR = PointClippingBehaviorKHR;
    pub type VkTessellationDomainOriginKHR = TessellationDomainOriginKHR;
    pub type VkInputAttachmentAspectReferenceKHR = InputAttachmentAspectReferenceKHR;
    pub type VkRenderPassInputAttachmentAspectCreateInfoKHR =
        RenderPassInputAttachmentAspectCreateInfoKHR<'static>;
    pub type VkPhysicalDevicePointClippingPropertiesKHR =
        PhysicalDevicePointClippingPropertiesKHR<'static>;
    pub type VkImageViewUsageCreateInfoKHR = ImageViewUsageCreateInfoKHR<'static>;
    pub type VkPipelineTessellationDomainOriginStateCreateInfoKHR =
        PipelineTessellationDomainOriginStateCreateInfoKHR<'static>;
}
