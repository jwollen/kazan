#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMemoryHostPointerInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub p_host_pointer: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportMemoryHostPointerInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_MEMORY_HOST_POINTER_INFO_EXT,
            p_next: core::ptr::null(),
            handle_type: Default::default(),
            p_host_pointer: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryHostPointerPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryHostPointerPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_HOST_POINTER_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            memory_type_bits: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_imported_host_pointer_alignment: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExternalMemoryHostPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            min_imported_host_pointer_alignment: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    p_host_pointer: *const c_void,
    p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT<'_>,
) -> Result;
