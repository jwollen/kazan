#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_framebuffer_tile_properties_qcom: PFN_vkGetFramebufferTilePropertiesQCOM,
    get_dynamic_rendering_tile_properties_qcom: PFN_vkGetDynamicRenderingTilePropertiesQCOM,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_framebuffer_tile_properties_qcom: transmute(
                    load(c"vkGetFramebufferTilePropertiesQCOM").ok_or(LoadingError)?,
                ),
                get_dynamic_rendering_tile_properties_qcom: transmute(
                    load(c"vkGetDynamicRenderingTilePropertiesQCOM").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_framebuffer_tile_properties_qcom(
        &self,
        device: Device,
        framebuffer: Framebuffer,
        properties: impl ExtendUninit<TilePropertiesQCOM>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |properties_count, properties| {
                let result = (self.get_framebuffer_tile_properties_qcom)(
                    device,
                    framebuffer,
                    properties_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn get_dynamic_rendering_tile_properties_qcom(
        &self,
        device: Device,
        rendering_info: &RenderingInfo,
    ) -> crate::Result<TilePropertiesQCOM> {
        unsafe {
            let mut properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_dynamic_rendering_tile_properties_qcom)(
                device,
                rendering_info,
                properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(properties.assume_init()),
                err => Err(err),
            }
        }
    }
}
