#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalComputeQueueNV(usize);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalComputeQueueDeviceCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub reserved_external_queues: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalComputeQueueCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub preferred_queue: Queue,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalComputeQueueDataParamsNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalComputeQueuePropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_data_size: u32,
    pub max_external_queues: u32,
}
pub type PFN_vkCreateExternalComputeQueueNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ExternalComputeQueueCreateInfoNV,
    p_allocator: *const AllocationCallbacks,
    p_external_queue: *mut ExternalComputeQueueNV,
) -> Result;
pub type PFN_vkDestroyExternalComputeQueueNV = unsafe extern "system" fn(
    device: Device,
    external_queue: ExternalComputeQueueNV,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetExternalComputeQueueDataNV = unsafe extern "system" fn(
    external_queue: ExternalComputeQueueNV,
    params: *mut ExternalComputeQueueDataParamsNV,
    p_data: *mut c_void,
);
