#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acquire_count: u32,
    pub p_acquire_syncs: *const DeviceMemory,
    pub p_acquire_keys: *const u64,
    pub p_acquire_timeouts: *const u32,
    pub release_count: u32,
    pub p_release_syncs: *const DeviceMemory,
    pub p_release_keys: *const u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for Win32KeyedMutexAcquireReleaseInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR,
            p_next: core::ptr::null(),
            acquire_count: Default::default(),
            p_acquire_syncs: core::ptr::null(),
            p_acquire_keys: core::ptr::null(),
            p_acquire_timeouts: core::ptr::null(),
            release_count: Default::default(),
            p_release_syncs: core::ptr::null(),
            p_release_keys: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
