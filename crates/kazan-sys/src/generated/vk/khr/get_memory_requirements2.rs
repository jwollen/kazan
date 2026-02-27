#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type BufferMemoryRequirementsInfo2KHR<'a> = BufferMemoryRequirementsInfo2<'a>;
pub type ImageMemoryRequirementsInfo2KHR<'a> = ImageMemoryRequirementsInfo2<'a>;
pub type ImageSparseMemoryRequirementsInfo2KHR<'a> = ImageSparseMemoryRequirementsInfo2<'a>;
pub type MemoryRequirements2KHR<'a> = MemoryRequirements2<'a>;
pub type SparseImageMemoryRequirements2KHR<'a> = SparseImageMemoryRequirements2<'a>;
pub type PFN_vkGetBufferMemoryRequirements2KHR = PFN_vkGetBufferMemoryRequirements2;
pub type PFN_vkGetImageMemoryRequirements2KHR = PFN_vkGetImageMemoryRequirements2;
pub type PFN_vkGetImageSparseMemoryRequirements2KHR = PFN_vkGetImageSparseMemoryRequirements2;
