#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_sampler_ycbcr_conversion_khr: PFN_vkCreateSamplerYcbcrConversion,
    destroy_sampler_ycbcr_conversion_khr: PFN_vkDestroySamplerYcbcrConversion,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_sampler_ycbcr_conversion_khr: transmute(
                    load(c"vkCreateSamplerYcbcrConversionKHR").ok_or(LoadingError)?,
                ),
                destroy_sampler_ycbcr_conversion_khr: transmute(
                    load(c"vkDestroySamplerYcbcrConversionKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_sampler_ycbcr_conversion_khr(
        &self,
        device: Device,
        create_info: &SamplerYcbcrConversionCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        ycbcr_conversion: &mut SamplerYcbcrConversion,
    ) -> Result {
        unsafe {
            (self.create_sampler_ycbcr_conversion_khr)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                ycbcr_conversion,
            )
        }
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
        &self,
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_sampler_ycbcr_conversion_khr)(
                device,
                ycbcr_conversion,
                allocator.to_raw_ptr(),
            )
        }
    }
}
