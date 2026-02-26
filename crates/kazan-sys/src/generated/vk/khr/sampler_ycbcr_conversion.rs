#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
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
