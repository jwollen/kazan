#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct ImportMemoryHostPointerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub p_host_pointer: *mut c_void,
}
#[repr(C)]
pub struct MemoryHostPointerPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
}
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_imported_host_pointer_alignment: DeviceSize,
}
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    p_host_pointer: *const c_void,
    p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
) -> Result;
