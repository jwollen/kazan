#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceQueueShaderCoreControlCreateInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceQueueShaderCoreControlCreateInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM,
            p_next: core::ptr::null_mut(),
            shader_core_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DeviceQueueShaderCoreControlCreateInfoARM<'a> {
    pub fn shader_core_count(mut self, shader_core_count: u32) -> Self {
        self.shader_core_count = shader_core_count;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSchedulingControlsFeaturesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub scheduling_controls: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceSchedulingControlsFeaturesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM,
            p_next: core::ptr::null_mut(),
            scheduling_controls: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceSchedulingControlsFeaturesARM<'a> {
    pub fn scheduling_controls(mut self, scheduling_controls: Bool32) -> Self {
        self.scheduling_controls = scheduling_controls;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSchedulingControlsPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub scheduling_controls_flags: PhysicalDeviceSchedulingControlsFlagsARM,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceSchedulingControlsPropertiesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM,
            p_next: core::ptr::null_mut(),
            scheduling_controls_flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceSchedulingControlsPropertiesARM<'a> {
    pub fn scheduling_controls_flags(
        mut self,
        scheduling_controls_flags: PhysicalDeviceSchedulingControlsFlagsARM,
    ) -> Self {
        self.scheduling_controls_flags = scheduling_controls_flags;
        self
    }
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
