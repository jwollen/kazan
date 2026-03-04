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
    pub type SamplerYcbcrModelConversionKHR = SamplerYcbcrModelConversion;
    pub type SamplerYcbcrRangeKHR = SamplerYcbcrRange;
    pub type ChromaLocationKHR = ChromaLocation;
    pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;
    pub type SamplerYcbcrConversionInfoKHR<'a> = SamplerYcbcrConversionInfo<'a>;
    pub type SamplerYcbcrConversionCreateInfoKHR<'a> = SamplerYcbcrConversionCreateInfo<'a>;
    pub type BindImagePlaneMemoryInfoKHR<'a> = BindImagePlaneMemoryInfo<'a>;
    pub type ImagePlaneMemoryRequirementsInfoKHR<'a> = ImagePlaneMemoryRequirementsInfo<'a>;
    pub type PhysicalDeviceSamplerYcbcrConversionFeaturesKHR<'a> =
        PhysicalDeviceSamplerYcbcrConversionFeatures<'a>;
    pub type SamplerYcbcrConversionImageFormatPropertiesKHR<'a> =
        SamplerYcbcrConversionImageFormatProperties<'a>;
    pub type PFN_vkCreateSamplerYcbcrConversionKHR = PFN_vkCreateSamplerYcbcrConversion;
    pub type PFN_vkDestroySamplerYcbcrConversionKHR = PFN_vkDestroySamplerYcbcrConversion;
}
pub struct DeviceFn {
    create_sampler_ycbcr_conversion_khr: PFN_vkCreateSamplerYcbcrConversion,
    destroy_sampler_ycbcr_conversion_khr: PFN_vkDestroySamplerYcbcrConversion,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_sampler_ycbcr_conversion_khr: transmute(
                    load(c"vkCreateSamplerYcbcrConversionKHR").ok_or(MissingEntryPointError)?,
                ),
                destroy_sampler_ycbcr_conversion_khr: transmute(
                    load(c"vkDestroySamplerYcbcrConversionKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_sampler_ycbcr_conversion_khr(
        &self,
        device: Device,
        create_info: &SamplerYcbcrConversionCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SamplerYcbcrConversion> {
        unsafe {
            let mut ycbcr_conversion = core::mem::MaybeUninit::uninit();
            let result = (self.create_sampler_ycbcr_conversion_khr)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                ycbcr_conversion.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(ycbcr_conversion.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
        &self,
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        allocator: Option<&AllocationCallbacks<'_>>,
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
