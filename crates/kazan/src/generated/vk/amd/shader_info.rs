#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_shader_info_amd: PFN_vkGetShaderInfoAMD,
}
impl DeviceFn {
    pub unsafe fn get_shader_info_amd(
        &self,
        device: Device,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        info: impl ExtendUninit<u8>,
    ) -> Result {
        unsafe {
            try_extend_uninit(info, |info_size, info| {
                (self.get_shader_info_amd)(
                    device,
                    pipeline,
                    shader_stage,
                    info_type,
                    info_size,
                    info as _,
                )
            })
        }
    }
}
