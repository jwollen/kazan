#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct DeviceQueueShaderCoreControlCreateInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceSchedulingControlsFeaturesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub scheduling_controls: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceSchedulingControlsPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub scheduling_controls_flags: PhysicalDeviceSchedulingControlsFlagsARM,
    pub _marker: PhantomData<&'a ()>,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PhysicalDeviceSchedulingControlsFlagsARM: Flags64 {
        const SHADER_CORE_COUNT_ARM = PhysicalDeviceSchedulingControlsFlagBitsARM::SHADER_CORE_COUNT_ARM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDeviceSchedulingControlsFlagBitsARM(u64);
impl PhysicalDeviceSchedulingControlsFlagBitsARM {
    pub const SHADER_CORE_COUNT_ARM: Self = Self(1 << 0);
}
