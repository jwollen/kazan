#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMemoryFdInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub fd: c_int,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportMemoryFdInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_MEMORY_FD_INFO_KHR,
            p_next: core::ptr::null(),
            handle_type: Default::default(),
            fd: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryFdPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryFdPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_FD_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            memory_type_bits: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryGetFdInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryGetFdInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_GET_FD_INFO_KHR,
            p_next: core::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetMemoryFdKHR = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const MemoryGetFdInfoKHR<'_>,
    p_fd: *mut c_int,
) -> Result;
pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    fd: c_int,
    p_memory_fd_properties: *mut MemoryFdPropertiesKHR<'_>,
) -> Result;
