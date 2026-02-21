#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type PFN_vkCmdSetCullModeEXT = PFN_vkCmdSetCullMode;
pub type PFN_vkCmdSetFrontFaceEXT = PFN_vkCmdSetFrontFace;
pub type PFN_vkCmdSetPrimitiveTopologyEXT = PFN_vkCmdSetPrimitiveTopology;
pub type PFN_vkCmdSetViewportWithCountEXT = PFN_vkCmdSetViewportWithCount;
pub type PFN_vkCmdSetScissorWithCountEXT = PFN_vkCmdSetScissorWithCount;
pub type PFN_vkCmdBindVertexBuffers2EXT = PFN_vkCmdBindVertexBuffers2;
pub type PFN_vkCmdSetDepthTestEnableEXT = PFN_vkCmdSetDepthTestEnable;
pub type PFN_vkCmdSetDepthWriteEnableEXT = PFN_vkCmdSetDepthWriteEnable;
pub type PFN_vkCmdSetDepthCompareOpEXT = PFN_vkCmdSetDepthCompareOp;
pub type PFN_vkCmdSetDepthBoundsTestEnableEXT = PFN_vkCmdSetDepthBoundsTestEnable;
pub type PFN_vkCmdSetStencilTestEnableEXT = PFN_vkCmdSetStencilTestEnable;
pub type PFN_vkCmdSetStencilOpEXT = PFN_vkCmdSetStencilOp;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_dynamic_state: Bool32,
}
