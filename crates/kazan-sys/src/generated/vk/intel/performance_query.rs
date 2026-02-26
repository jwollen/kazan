#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PerformanceConfigurationINTEL(u64);
pub type QueryPoolCreateInfoINTEL<'a> = QueryPoolPerformanceQueryCreateInfoINTEL<'a>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceValueINTEL<'a> {
    pub ty: PerformanceValueTypeINTEL,
    pub data: PerformanceValueDataINTEL<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerformanceValueINTEL<'_> {
    fn default() -> Self {
        Self {
            ty: Default::default(),
            data: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InitializePerformanceApiInfoINTEL<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_user_data: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for InitializePerformanceApiInfoINTEL<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::INITIALIZE_PERFORMANCE_API_INFO_INTEL,
            p_next: core::ptr::null(),
            p_user_data: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueryPoolPerformanceQueryCreateInfoINTEL<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub performance_counters_sampling: QueryPoolSamplingModeINTEL,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for QueryPoolPerformanceQueryCreateInfoINTEL<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL,
            p_next: core::ptr::null(),
            performance_counters_sampling: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceMarkerInfoINTEL<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub marker: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerformanceMarkerInfoINTEL<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PERFORMANCE_MARKER_INFO_INTEL,
            p_next: core::ptr::null(),
            marker: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceStreamMarkerInfoINTEL<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub marker: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerformanceStreamMarkerInfoINTEL<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PERFORMANCE_STREAM_MARKER_INFO_INTEL,
            p_next: core::ptr::null(),
            marker: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceOverrideInfoINTEL<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: PerformanceOverrideTypeINTEL,
    pub enable: Bool32,
    pub parameter: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerformanceOverrideInfoINTEL<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PERFORMANCE_OVERRIDE_INFO_INTEL,
            p_next: core::ptr::null(),
            ty: Default::default(),
            enable: Default::default(),
            parameter: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceConfigurationAcquireInfoINTEL<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: PerformanceConfigurationTypeINTEL,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerformanceConfigurationAcquireInfoINTEL<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL,
            p_next: core::ptr::null(),
            ty: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union PerformanceValueDataINTEL<'a> {
    pub value32: u32,
    pub value64: u64,
    pub value_float: f32,
    pub value_bool: Bool32,
    pub value_string: *const c_char,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerformanceValueDataINTEL<'_> {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceConfigurationTypeINTEL(i32);
impl PerformanceConfigurationTypeINTEL {
    pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPoolSamplingModeINTEL(i32);
impl QueryPoolSamplingModeINTEL {
    pub const MANUAL_INTEL: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceOverrideTypeINTEL(i32);
impl PerformanceOverrideTypeINTEL {
    pub const NULL_HARDWARE_INTEL: Self = Self(0);
    pub const FLUSH_GPU_CACHES_INTEL: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceParameterTypeINTEL(i32);
impl PerformanceParameterTypeINTEL {
    pub const HW_COUNTERS_SUPPORTED_INTEL: Self = Self(0);
    pub const STREAM_MARKER_VALID_BITS_INTEL: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceValueTypeINTEL(i32);
impl PerformanceValueTypeINTEL {
    pub const UINT32_INTEL: Self = Self(0);
    pub const UINT64_INTEL: Self = Self(1);
    pub const FLOAT_INTEL: Self = Self(2);
    pub const BOOL_INTEL: Self = Self(3);
    pub const STRING_INTEL: Self = Self(4);
}
pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "system" fn(
    device: Device,
    p_initialize_info: *const InitializePerformanceApiInfoINTEL<'_>,
) -> Result;
pub type PFN_vkUninitializePerformanceApiINTEL = unsafe extern "system" fn(device: Device);
pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const PerformanceMarkerInfoINTEL<'_>,
) -> Result;
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const PerformanceStreamMarkerInfoINTEL<'_>,
) -> Result;
pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_override_info: *const PerformanceOverrideInfoINTEL<'_>,
) -> Result;
pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: Device,
    p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL<'_>,
    p_configuration: *mut PerformanceConfigurationINTEL,
) -> Result;
pub type PFN_vkReleasePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: Device,
    configuration: PerformanceConfigurationINTEL,
) -> Result;
pub type PFN_vkQueueSetPerformanceConfigurationINTEL =
    unsafe extern "system" fn(queue: Queue, configuration: PerformanceConfigurationINTEL) -> Result;
pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "system" fn(
    device: Device,
    parameter: PerformanceParameterTypeINTEL,
    p_value: *mut PerformanceValueINTEL<'_>,
) -> Result;
