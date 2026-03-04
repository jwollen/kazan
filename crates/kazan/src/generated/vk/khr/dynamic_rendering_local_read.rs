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
    pub type PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR<'a> =
        PhysicalDeviceDynamicRenderingLocalReadFeatures<'a>;
    pub type RenderingAttachmentLocationInfoKHR<'a> = RenderingAttachmentLocationInfo<'a>;
    pub type RenderingInputAttachmentIndexInfoKHR<'a> = RenderingInputAttachmentIndexInfo<'a>;
    pub type PFN_vkCmdSetRenderingAttachmentLocationsKHR = PFN_vkCmdSetRenderingAttachmentLocations;
    pub type PFN_vkCmdSetRenderingInputAttachmentIndicesKHR =
        PFN_vkCmdSetRenderingInputAttachmentIndices;
}
pub struct DeviceFn {
    cmd_set_rendering_attachment_locations_khr: PFN_vkCmdSetRenderingAttachmentLocations,
    cmd_set_rendering_input_attachment_indices_khr: PFN_vkCmdSetRenderingInputAttachmentIndices,
}
impl DeviceFn {
    pub unsafe fn load(
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
    pub unsafe fn cmd_set_rendering_attachment_locations_khr(
        &self,
        command_buffer: CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo<'_>,
    ) {
        unsafe { (self.cmd_set_rendering_attachment_locations_khr)(command_buffer, location_info) }
    }
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
