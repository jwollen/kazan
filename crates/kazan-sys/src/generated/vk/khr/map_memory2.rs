#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type MemoryMapInfoKHR<'a> = MemoryMapInfo<'a>;
pub type MemoryUnmapInfoKHR<'a> = MemoryUnmapInfo<'a>;
pub type MemoryUnmapFlagsKHR = MemoryUnmapFlags;
pub type PFN_vkMapMemory2KHR = PFN_vkMapMemory2;
pub type PFN_vkUnmapMemory2KHR = PFN_vkUnmapMemory2;
