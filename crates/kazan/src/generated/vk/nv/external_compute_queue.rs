#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    create_external_compute_queue_nv: PFN_vkCreateExternalComputeQueueNV,
    destroy_external_compute_queue_nv: PFN_vkDestroyExternalComputeQueueNV,
    get_external_compute_queue_data_nv: PFN_vkGetExternalComputeQueueDataNV,
}
impl DeviceFn {
    pub unsafe fn create_external_compute_queue_nv(
        &self,
        device: Device,
        create_info: &ExternalComputeQueueCreateInfoNV,
        allocator: &AllocationCallbacks,
        external_queue: &mut ExternalComputeQueueNV,
    ) -> Result {
        unsafe {
            (self.create_external_compute_queue_nv)(device, create_info, allocator, external_queue)
        }
    }
    pub unsafe fn destroy_external_compute_queue_nv(
        &self,
        device: Device,
        external_queue: ExternalComputeQueueNV,
        allocator: &AllocationCallbacks,
    ) {
        unsafe { (self.destroy_external_compute_queue_nv)(device, external_queue, allocator) }
    }
    pub unsafe fn get_external_compute_queue_data_nv(
        &self,
        external_queue: ExternalComputeQueueNV,
        params: &mut ExternalComputeQueueDataParamsNV,
        data: &mut c_void,
    ) {
        unsafe { (self.get_external_compute_queue_data_nv)(external_queue, params, data) }
    }
}
