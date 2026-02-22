#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_features2_khr: PFN_vkGetPhysicalDeviceFeatures2,
    get_physical_device_properties2_khr: PFN_vkGetPhysicalDeviceProperties2,
    get_physical_device_format_properties2_khr: PFN_vkGetPhysicalDeviceFormatProperties2,
    get_physical_device_image_format_properties2_khr: PFN_vkGetPhysicalDeviceImageFormatProperties2,
    get_physical_device_queue_family_properties2_khr: PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    get_physical_device_memory_properties2_khr: PFN_vkGetPhysicalDeviceMemoryProperties2,
    get_physical_device_sparse_image_format_properties2_khr:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_features2_khr: transmute(
                    load(c"vkGetPhysicalDeviceFeatures2KHR").ok_or(LoadingError)?,
                ),
                get_physical_device_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceProperties2KHR").ok_or(LoadingError)?,
                ),
                get_physical_device_format_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceFormatProperties2KHR").ok_or(LoadingError)?,
                ),
                get_physical_device_image_format_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceImageFormatProperties2KHR").ok_or(LoadingError)?,
                ),
                get_physical_device_queue_family_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyProperties2KHR").ok_or(LoadingError)?,
                ),
                get_physical_device_memory_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceMemoryProperties2KHR").ok_or(LoadingError)?,
                ),
                get_physical_device_sparse_image_format_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_features2_khr(
        &self,
        physical_device: PhysicalDevice,
        features: &mut PhysicalDeviceFeatures2,
    ) {
        unsafe { (self.get_physical_device_features2_khr)(physical_device, features) }
    }
    pub unsafe fn get_physical_device_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        properties: &mut PhysicalDeviceProperties2,
    ) {
        unsafe { (self.get_physical_device_properties2_khr)(physical_device, properties) }
    }
    pub unsafe fn get_physical_device_format_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        format_properties: &mut FormatProperties2,
    ) {
        unsafe {
            (self.get_physical_device_format_properties2_khr)(
                physical_device,
                format,
                format_properties,
            )
        }
    }
    pub unsafe fn get_physical_device_image_format_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        image_format_info: &PhysicalDeviceImageFormatInfo2,
        image_format_properties: &mut ImageFormatProperties2,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_physical_device_image_format_properties2_khr)(
                physical_device,
                image_format_info,
                image_format_properties,
            ))
        }
    }
    pub unsafe fn get_physical_device_queue_family_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_properties: impl ExtendUninit<QueueFamilyProperties2>,
    ) {
        unsafe {
            extend_uninit(
                queue_family_properties,
                |queue_family_property_count, queue_family_properties| {
                    (self.get_physical_device_queue_family_properties2_khr)(
                        physical_device,
                        queue_family_property_count,
                        queue_family_properties as _,
                    )
                },
            )
        }
    }
    pub unsafe fn get_physical_device_memory_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        memory_properties: &mut PhysicalDeviceMemoryProperties2,
    ) {
        unsafe {
            (self.get_physical_device_memory_properties2_khr)(physical_device, memory_properties)
        }
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2,
        properties: impl ExtendUninit<SparseImageFormatProperties2>,
    ) {
        unsafe {
            extend_uninit(properties, |property_count, properties| {
                (self.get_physical_device_sparse_image_format_properties2_khr)(
                    physical_device,
                    format_info,
                    property_count,
                    properties as _,
                )
            })
        }
    }
}
