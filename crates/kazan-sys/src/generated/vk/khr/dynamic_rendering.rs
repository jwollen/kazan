#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub type PipelineRenderingCreateInfoKHR = PipelineRenderingCreateInfo;
pub type RenderingInfoKHR = RenderingInfo;
pub type RenderingAttachmentInfoKHR = RenderingAttachmentInfo;
pub type PhysicalDeviceDynamicRenderingFeaturesKHR = PhysicalDeviceDynamicRenderingFeatures;
pub type CommandBufferInheritanceRenderingInfoKHR = CommandBufferInheritanceRenderingInfo;
pub type RenderingFlagsKHR = RenderingFlags;
pub type PFN_vkCmdBeginRenderingKHR = PFN_vkCmdBeginRendering;
pub type PFN_vkCmdEndRenderingKHR = PFN_vkCmdEndRendering;
