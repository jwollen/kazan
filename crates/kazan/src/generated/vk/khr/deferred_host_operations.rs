#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    create_deferred_operation_khr: PFN_vkCreateDeferredOperationKHR,
    destroy_deferred_operation_khr: PFN_vkDestroyDeferredOperationKHR,
    get_deferred_operation_max_concurrency_khr: PFN_vkGetDeferredOperationMaxConcurrencyKHR,
    get_deferred_operation_result_khr: PFN_vkGetDeferredOperationResultKHR,
    deferred_operation_join_khr: PFN_vkDeferredOperationJoinKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_deferred_operation_khr: transmute(
                    load(c"vkCreateDeferredOperationKHR").ok_or(LoadingError)?,
                ),
                destroy_deferred_operation_khr: transmute(
                    load(c"vkDestroyDeferredOperationKHR").ok_or(LoadingError)?,
                ),
                get_deferred_operation_max_concurrency_khr: transmute(
                    load(c"vkGetDeferredOperationMaxConcurrencyKHR").ok_or(LoadingError)?,
                ),
                get_deferred_operation_result_khr: transmute(
                    load(c"vkGetDeferredOperationResultKHR").ok_or(LoadingError)?,
                ),
                deferred_operation_join_khr: transmute(
                    load(c"vkDeferredOperationJoinKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_deferred_operation_khr(
        &self,
        device: Device,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<DeferredOperationKHR> {
        unsafe {
            let mut deferred_operation = core::mem::MaybeUninit::uninit();
            let result = (self.create_deferred_operation_khr)(
                device,
                allocator.to_raw_ptr(),
                deferred_operation.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(deferred_operation.assume_init()),
                err => Err(err),
            }
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
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_deferred_operation_result_khr)(device, operation);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::NOT_READY => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn deferred_operation_join_khr(
        &self,
        device: Device,
        operation: DeferredOperationKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.deferred_operation_join_khr)(device, operation);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::THREAD_DONE_KHR => Ok(()),
                VkResult::THREAD_IDLE_KHR => Ok(()),
                err => Err(err),
            }
        }
    }
}
