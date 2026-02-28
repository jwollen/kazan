#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub type PointClippingBehaviorKHR = PointClippingBehavior;
    pub type TessellationDomainOriginKHR = TessellationDomainOrigin;
    pub type InputAttachmentAspectReferenceKHR = InputAttachmentAspectReference;
    pub type RenderPassInputAttachmentAspectCreateInfoKHR<'a> =
        RenderPassInputAttachmentAspectCreateInfo<'a>;
    pub type PhysicalDevicePointClippingPropertiesKHR<'a> =
        PhysicalDevicePointClippingProperties<'a>;
    pub type ImageViewUsageCreateInfoKHR<'a> = ImageViewUsageCreateInfo<'a>;
    pub type PipelineTessellationDomainOriginStateCreateInfoKHR<'a> =
        PipelineTessellationDomainOriginStateCreateInfo<'a>;
}
