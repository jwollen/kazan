#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewHandleInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub descriptor_type: DescriptorType,
    pub sampler: Sampler,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewAddressPropertiesNVX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_address: DeviceAddress,
    pub size: DeviceSize,
}
pub type PFN_vkGetImageViewHandleNVX =
    unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX) -> u32;
pub type PFN_vkGetImageViewHandle64NVX =
    unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX) -> u64;
pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
    device: Device,
    image_view: ImageView,
    p_properties: *mut ImageViewAddressPropertiesNVX,
) -> Result;
pub type PFN_vkGetDeviceCombinedImageSamplerIndexNVX =
    unsafe extern "system" fn(device: Device, image_view_index: u64, sampler_index: u64) -> u64;
