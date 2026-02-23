#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DeferredOperationKHR(u64);
pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "system" fn(
    device: Device,
    p_allocator: *const AllocationCallbacks,
    p_deferred_operation: *mut DeferredOperationKHR,
) -> Result;
pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "system" fn(
    device: Device,
    operation: DeferredOperationKHR,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> u32;
pub type PFN_vkGetDeferredOperationResultKHR =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> Result;
pub type PFN_vkDeferredOperationJoinKHR =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> Result;
