#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type PFN_vkCmdSetRasterizerDiscardEnableEXT = PFN_vkCmdSetRasterizerDiscardEnable;
pub type PFN_vkCmdSetDepthBiasEnableEXT = PFN_vkCmdSetDepthBiasEnable;
pub type PFN_vkCmdSetPrimitiveRestartEnableEXT = PFN_vkCmdSetPrimitiveRestartEnable;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_dynamic_state2: Bool32,
    pub extended_dynamic_state2_logic_op: Bool32,
    pub extended_dynamic_state2_patch_control_points: Bool32,
}
pub type PFN_vkCmdSetPatchControlPointsEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, patch_control_points: u32);
pub type PFN_vkCmdSetLogicOpEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op: LogicOp);
