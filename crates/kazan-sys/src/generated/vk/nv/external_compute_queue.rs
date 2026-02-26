#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ExternalComputeQueueNV(usize);
#[repr(C)]
pub struct ExternalComputeQueueDeviceCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub reserved_external_queues: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ExternalComputeQueueCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub preferred_queue: Queue,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ExternalComputeQueueDataParamsNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceExternalComputeQueuePropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_data_size: u32,
    pub max_external_queues: u32,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkCreateExternalComputeQueueNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ExternalComputeQueueCreateInfoNV<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_external_queue: *mut ExternalComputeQueueNV,
) -> Result;
pub type PFN_vkDestroyExternalComputeQueueNV = unsafe extern "system" fn(
    device: Device,
    external_queue: ExternalComputeQueueNV,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetExternalComputeQueueDataNV = unsafe extern "system" fn(
    external_queue: ExternalComputeQueueNV,
    params: *mut ExternalComputeQueueDataParamsNV<'_>,
    p_data: *mut c_void,
);
