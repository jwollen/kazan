#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    pub type LineRasterizationModeEXT = LineRasterizationMode;
    pub type PhysicalDeviceLineRasterizationFeaturesEXT<'a> =
        PhysicalDeviceLineRasterizationFeatures<'a>;
    pub type PhysicalDeviceLineRasterizationPropertiesEXT<'a> =
        PhysicalDeviceLineRasterizationProperties<'a>;
    pub type PipelineRasterizationLineStateCreateInfoEXT<'a> =
        PipelineRasterizationLineStateCreateInfo<'a>;
    pub type PFN_vkCmdSetLineStippleEXT = PFN_vkCmdSetLineStipple;
}
pub struct DeviceFn {
    cmd_set_line_stipple_ext: PFN_vkCmdSetLineStipple,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_line_stipple_ext: transmute(
                    load(c"vkCmdSetLineStippleEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_line_stipple_ext(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        unsafe {
            (self.cmd_set_line_stipple_ext)(
                command_buffer,
                line_stipple_factor,
                line_stipple_pattern,
            )
        }
    }
}
