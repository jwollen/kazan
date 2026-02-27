#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type BindBufferMemoryInfoKHR<'a> = BindBufferMemoryInfo<'a>;
pub type BindImageMemoryInfoKHR<'a> = BindImageMemoryInfo<'a>;
pub type PFN_vkBindBufferMemory2KHR = PFN_vkBindBufferMemory2;
pub type PFN_vkBindImageMemory2KHR = PFN_vkBindImageMemory2;
