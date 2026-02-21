#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_compression_control_swapchain: Bool32,
}
