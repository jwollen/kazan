#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type RemoteAddressNV = c_void;
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_rdma: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MemoryGetRemoteAddressInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(
    device: Device,
    p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV<'_>,
    p_address: *mut RemoteAddressNV,
) -> Result;
