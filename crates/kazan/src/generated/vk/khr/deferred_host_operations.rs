#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    create_deferred_operation_khr: PFN_vkCreateDeferredOperationKHR,
    destroy_deferred_operation_khr: PFN_vkDestroyDeferredOperationKHR,
    get_deferred_operation_max_concurrency_khr: PFN_vkGetDeferredOperationMaxConcurrencyKHR,
    get_deferred_operation_result_khr: PFN_vkGetDeferredOperationResultKHR,
    deferred_operation_join_khr: PFN_vkDeferredOperationJoinKHR,
}
impl DeviceFn {
    pub unsafe fn create_deferred_operation_khr(
        &self,
        device: Device,
        allocator: Option<&AllocationCallbacks>,
        deferred_operation: &mut DeferredOperationKHR,
    ) -> Result {
        unsafe {
            (self.create_deferred_operation_khr)(device, allocator.to_raw_ptr(), deferred_operation)
        }
    }
    pub unsafe fn destroy_deferred_operation_khr(
        &self,
        device: Device,
        operation: DeferredOperationKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_deferred_operation_khr)(device, operation, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_deferred_operation_max_concurrency_khr(
        &self,
        device: Device,
        operation: DeferredOperationKHR,
    ) -> u32 {
        unsafe { (self.get_deferred_operation_max_concurrency_khr)(device, operation) }
    }
    pub unsafe fn get_deferred_operation_result_khr(
        &self,
        device: Device,
        operation: DeferredOperationKHR,
    ) -> Result {
        unsafe { (self.get_deferred_operation_result_khr)(device, operation) }
    }
    pub unsafe fn deferred_operation_join_khr(
        &self,
        device: Device,
        operation: DeferredOperationKHR,
    ) -> Result {
        unsafe { (self.deferred_operation_join_khr)(device, operation) }
    }
}
