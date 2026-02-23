#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct ImportMemoryMetalHandleInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub handle: *mut c_void,
}
#[repr(C)]
pub struct MemoryMetalHandlePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
}
#[repr(C)]
pub struct MemoryGetMetalHandleInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
pub type PFN_vkGetMemoryMetalHandleEXT = unsafe extern "system" fn(
    device: Device,
    p_get_metal_handle_info: *const MemoryGetMetalHandleInfoEXT,
    p_handle: *mut *mut c_void,
) -> Result;
pub type PFN_vkGetMemoryMetalHandlePropertiesEXT = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    p_handle: *const c_void,
    p_memory_metal_handle_properties: *mut MemoryMetalHandlePropertiesEXT,
) -> Result;
