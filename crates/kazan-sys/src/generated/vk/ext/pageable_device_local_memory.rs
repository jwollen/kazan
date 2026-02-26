#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pageable_device_local_memory: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            pageable_device_local_memory: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkSetDeviceMemoryPriorityEXT =
    unsafe extern "system" fn(device: Device, memory: DeviceMemory, priority: f32);
