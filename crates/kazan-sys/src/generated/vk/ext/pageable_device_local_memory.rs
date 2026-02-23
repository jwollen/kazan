#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pageable_device_local_memory: Bool32,
}
pub type PFN_vkSetDeviceMemoryPriorityEXT =
    unsafe extern "system" fn(device: Device, memory: DeviceMemory, priority: f32);
