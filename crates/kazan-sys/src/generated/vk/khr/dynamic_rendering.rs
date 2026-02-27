#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PipelineRenderingCreateInfoKHR<'a> = PipelineRenderingCreateInfo<'a>;
pub type RenderingInfoKHR<'a> = RenderingInfo<'a>;
pub type RenderingAttachmentInfoKHR<'a> = RenderingAttachmentInfo<'a>;
pub type PhysicalDeviceDynamicRenderingFeaturesKHR<'a> = PhysicalDeviceDynamicRenderingFeatures<'a>;
pub type CommandBufferInheritanceRenderingInfoKHR<'a> = CommandBufferInheritanceRenderingInfo<'a>;
pub type RenderingFlagsKHR = RenderingFlags;
pub type PFN_vkCmdBeginRenderingKHR = PFN_vkCmdBeginRendering;
pub type PFN_vkCmdEndRenderingKHR = PFN_vkCmdEndRendering;
