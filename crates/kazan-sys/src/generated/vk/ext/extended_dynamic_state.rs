#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
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
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_dynamic_state: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExtendedDynamicStateFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            extended_dynamic_state: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceExtendedDynamicStateFeaturesEXT<'a> {
    pub fn extended_dynamic_state(mut self, extended_dynamic_state: Bool32) -> Self {
        self.extended_dynamic_state = extended_dynamic_state;
        self
    }
}
