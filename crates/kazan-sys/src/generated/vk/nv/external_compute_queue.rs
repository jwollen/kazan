#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ExternalComputeQueueNV(usize);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalComputeQueueDeviceCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub reserved_external_queues: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalComputeQueueDeviceCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            reserved_external_queues: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalComputeQueueCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub preferred_queue: Queue,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalComputeQueueCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            preferred_queue: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalComputeQueueDataParamsNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalComputeQueueDataParamsNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV,
            p_next: core::ptr::null(),
            device_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalComputeQueuePropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_data_size: u32,
    pub max_external_queues: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExternalComputeQueuePropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            external_data_size: Default::default(),
            max_external_queues: Default::default(),
            _marker: PhantomData,
        }
    }
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
