#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub type MemoryMapInfoKHR = MemoryMapInfo;
pub type MemoryUnmapInfoKHR = MemoryUnmapInfo;
pub type MemoryUnmapFlagsKHR = MemoryUnmapFlags;
pub type PFN_vkMapMemory2KHR = PFN_vkMapMemory2;
pub type PFN_vkUnmapMemory2KHR = PFN_vkUnmapMemory2;
