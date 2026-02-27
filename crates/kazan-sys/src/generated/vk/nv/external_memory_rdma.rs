#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type RemoteAddressNV = c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_rdma: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExternalMemoryRDMAFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            external_memory_rdma: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {
    pub fn external_memory_rdma(mut self, external_memory_rdma: Bool32) -> Self {
        self.external_memory_rdma = external_memory_rdma;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryGetRemoteAddressInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryGetRemoteAddressInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_GET_REMOTE_ADDRESS_INFO_NV,
            p_next: core::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> MemoryGetRemoteAddressInfoNV<'a> {
    pub fn memory(mut self, memory: DeviceMemory) -> Self {
        self.memory = memory;
        self
    }
    pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(
    device: Device,
    p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV<'_>,
    p_address: *mut RemoteAddressNV,
) -> Result;
