#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_pipeline_properties_ext: PFN_vkGetPipelinePropertiesEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_pipeline_properties_ext: transmute(
                    load(c"vkGetPipelinePropertiesEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_pipeline_properties_ext(
        &self,
        device: Device,
        pipeline_info: &PipelineInfoEXT,
    ) -> crate::Result<BaseOutStructure> {
        unsafe {
            let mut pipeline_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_pipeline_properties_ext)(
                device,
                pipeline_info,
                pipeline_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(pipeline_properties.assume_init()),
                err => Err(err),
            }
        }
    }
}
