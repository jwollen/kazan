#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub diagnostics_config: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceDiagnosticsConfigCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceDiagnosticsConfigFlagsNV,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DeviceDiagnosticsConfigFlagsNV: Flags {
        const ENABLE_SHADER_DEBUG_INFO_NV = DeviceDiagnosticsConfigFlagBitsNV::ENABLE_SHADER_DEBUG_INFO_NV.0;
        const ENABLE_RESOURCE_TRACKING_NV = DeviceDiagnosticsConfigFlagBitsNV::ENABLE_RESOURCE_TRACKING_NV.0;
        const ENABLE_AUTOMATIC_CHECKPOINTS_NV = DeviceDiagnosticsConfigFlagBitsNV::ENABLE_AUTOMATIC_CHECKPOINTS_NV.0;
        const ENABLE_SHADER_ERROR_REPORTING_NV = DeviceDiagnosticsConfigFlagBitsNV::ENABLE_SHADER_ERROR_REPORTING_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceDiagnosticsConfigFlagBitsNV(u32);
impl DeviceDiagnosticsConfigFlagBitsNV {
    pub const ENABLE_SHADER_DEBUG_INFO_NV: Self = Self(1 << 0);
    pub const ENABLE_RESOURCE_TRACKING_NV: Self = Self(1 << 1);
    pub const ENABLE_AUTOMATIC_CHECKPOINTS_NV: Self = Self(1 << 2);
    pub const ENABLE_SHADER_ERROR_REPORTING_NV: Self = Self(1 << 3);
}
