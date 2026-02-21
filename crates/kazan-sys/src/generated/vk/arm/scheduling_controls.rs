#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceQueueShaderCoreControlCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSchedulingControlsFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub scheduling_controls: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSchedulingControlsPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub scheduling_controls_flags: PhysicalDeviceSchedulingControlsFlagsARM,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PhysicalDeviceSchedulingControlsFlagsARM: Flags64 {
        const SHADER_CORE_COUNT_ARM = PhysicalDeviceSchedulingControlsFlagBitsARM::SHADER_CORE_COUNT_ARM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDeviceSchedulingControlsFlagBitsARM(u64);
impl PhysicalDeviceSchedulingControlsFlagBitsARM {
    pub const SHADER_CORE_COUNT_ARM: Self = Self(1 << 0);
}
