#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceInvocationMaskFeaturesHUAWEI<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub invocation_mask: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceInvocationMaskFeaturesHUAWEI<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI,
            p_next: core::ptr::null_mut(),
            invocation_mask: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdBindInvocationMaskHUAWEI = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image_view: ImageView,
    image_layout: ImageLayout,
);
