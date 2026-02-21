#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_present_rectangles_khr: PFN_vkGetPhysicalDevicePresentRectanglesKHR,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        rects: impl ExtendUninit<Rect2D>,
    ) -> Result {
        unsafe {
            try_extend_uninit(rects, |rect_count, rects| {
                (self.get_physical_device_present_rectangles_khr)(
                    physical_device,
                    surface,
                    rect_count,
                    rects as _,
                )
            })
        }
    }
}
pub struct DeviceFn {
    get_device_group_peer_memory_features: PFN_vkGetDeviceGroupPeerMemoryFeatures,
    cmd_set_device_mask: PFN_vkCmdSetDeviceMask,
    cmd_dispatch_base: PFN_vkCmdDispatchBase,
    get_device_group_present_capabilities_khr: PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
    get_device_group_surface_present_modes_khr: PFN_vkGetDeviceGroupSurfacePresentModesKHR,
    acquire_next_image2_khr: PFN_vkAcquireNextImage2KHR,
}
impl DeviceFn {
    pub unsafe fn get_device_group_peer_memory_features_khr(
        &self,
        device: Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        peer_memory_features: &mut PeerMemoryFeatureFlags,
    ) {
        unsafe {
            (self.get_device_group_peer_memory_features)(
                device,
                heap_index,
                local_device_index,
                remote_device_index,
                peer_memory_features,
            )
        }
    }
    pub unsafe fn cmd_set_device_mask_khr(&self, command_buffer: CommandBuffer, device_mask: u32) {
        unsafe { (self.cmd_set_device_mask)(command_buffer, device_mask) }
    }
    pub unsafe fn cmd_dispatch_base_khr(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            (self.cmd_dispatch_base)(
                command_buffer,
                base_group_x,
                base_group_y,
                base_group_z,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
        device: Device,
        device_group_present_capabilities: &mut DeviceGroupPresentCapabilitiesKHR,
    ) -> Result {
        unsafe {
            (self.get_device_group_present_capabilities_khr)(
                device,
                device_group_present_capabilities,
            )
        }
    }
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        device: Device,
        surface: SurfaceKHR,
        modes: &mut DeviceGroupPresentModeFlagsKHR,
    ) -> Result {
        unsafe { (self.get_device_group_surface_present_modes_khr)(device, surface, modes) }
    }
    pub unsafe fn acquire_next_image2_khr(
        &self,
        device: Device,
        acquire_info: &AcquireNextImageInfoKHR,
        image_index: &mut u32,
    ) -> Result {
        unsafe { (self.acquire_next_image2_khr)(device, acquire_info, image_index) }
    }
}
