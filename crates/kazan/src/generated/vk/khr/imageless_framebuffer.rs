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
    pub type PhysicalDeviceImagelessFramebufferFeaturesKHR<'a> =
        PhysicalDeviceImagelessFramebufferFeatures<'a>;
    pub type FramebufferAttachmentsCreateInfoKHR<'a> = FramebufferAttachmentsCreateInfo<'a>;
    pub type FramebufferAttachmentImageInfoKHR<'a> = FramebufferAttachmentImageInfo<'a>;
    pub type RenderPassAttachmentBeginInfoKHR<'a> = RenderPassAttachmentBeginInfo<'a>;
}
