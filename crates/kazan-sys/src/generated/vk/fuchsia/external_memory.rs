#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMemoryZirconHandleInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub handle: zx_handle_t,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportMemoryZirconHandleInfoFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA,
            p_next: core::ptr::null(),
            handle_type: Default::default(),
            handle: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryZirconHandlePropertiesFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryZirconHandlePropertiesFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA,
            p_next: core::ptr::null_mut(),
            memory_type_bits: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryGetZirconHandleInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryGetZirconHandleInfoFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA,
            p_next: core::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA<'_>,
    p_zircon_handle: *mut zx_handle_t,
) -> Result;
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    zircon_handle: zx_handle_t,
    p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA<'_>,
) -> Result;
