#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PFN_vkCmdSetRasterizerDiscardEnableEXT = PFN_vkCmdSetRasterizerDiscardEnable;
pub type PFN_vkCmdSetDepthBiasEnableEXT = PFN_vkCmdSetDepthBiasEnable;
pub type PFN_vkCmdSetPrimitiveRestartEnableEXT = PFN_vkCmdSetPrimitiveRestartEnable;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_dynamic_state2: Bool32,
    pub extended_dynamic_state2_logic_op: Bool32,
    pub extended_dynamic_state2_patch_control_points: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExtendedDynamicState2FeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            extended_dynamic_state2: Default::default(),
            extended_dynamic_state2_logic_op: Default::default(),
            extended_dynamic_state2_patch_control_points: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceExtendedDynamicState2FeaturesEXT<'a> {
    pub fn extended_dynamic_state2(mut self, extended_dynamic_state2: Bool32) -> Self {
        self.extended_dynamic_state2 = extended_dynamic_state2;
        self
    }
    pub fn extended_dynamic_state2_logic_op(
        mut self,
        extended_dynamic_state2_logic_op: Bool32,
    ) -> Self {
        self.extended_dynamic_state2_logic_op = extended_dynamic_state2_logic_op;
        self
    }
    pub fn extended_dynamic_state2_patch_control_points(
        mut self,
        extended_dynamic_state2_patch_control_points: Bool32,
    ) -> Self {
        self.extended_dynamic_state2_patch_control_points =
            extended_dynamic_state2_patch_control_points;
        self
    }
}
pub type PFN_vkCmdSetPatchControlPointsEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, patch_control_points: u32);
pub type PFN_vkCmdSetLogicOpEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op: LogicOp);
