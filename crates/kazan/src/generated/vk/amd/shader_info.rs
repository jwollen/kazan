#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_shader_info_amd: PFN_vkGetShaderInfoAMD,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_shader_info_amd: transmute(load(c"vkGetShaderInfoAMD").ok_or(LoadingError)?),
            })
        }
    }
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
