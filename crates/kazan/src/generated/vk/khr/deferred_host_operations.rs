//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_deferred_host_operations.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_deferred_host_operations";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        DeferredOperationKHR,
        DEFERRED_OPERATION_KHR,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeferredOperationKHR.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDeferredOperationKHR.html>
    pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "system" fn(
        device: Device,
        p_allocator: *const AllocationCallbacks<'_>,
        p_deferred_operation: *mut DeferredOperationKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyDeferredOperationKHR.html>
    pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "system" fn(
        device: Device,
        operation: DeferredOperationKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html>
    pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
        unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> u32;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeferredOperationResultKHR.html>
    pub type PFN_vkGetDeferredOperationResultKHR =
        unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDeferredOperationJoinKHR.html>
    pub type PFN_vkDeferredOperationJoinKHR =
        unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDeferredOperationKHR = DeferredOperationKHR;
}

pub struct DeviceFn {
    create_deferred_operation: PFN_vkCreateDeferredOperationKHR,
    destroy_deferred_operation: PFN_vkDestroyDeferredOperationKHR,
    get_deferred_operation_max_concurrency: PFN_vkGetDeferredOperationMaxConcurrencyKHR,
    get_deferred_operation_result: PFN_vkGetDeferredOperationResultKHR,
    deferred_operation_join: PFN_vkDeferredOperationJoinKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_deferred_operation: transmute(
                    load(c"vkCreateDeferredOperationKHR").ok_or(MissingEntryPointError)?,
                ),
                destroy_deferred_operation: transmute(
                    load(c"vkDestroyDeferredOperationKHR").ok_or(MissingEntryPointError)?,
                ),
                get_deferred_operation_max_concurrency: transmute(
                    load(c"vkGetDeferredOperationMaxConcurrencyKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_deferred_operation_result: transmute(
                    load(c"vkGetDeferredOperationResultKHR").ok_or(MissingEntryPointError)?,
                ),
                deferred_operation_join: transmute(
                    load(c"vkDeferredOperationJoinKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDeferredOperationKHR.html>
    #[inline]
    pub unsafe fn create_deferred_operation(
        &self,
        device: Device,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<DeferredOperationKHR> {
        unsafe {
            let mut deferred_operation = core::mem::MaybeUninit::uninit();
            let result = (self.create_deferred_operation)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyDeferredOperationKHR.html>
    #[inline]
    pub unsafe fn destroy_deferred_operation(
        &self,
        device: Device,
        operation: DeferredOperationKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_deferred_operation)(device, operation, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html>
    #[inline]
    pub unsafe fn get_deferred_operation_max_concurrency(
        &self,
        device: Device,
        operation: DeferredOperationKHR,
    ) -> u32 {
        unsafe { (self.get_deferred_operation_max_concurrency)(device, operation) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeferredOperationResultKHR.html>
    #[inline]
    pub unsafe fn get_deferred_operation_result(
        &self,
        device: Device,
        operation: DeferredOperationKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_deferred_operation_result)(device, operation);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDeferredOperationJoinKHR.html>
    #[inline]
    pub unsafe fn deferred_operation_join(
        &self,
        device: Device,
        operation: DeferredOperationKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.deferred_operation_join)(device, operation);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
