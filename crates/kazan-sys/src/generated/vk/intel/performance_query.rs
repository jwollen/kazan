#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceConfigurationINTEL(u64);
pub type QueryPoolCreateInfoINTEL = QueryPoolPerformanceQueryCreateInfoINTEL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceValueINTEL {
    pub ty: PerformanceValueTypeINTEL,
    pub data: PerformanceValueDataINTEL,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InitializePerformanceApiInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_user_data: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueryPoolPerformanceQueryCreateInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub performance_counters_sampling: QueryPoolSamplingModeINTEL,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceMarkerInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub marker: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceStreamMarkerInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub marker: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceOverrideInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: PerformanceOverrideTypeINTEL,
    pub enable: Bool32,
    pub parameter: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceConfigurationAcquireInfoINTEL {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: PerformanceConfigurationTypeINTEL,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union PerformanceValueDataINTEL {
    pub value32: u32,
    pub value64: u64,
    pub value_float: f32,
    pub value_bool: Bool32,
    pub value_string: *const c_char,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceConfigurationTypeINTEL(i32);
impl PerformanceConfigurationTypeINTEL {
    pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPoolSamplingModeINTEL(i32);
impl QueryPoolSamplingModeINTEL {
    pub const MANUAL_INTEL: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceOverrideTypeINTEL(i32);
impl PerformanceOverrideTypeINTEL {
    pub const NULL_HARDWARE_INTEL: Self = Self(0);
    pub const FLUSH_GPU_CACHES_INTEL: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceParameterTypeINTEL(i32);
impl PerformanceParameterTypeINTEL {
    pub const HW_COUNTERS_SUPPORTED_INTEL: Self = Self(0);
    pub const STREAM_MARKER_VALID_BITS_INTEL: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    p_initialize_info: *const InitializePerformanceApiInfoINTEL,
) -> Result;
pub type PFN_vkUninitializePerformanceApiINTEL = unsafe extern "system" fn(device: Device);
pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const PerformanceMarkerInfoINTEL,
) -> Result;
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const PerformanceStreamMarkerInfoINTEL,
) -> Result;
pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_override_info: *const PerformanceOverrideInfoINTEL,
) -> Result;
pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: Device,
    p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
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
    p_value: *mut PerformanceValueINTEL,
) -> Result;
