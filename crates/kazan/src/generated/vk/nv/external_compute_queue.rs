#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    create_external_compute_queue_nv: PFN_vkCreateExternalComputeQueueNV,
    destroy_external_compute_queue_nv: PFN_vkDestroyExternalComputeQueueNV,
    get_external_compute_queue_data_nv: PFN_vkGetExternalComputeQueueDataNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_external_compute_queue_nv: transmute(
                    load(c"vkCreateExternalComputeQueueNV").ok_or(LoadingError)?,
                ),
                destroy_external_compute_queue_nv: transmute(
                    load(c"vkDestroyExternalComputeQueueNV").ok_or(LoadingError)?,
                ),
                get_external_compute_queue_data_nv: transmute(
                    load(c"vkGetExternalComputeQueueDataNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_external_compute_queue_nv(
        &self,
        device: Device,
        create_info: &ExternalComputeQueueCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<ExternalComputeQueueNV> {
        unsafe {
            let mut external_queue = core::mem::MaybeUninit::uninit();
            let result = (self.create_external_compute_queue_nv)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                external_queue.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(external_queue.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_external_compute_queue_nv(
        &self,
        device: Device,
        external_queue: ExternalComputeQueueNV,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_external_compute_queue_nv)(device, external_queue, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn get_external_compute_queue_data_nv(
        &self,
        external_queue: ExternalComputeQueueNV,
        data: &mut c_void,
    ) -> ExternalComputeQueueDataParamsNV {
        unsafe {
            let mut params = core::mem::MaybeUninit::uninit();
            (self.get_external_compute_queue_data_nv)(external_queue, params.as_mut_ptr(), data);
            params.assume_init()
        }
    }
}
