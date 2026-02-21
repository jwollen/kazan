#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_shaders_ext: PFN_vkCreateShadersEXT,
    destroy_shader_ext: PFN_vkDestroyShaderEXT,
    get_shader_binary_data_ext: PFN_vkGetShaderBinaryDataEXT,
    cmd_bind_shaders_ext: PFN_vkCmdBindShadersEXT,
    cmd_set_depth_clamp_range_ext: PFN_vkCmdSetDepthClampRangeEXT,
}
impl DeviceFn {
    pub unsafe fn create_shaders_ext(
        &self,
        device: Device,
        create_infos: &[ShaderCreateInfoEXT],
        allocator: Option<&AllocationCallbacks>,
        shaders: &mut [ShaderEXT],
    ) -> Result {
        unsafe {
            (self.create_shaders_ext)(
                device,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                shaders.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn destroy_shader_ext(
        &self,
        device: Device,
        shader: ShaderEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_shader_ext)(device, shader, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_shader_binary_data_ext(
        &self,
        device: Device,
        shader: ShaderEXT,
        data: impl ExtendUninit<u8>,
    ) -> Result {
        unsafe {
            try_extend_uninit(data, |data_size, data| {
                (self.get_shader_binary_data_ext)(device, shader, data_size, data as _)
            })
        }
    }
    pub unsafe fn cmd_bind_shaders_ext(
        &self,
        command_buffer: CommandBuffer,
        stages: &[ShaderStageFlags],
        shaders: Option<&[ShaderEXT]>,
    ) {
        unsafe {
            (self.cmd_bind_shaders_ext)(
                command_buffer,
                stages.len().try_into().unwrap(),
                stages.as_ptr() as _,
                shaders.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn cmd_set_depth_clamp_range_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clamp_mode: DepthClampModeEXT,
        depth_clamp_range: Option<&DepthClampRangeEXT>,
    ) {
        unsafe {
            (self.cmd_set_depth_clamp_range_ext)(
                command_buffer,
                depth_clamp_mode,
                depth_clamp_range.to_raw_ptr(),
            )
        }
    }
}
