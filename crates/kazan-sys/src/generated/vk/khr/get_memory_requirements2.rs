#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub type BufferMemoryRequirementsInfo2KHR = BufferMemoryRequirementsInfo2;
pub type ImageMemoryRequirementsInfo2KHR = ImageMemoryRequirementsInfo2;
pub type ImageSparseMemoryRequirementsInfo2KHR = ImageSparseMemoryRequirementsInfo2;
pub type MemoryRequirements2KHR = MemoryRequirements2;
pub type SparseImageMemoryRequirements2KHR = SparseImageMemoryRequirements2;
pub type PFN_vkGetBufferMemoryRequirements2KHR = PFN_vkGetBufferMemoryRequirements2;
pub type PFN_vkGetImageMemoryRequirements2KHR = PFN_vkGetImageMemoryRequirements2;
pub type PFN_vkGetImageSparseMemoryRequirements2KHR = PFN_vkGetImageSparseMemoryRequirements2;
