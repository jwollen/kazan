#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    copy_memory_to_image: PFN_vkCopyMemoryToImage,
    copy_image_to_memory: PFN_vkCopyImageToMemory,
    copy_image_to_image: PFN_vkCopyImageToImage,
    transition_image_layout: PFN_vkTransitionImageLayout,
    get_image_subresource_layout2: PFN_vkGetImageSubresourceLayout2,
    get_device_image_subresource_layout: PFN_vkGetDeviceImageSubresourceLayout,
    map_memory2: PFN_vkMapMemory2,
    unmap_memory2: PFN_vkUnmapMemory2,
}
impl DeviceFn {
    pub unsafe fn copy_memory_to_image(
        &self,
        device: Device,
        copy_memory_to_image_info: &CopyMemoryToImageInfo,
    ) -> Result {
        unsafe { (self.copy_memory_to_image)(device, copy_memory_to_image_info) }
    }
    pub unsafe fn copy_image_to_memory(
        &self,
        device: Device,
        copy_image_to_memory_info: &CopyImageToMemoryInfo,
    ) -> Result {
        unsafe { (self.copy_image_to_memory)(device, copy_image_to_memory_info) }
    }
    pub unsafe fn copy_image_to_image(
        &self,
        device: Device,
        copy_image_to_image_info: &CopyImageToImageInfo,
    ) -> Result {
        unsafe { (self.copy_image_to_image)(device, copy_image_to_image_info) }
    }
    pub unsafe fn transition_image_layout(
        &self,
        device: Device,
        transitions: &[HostImageLayoutTransitionInfo],
    ) -> Result {
        unsafe {
            (self.transition_image_layout)(
                device,
                transitions.len().try_into().unwrap(),
                transitions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn get_image_subresource_layout2(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2,
        layout: &mut SubresourceLayout2,
    ) {
        unsafe { (self.get_image_subresource_layout2)(device, image, subresource, layout) }
    }
    pub unsafe fn get_device_image_subresource_layout(
        &self,
        device: Device,
        info: &DeviceImageSubresourceInfo,
        layout: &mut SubresourceLayout2,
    ) {
        unsafe { (self.get_device_image_subresource_layout)(device, info, layout) }
    }
    pub unsafe fn map_memory2(
        &self,
        device: Device,
        memory_map_info: &MemoryMapInfo,
        data: &mut *mut c_void,
    ) -> Result {
        unsafe { (self.map_memory2)(device, memory_map_info, data) }
    }
    pub unsafe fn unmap_memory2(
        &self,
        device: Device,
        memory_unmap_info: &MemoryUnmapInfo,
    ) -> Result {
        unsafe { (self.unmap_memory2)(device, memory_unmap_info) }
    }
}
