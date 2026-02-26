#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dedicated_allocation_image_aliasing: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            dedicated_allocation_image_aliasing: Default::default(),
            _marker: PhantomData,
        }
    }
}
