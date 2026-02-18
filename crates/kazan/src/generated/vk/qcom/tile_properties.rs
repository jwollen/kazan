#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    get_framebuffer_tile_properties_qcom: PFN_vkGetFramebufferTilePropertiesQCOM,
    get_dynamic_rendering_tile_properties_qcom: PFN_vkGetDynamicRenderingTilePropertiesQCOM,
}
impl DeviceFn {
    pub unsafe fn get_framebuffer_tile_properties_qcom(
        &self,
        device: Device,
        framebuffer: Framebuffer,
        properties: impl ExtendUninit<TilePropertiesQCOM>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |properties_count, properties| {
                (self.get_framebuffer_tile_properties_qcom)(
                    device,
                    framebuffer,
                    properties_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn get_dynamic_rendering_tile_properties_qcom(
        &self,
        device: Device,
        rendering_info: &RenderingInfo,
        properties: &mut TilePropertiesQCOM,
    ) -> Result {
        unsafe {
            (self.get_dynamic_rendering_tile_properties_qcom)(device, rendering_info, properties)
        }
    }
}
