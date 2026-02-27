#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
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
impl<'a> ExternalComputeQueueDeviceCreateInfoNV<'a> {
    pub fn reserved_external_queues(mut self, reserved_external_queues: u32) -> Self {
        self.reserved_external_queues = reserved_external_queues;
        self
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
impl<'a> ExternalComputeQueueCreateInfoNV<'a> {
    pub fn preferred_queue(mut self, preferred_queue: Queue) -> Self {
        self.preferred_queue = preferred_queue;
        self
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
impl<'a> ExternalComputeQueueDataParamsNV<'a> {
    pub fn device_index(mut self, device_index: u32) -> Self {
        self.device_index = device_index;
        self
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
impl<'a> PhysicalDeviceExternalComputeQueuePropertiesNV<'a> {
    pub fn external_data_size(mut self, external_data_size: u32) -> Self {
        self.external_data_size = external_data_size;
        self
    }
    pub fn max_external_queues(mut self, max_external_queues: u32) -> Self {
        self.max_external_queues = max_external_queues;
        self
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
