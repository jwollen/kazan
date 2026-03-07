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
