#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceAntiLagFeaturesAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub anti_lag: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceAntiLagFeaturesAMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD,
            p_next: core::ptr::null_mut(),
            anti_lag: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AntiLagDataAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mode: AntiLagModeAMD,
    pub max_fps: u32,
    pub p_presentation_info: *const AntiLagPresentationInfoAMD<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AntiLagDataAMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ANTI_LAG_DATA_AMD,
            p_next: core::ptr::null(),
            mode: Default::default(),
            max_fps: Default::default(),
            p_presentation_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AntiLagPresentationInfoAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stage: AntiLagStageAMD,
    pub frame_index: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AntiLagPresentationInfoAMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ANTI_LAG_PRESENTATION_INFO_AMD,
            p_next: core::ptr::null_mut(),
            stage: Default::default(),
            frame_index: Default::default(),
            _marker: PhantomData,
        }
    }
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
    unsafe extern "system" fn(device: Device, p_data: *const AntiLagDataAMD<'_>);
