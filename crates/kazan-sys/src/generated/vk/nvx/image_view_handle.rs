#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewHandleInfoNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub descriptor_type: DescriptorType,
    pub sampler: Sampler,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageViewHandleInfoNVX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_VIEW_HANDLE_INFO_NVX,
            p_next: core::ptr::null(),
            image_view: Default::default(),
            descriptor_type: Default::default(),
            sampler: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewAddressPropertiesNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_address: DeviceAddress,
    pub size: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageViewAddressPropertiesNVX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX,
            p_next: core::ptr::null_mut(),
            device_address: Default::default(),
            size: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetImageViewHandleNVX =
    unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX<'_>) -> u32;
pub type PFN_vkGetImageViewHandle64NVX =
    unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX<'_>) -> u64;
pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
    device: Device,
    image_view: ImageView,
    p_properties: *mut ImageViewAddressPropertiesNVX<'_>,
) -> Result;
pub type PFN_vkGetDeviceCombinedImageSamplerIndexNVX =
    unsafe extern "system" fn(device: Device, image_view_index: u64, sampler_index: u64) -> u64;
