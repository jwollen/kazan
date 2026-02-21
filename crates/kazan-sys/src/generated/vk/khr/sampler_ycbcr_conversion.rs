#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type SamplerYcbcrModelConversionKHR = SamplerYcbcrModelConversion;
pub type SamplerYcbcrRangeKHR = SamplerYcbcrRange;
pub type ChromaLocationKHR = ChromaLocation;
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;
pub type SamplerYcbcrConversionInfoKHR = SamplerYcbcrConversionInfo;
pub type SamplerYcbcrConversionCreateInfoKHR = SamplerYcbcrConversionCreateInfo;
pub type BindImagePlaneMemoryInfoKHR = BindImagePlaneMemoryInfo;
pub type ImagePlaneMemoryRequirementsInfoKHR = ImagePlaneMemoryRequirementsInfo;
pub type PhysicalDeviceSamplerYcbcrConversionFeaturesKHR =
    PhysicalDeviceSamplerYcbcrConversionFeatures;
pub type SamplerYcbcrConversionImageFormatPropertiesKHR =
    SamplerYcbcrConversionImageFormatProperties;
pub type PFN_vkCreateSamplerYcbcrConversionKHR = PFN_vkCreateSamplerYcbcrConversion;
pub type PFN_vkDestroySamplerYcbcrConversionKHR = PFN_vkDestroySamplerYcbcrConversion;
