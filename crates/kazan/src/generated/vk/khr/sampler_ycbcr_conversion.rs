#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_sampler_ycbcr_conversion: PFN_vkCreateSamplerYcbcrConversion,
    destroy_sampler_ycbcr_conversion: PFN_vkDestroySamplerYcbcrConversion,
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
            (self.create_sampler_ycbcr_conversion)(
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
            (self.destroy_sampler_ycbcr_conversion)(
                device,
                ycbcr_conversion,
                allocator.to_raw_ptr(),
            )
        }
    }
}
