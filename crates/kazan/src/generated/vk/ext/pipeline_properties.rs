#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    get_pipeline_properties_ext: PFN_vkGetPipelinePropertiesEXT,
}
impl DeviceFn {
    pub unsafe fn get_pipeline_properties_ext(
        &self,
        device: Device,
        pipeline_info: &PipelineInfoEXT,
        pipeline_properties: &mut BaseOutStructure,
    ) -> Result {
        unsafe { (self.get_pipeline_properties_ext)(device, pipeline_info, pipeline_properties) }
    }
}
