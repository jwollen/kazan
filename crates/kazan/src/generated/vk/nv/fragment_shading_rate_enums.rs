#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_fragment_shading_rate_enum_nv: PFN_vkCmdSetFragmentShadingRateEnumNV,
}
impl DeviceFn {
    pub unsafe fn cmd_set_fragment_shading_rate_enum_nv(
        &self,
        command_buffer: CommandBuffer,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: &[FragmentShadingRateCombinerOpKHR; 2],
    ) {
        unsafe {
            (self.cmd_set_fragment_shading_rate_enum_nv)(command_buffer, shading_rate, combiner_ops)
        }
    }
}
