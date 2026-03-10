//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_sampler_ycbcr_conversion.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_sampler_ycbcr_conversion";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerYcbcrModelConversionKHR.html>
    pub type SamplerYcbcrModelConversionKHR = SamplerYcbcrModelConversion;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerYcbcrRangeKHR.html>
    pub type SamplerYcbcrRangeKHR = SamplerYcbcrRange;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkChromaLocationKHR.html>
    pub type ChromaLocationKHR = ChromaLocation;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerYcbcrConversionKHR.html>
    pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerYcbcrConversionInfoKHR.html>
    pub type SamplerYcbcrConversionInfoKHR<'a> = SamplerYcbcrConversionInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerYcbcrConversionCreateInfoKHR.html>
    pub type SamplerYcbcrConversionCreateInfoKHR<'a> = SamplerYcbcrConversionCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindImagePlaneMemoryInfoKHR.html>
    pub type BindImagePlaneMemoryInfoKHR<'a> = BindImagePlaneMemoryInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImagePlaneMemoryRequirementsInfoKHR.html>
    pub type ImagePlaneMemoryRequirementsInfoKHR<'a> = ImagePlaneMemoryRequirementsInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR.html>
    pub type PhysicalDeviceSamplerYcbcrConversionFeaturesKHR<'a> =
        PhysicalDeviceSamplerYcbcrConversionFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerYcbcrConversionImageFormatPropertiesKHR.html>
    pub type SamplerYcbcrConversionImageFormatPropertiesKHR<'a> =
        SamplerYcbcrConversionImageFormatProperties<'a>;
    pub type PFN_vkCreateSamplerYcbcrConversionKHR = PFN_vkCreateSamplerYcbcrConversion;
    pub type PFN_vkDestroySamplerYcbcrConversionKHR = PFN_vkDestroySamplerYcbcrConversion;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkSamplerYcbcrModelConversionKHR = SamplerYcbcrModelConversionKHR;
    pub type VkSamplerYcbcrRangeKHR = SamplerYcbcrRangeKHR;
    pub type VkChromaLocationKHR = ChromaLocationKHR;
    pub type VkSamplerYcbcrConversionKHR = SamplerYcbcrConversionKHR;
    pub type VkSamplerYcbcrConversionInfoKHR = SamplerYcbcrConversionInfoKHR<'static>;
    pub type VkSamplerYcbcrConversionCreateInfoKHR = SamplerYcbcrConversionCreateInfoKHR<'static>;
    pub type VkBindImagePlaneMemoryInfoKHR = BindImagePlaneMemoryInfoKHR<'static>;
    pub type VkImagePlaneMemoryRequirementsInfoKHR = ImagePlaneMemoryRequirementsInfoKHR<'static>;
    pub type VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR =
        PhysicalDeviceSamplerYcbcrConversionFeaturesKHR<'static>;
    pub type VkSamplerYcbcrConversionImageFormatPropertiesKHR =
        SamplerYcbcrConversionImageFormatPropertiesKHR<'static>;
}

pub struct DeviceFn {
    create_sampler_ycbcr_conversion_khr: PFN_vkCreateSamplerYcbcrConversion,
    destroy_sampler_ycbcr_conversion_khr: PFN_vkDestroySamplerYcbcrConversion,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateSamplerYcbcrConversionKHR.html>
    #[inline]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroySamplerYcbcrConversionKHR.html>
    #[inline]
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
