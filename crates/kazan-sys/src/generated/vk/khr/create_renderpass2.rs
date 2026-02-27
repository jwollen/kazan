#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type AttachmentDescription2KHR<'a> = AttachmentDescription2<'a>;
pub type AttachmentReference2KHR<'a> = AttachmentReference2<'a>;
pub type SubpassDescription2KHR<'a> = SubpassDescription2<'a>;
pub type SubpassDependency2KHR<'a> = SubpassDependency2<'a>;
pub type RenderPassCreateInfo2KHR<'a> = RenderPassCreateInfo2<'a>;
pub type SubpassBeginInfoKHR<'a> = SubpassBeginInfo<'a>;
pub type SubpassEndInfoKHR<'a> = SubpassEndInfo<'a>;
pub type PFN_vkCreateRenderPass2KHR = PFN_vkCreateRenderPass2;
pub type PFN_vkCmdBeginRenderPass2KHR = PFN_vkCmdBeginRenderPass2;
pub type PFN_vkCmdNextSubpass2KHR = PFN_vkCmdNextSubpass2;
pub type PFN_vkCmdEndRenderPass2KHR = PFN_vkCmdEndRenderPass2;
