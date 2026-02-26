#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub performance_counter_query_pools: Bool32,
    pub performance_counter_multiple_query_pools: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePerformanceQueryFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            performance_counter_query_pools: Default::default(),
            performance_counter_multiple_query_pools: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allow_command_buffer_query_copies: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePerformanceQueryPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            allow_command_buffer_query_copies: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceCounterKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub unit: PerformanceCounterUnitKHR,
    pub scope: PerformanceCounterScopeKHR,
    pub storage: PerformanceCounterStorageKHR,
    pub uuid: [u8; UUID_SIZE as usize],
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerformanceCounterKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PERFORMANCE_COUNTER_KHR,
            p_next: core::ptr::null_mut(),
            unit: Default::default(),
            scope: Default::default(),
            storage: Default::default(),
            uuid: [Default::default(); _],
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceCounterDescriptionKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: PerformanceCounterDescriptionFlagsKHR,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub category: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerformanceCounterDescriptionKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PERFORMANCE_COUNTER_DESCRIPTION_KHR,
            p_next: core::ptr::null_mut(),
            flags: Default::default(),
            name: [Default::default(); _],
            category: [Default::default(); _],
            description: [Default::default(); _],
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueryPoolPerformanceCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_family_index: u32,
    pub counter_index_count: u32,
    pub p_counter_indices: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for QueryPoolPerformanceCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            queue_family_index: Default::default(),
            counter_index_count: Default::default(),
            p_counter_indices: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AcquireProfilingLockInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: AcquireProfilingLockFlagsKHR,
    pub timeout: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AcquireProfilingLockInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACQUIRE_PROFILING_LOCK_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            timeout: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceQuerySubmitInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub counter_pass_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerformanceQuerySubmitInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PERFORMANCE_QUERY_SUBMIT_INFO_KHR,
            p_next: core::ptr::null(),
            counter_pass_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union PerformanceCounterResultKHR {
    pub int32: i32,
    pub int64: i64,
    pub uint32: u32,
    pub uint64: u64,
    pub float32: f32,
    pub float64: f64,
}
impl Default for PerformanceCounterResultKHR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceCounterScopeKHR(i32);
impl PerformanceCounterScopeKHR {
    pub const COMMAND_BUFFER_KHR: Self = Self(0);
    pub const RENDER_PASS_KHR: Self = Self(1);
    pub const COMMAND_KHR: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceCounterUnitKHR(i32);
impl PerformanceCounterUnitKHR {
    pub const GENERIC_KHR: Self = Self(0);
    pub const PERCENTAGE_KHR: Self = Self(1);
    pub const NANOSECONDS_KHR: Self = Self(2);
    pub const BYTES_KHR: Self = Self(3);
    pub const BYTES_PER_SECOND_KHR: Self = Self(4);
    pub const KELVIN_KHR: Self = Self(5);
    pub const WATTS_KHR: Self = Self(6);
    pub const VOLTS_KHR: Self = Self(7);
    pub const AMPS_KHR: Self = Self(8);
    pub const HERTZ_KHR: Self = Self(9);
    pub const CYCLES_KHR: Self = Self(10);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceCounterStorageKHR(i32);
impl PerformanceCounterStorageKHR {
    pub const INT32_KHR: Self = Self(0);
    pub const INT64_KHR: Self = Self(1);
    pub const UINT32_KHR: Self = Self(2);
    pub const UINT64_KHR: Self = Self(3);
    pub const FLOAT32_KHR: Self = Self(4);
    pub const FLOAT64_KHR: Self = Self(5);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerformanceCounterDescriptionFlagsKHR: Flags {
        const PERFORMANCE_IMPACTING_KHR = PerformanceCounterDescriptionFlagBitsKHR::PERFORMANCE_IMPACTING_KHR.0;
        const CONCURRENTLY_IMPACTED_KHR = PerformanceCounterDescriptionFlagBitsKHR::CONCURRENTLY_IMPACTED_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceCounterDescriptionFlagBitsKHR(u32);
impl PerformanceCounterDescriptionFlagBitsKHR {
    pub const PERFORMANCE_IMPACTING_KHR: Self = Self(1 << 0);
    pub const CONCURRENTLY_IMPACTED_KHR: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AcquireProfilingLockFlagsKHR: Flags {
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AcquireProfilingLockFlagBitsKHR(u32);
impl AcquireProfilingLockFlagBitsKHR {}
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterKHR<'_>,
        p_counter_descriptions: *mut PerformanceCounterDescriptionKHR<'_>,
    ) -> Result;
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR<'_>,
        p_num_passes: *mut u32,
    );
pub type PFN_vkAcquireProfilingLockKHR = unsafe extern "system" fn(
    device: Device,
    p_info: *const AcquireProfilingLockInfoKHR<'_>,
) -> Result;
pub type PFN_vkReleaseProfilingLockKHR = unsafe extern "system" fn(device: Device);
