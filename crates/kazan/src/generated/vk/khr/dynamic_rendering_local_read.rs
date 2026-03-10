//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_dynamic_rendering_local_read.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_dynamic_rendering_local_read";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDynamicRenderingLocalReadFeaturesKHR.html>
    pub type PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR<'a> =
        PhysicalDeviceDynamicRenderingLocalReadFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingAttachmentLocationInfoKHR.html>
    pub type RenderingAttachmentLocationInfoKHR<'a> = RenderingAttachmentLocationInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingInputAttachmentIndexInfoKHR.html>
    pub type RenderingInputAttachmentIndexInfoKHR<'a> = RenderingInputAttachmentIndexInfo<'a>;
    pub type PFN_vkCmdSetRenderingAttachmentLocationsKHR = PFN_vkCmdSetRenderingAttachmentLocations;
    pub type PFN_vkCmdSetRenderingInputAttachmentIndicesKHR =
        PFN_vkCmdSetRenderingInputAttachmentIndices;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceDynamicRenderingLocalReadFeaturesKHR =
        PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR<'static>;
    pub type VkRenderingAttachmentLocationInfoKHR = RenderingAttachmentLocationInfoKHR<'static>;
    pub type VkRenderingInputAttachmentIndexInfoKHR = RenderingInputAttachmentIndexInfoKHR<'static>;
}

pub struct DeviceFn {
    cmd_set_rendering_attachment_locations_khr: PFN_vkCmdSetRenderingAttachmentLocations,
    cmd_set_rendering_input_attachment_indices_khr: PFN_vkCmdSetRenderingInputAttachmentIndices,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_rendering_attachment_locations_khr: transmute(
                    load(c"vkCmdSetRenderingAttachmentLocationsKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_set_rendering_input_attachment_indices_khr: transmute(
                    load(c"vkCmdSetRenderingInputAttachmentIndicesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRenderingAttachmentLocationsKHR.html>
    #[inline]
    pub unsafe fn cmd_set_rendering_attachment_locations_khr(
        &self,
        command_buffer: CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo<'_>,
    ) {
        unsafe { (self.cmd_set_rendering_attachment_locations_khr)(command_buffer, location_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRenderingInputAttachmentIndicesKHR.html>
    #[inline]
    pub unsafe fn cmd_set_rendering_input_attachment_indices_khr(
        &self,
        command_buffer: CommandBuffer,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo<'_>,
    ) {
        unsafe {
            (self.cmd_set_rendering_input_attachment_indices_khr)(
                command_buffer,
                input_attachment_index_info,
            )
        }
    }
}
