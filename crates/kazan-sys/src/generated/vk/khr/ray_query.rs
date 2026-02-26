#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayQueryFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_query: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRayQueryFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            ray_query: Default::default(),
            _marker: PhantomData,
        }
    }
}
