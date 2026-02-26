#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR<'a> =
    PhysicalDeviceDynamicRenderingLocalReadFeatures<'a>;
pub type RenderingAttachmentLocationInfoKHR<'a> = RenderingAttachmentLocationInfo<'a>;
pub type RenderingInputAttachmentIndexInfoKHR<'a> = RenderingInputAttachmentIndexInfo<'a>;
pub type PFN_vkCmdSetRenderingAttachmentLocationsKHR = PFN_vkCmdSetRenderingAttachmentLocations;
pub type PFN_vkCmdSetRenderingInputAttachmentIndicesKHR =
    PFN_vkCmdSetRenderingInputAttachmentIndices;
