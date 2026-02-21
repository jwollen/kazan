#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    set_device_memory_priority_ext: PFN_vkSetDeviceMemoryPriorityEXT,
}
impl DeviceFn {
    pub unsafe fn set_device_memory_priority_ext(
        &self,
        device: Device,
        memory: DeviceMemory,
        priority: f32,
    ) {
        unsafe { (self.set_device_memory_priority_ext)(device, memory, priority) }
    }
}
