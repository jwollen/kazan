#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceAntiLagFeaturesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub anti_lag: Bool32,
}
#[repr(C)]
pub struct AntiLagDataAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mode: AntiLagModeAMD,
    pub max_fps: u32,
    pub p_presentation_info: *const AntiLagPresentationInfoAMD,
}
#[repr(C)]
pub struct AntiLagPresentationInfoAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stage: AntiLagStageAMD,
    pub frame_index: u64,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AntiLagModeAMD(i32);
impl AntiLagModeAMD {
    pub const DRIVER_CONTROL_AMD: Self = Self(0);
    pub const ON_AMD: Self = Self(1);
    pub const OFF_AMD: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AntiLagStageAMD(i32);
impl AntiLagStageAMD {
    pub const INPUT_AMD: Self = Self(0);
    pub const PRESENT_AMD: Self = Self(1);
}
pub type PFN_vkAntiLagUpdateAMD =
    unsafe extern "system" fn(device: Device, p_data: *const AntiLagDataAMD);
