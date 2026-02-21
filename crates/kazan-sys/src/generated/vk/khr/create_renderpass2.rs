#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type AttachmentDescription2KHR = AttachmentDescription2;
pub type AttachmentReference2KHR = AttachmentReference2;
pub type SubpassDescription2KHR = SubpassDescription2;
pub type SubpassDependency2KHR = SubpassDependency2;
pub type RenderPassCreateInfo2KHR = RenderPassCreateInfo2;
pub type SubpassBeginInfoKHR = SubpassBeginInfo;
pub type SubpassEndInfoKHR = SubpassEndInfo;
pub type PFN_vkCreateRenderPass2KHR = PFN_vkCreateRenderPass2;
pub type PFN_vkCmdBeginRenderPass2KHR = PFN_vkCmdBeginRenderPass2;
pub type PFN_vkCmdNextSubpass2KHR = PFN_vkCmdNextSubpass2;
pub type PFN_vkCmdEndRenderPass2KHR = PFN_vkCmdEndRenderPass2;
