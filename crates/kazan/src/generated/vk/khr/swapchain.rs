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
    create_swapchain_khr: PFN_vkCreateSwapchainKHR,
    destroy_swapchain_khr: PFN_vkDestroySwapchainKHR,
    get_swapchain_images_khr: PFN_vkGetSwapchainImagesKHR,
    acquire_next_image_khr: PFN_vkAcquireNextImageKHR,
    queue_present_khr: PFN_vkQueuePresentKHR,
    get_device_group_present_capabilities_khr: PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
    get_device_group_surface_present_modes_khr: PFN_vkGetDeviceGroupSurfacePresentModesKHR,
    acquire_next_image2_khr: PFN_vkAcquireNextImage2KHR,
}
impl DeviceFn {
    pub unsafe fn create_swapchain_khr(
        &self,
        device: Device,
        create_info: &SwapchainCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
        swapchain: &mut SwapchainKHR,
    ) -> Result {
        unsafe {
            (self.create_swapchain_khr)(device, create_info, allocator.to_raw_ptr(), swapchain)
        }
    }
    pub unsafe fn destroy_swapchain_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_swapchain_khr)(device, swapchain, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_swapchain_images_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        swapchain_images: impl ExtendUninit<Image>,
    ) -> Result {
        unsafe {
            try_extend_uninit(
                swapchain_images,
                |swapchain_image_count, swapchain_images| {
                    (self.get_swapchain_images_khr)(
                        device,
                        swapchain,
                        swapchain_image_count,
                        swapchain_images as _,
                    )
                },
            )
        }
    }
    pub unsafe fn acquire_next_image_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
        image_index: &mut u32,
    ) -> Result {
        unsafe {
            (self.acquire_next_image_khr)(device, swapchain, timeout, semaphore, fence, image_index)
        }
    }
    pub unsafe fn queue_present_khr(&self, queue: Queue, present_info: &PresentInfoKHR) -> Result {
        unsafe { (self.queue_present_khr)(queue, present_info) }
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
