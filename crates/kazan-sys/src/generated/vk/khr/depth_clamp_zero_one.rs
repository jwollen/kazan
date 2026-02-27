#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDepthClampZeroOneFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_clamp_zero_one: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDepthClampZeroOneFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            depth_clamp_zero_one: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceDepthClampZeroOneFeaturesKHR<'a> {
    pub fn depth_clamp_zero_one(mut self, depth_clamp_zero_one: Bool32) -> Self {
        self.depth_clamp_zero_one = depth_clamp_zero_one;
        self
    }
}
