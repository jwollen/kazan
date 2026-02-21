#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_bind_shading_rate_image_nv: PFN_vkCmdBindShadingRateImageNV,
    cmd_set_viewport_shading_rate_palette_nv: PFN_vkCmdSetViewportShadingRatePaletteNV,
    cmd_set_coarse_sample_order_nv: PFN_vkCmdSetCoarseSampleOrderNV,
}
impl DeviceFn {
    pub unsafe fn cmd_bind_shading_rate_image_nv(
        &self,
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    ) {
        unsafe { (self.cmd_bind_shading_rate_image_nv)(command_buffer, image_view, image_layout) }
    }
    pub unsafe fn cmd_set_viewport_shading_rate_palette_nv(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        shading_rate_palettes: &[ShadingRatePaletteNV],
    ) {
        unsafe {
            (self.cmd_set_viewport_shading_rate_palette_nv)(
                command_buffer,
                first_viewport,
                shading_rate_palettes.len().try_into().unwrap(),
                shading_rate_palettes.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_coarse_sample_order_nv(
        &self,
        command_buffer: CommandBuffer,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_orders: &[CoarseSampleOrderCustomNV],
    ) {
        unsafe {
            (self.cmd_set_coarse_sample_order_nv)(
                command_buffer,
                sample_order_type,
                custom_sample_orders.len().try_into().unwrap(),
                custom_sample_orders.as_ptr() as _,
            )
        }
    }
}
