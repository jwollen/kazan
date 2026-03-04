#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    handle_nondispatchable!(DeferredOperationKHR, DEFERRED_OPERATION_KHR, doc = "");
    pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "system" fn(
        device: Device,
        p_allocator: *const AllocationCallbacks<'_>,
        p_deferred_operation: *mut DeferredOperationKHR,
    ) -> vk::Result;
    pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "system" fn(
        device: Device,
        operation: DeferredOperationKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
        unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> u32;
    pub type PFN_vkGetDeferredOperationResultKHR =
        unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> vk::Result;
    pub type PFN_vkDeferredOperationJoinKHR =
        unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> vk::Result;
}
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
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_deferred_operation_khr: transmute(
                    load(c"vkCreateDeferredOperationKHR").ok_or(MissingEntryPointError)?,
                ),
                destroy_deferred_operation_khr: transmute(
                    load(c"vkDestroyDeferredOperationKHR").ok_or(MissingEntryPointError)?,
                ),
                get_deferred_operation_max_concurrency_khr: transmute(
                    load(c"vkGetDeferredOperationMaxConcurrencyKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_deferred_operation_result_khr: transmute(
                    load(c"vkGetDeferredOperationResultKHR").ok_or(MissingEntryPointError)?,
                ),
                deferred_operation_join_khr: transmute(
                    load(c"vkDeferredOperationJoinKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_deferred_operation_khr(
        &self,
        device: Device,
        allocator: Option<&AllocationCallbacks<'_>>,
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
        allocator: Option<&AllocationCallbacks<'_>>,
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
