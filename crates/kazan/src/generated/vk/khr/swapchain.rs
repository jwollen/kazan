#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    get_physical_device_present_rectangles_khr: Option<PFN_vkGetPhysicalDevicePresentRectanglesKHR>,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_present_rectangles_khr: transmute(load(
                    c"vkGetPhysicalDevicePresentRectanglesKHR",
                )),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        rects: impl ExtendUninit<Rect2D>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(rects, |rect_count, rects| {
                let result = (self.get_physical_device_present_rectangles_khr.unwrap())(
                    physical_device,
                    surface,
                    rect_count,
                    rects as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
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
    get_device_group_present_capabilities_khr: Option<PFN_vkGetDeviceGroupPresentCapabilitiesKHR>,
    get_device_group_surface_present_modes_khr: Option<PFN_vkGetDeviceGroupSurfacePresentModesKHR>,
    acquire_next_image2_khr: Option<PFN_vkAcquireNextImage2KHR>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_swapchain_khr: transmute(load(c"vkCreateSwapchainKHR").ok_or(LoadingError)?),
                destroy_swapchain_khr: transmute(
                    load(c"vkDestroySwapchainKHR").ok_or(LoadingError)?,
                ),
                get_swapchain_images_khr: transmute(
                    load(c"vkGetSwapchainImagesKHR").ok_or(LoadingError)?,
                ),
                acquire_next_image_khr: transmute(
                    load(c"vkAcquireNextImageKHR").ok_or(LoadingError)?,
                ),
                queue_present_khr: transmute(load(c"vkQueuePresentKHR").ok_or(LoadingError)?),
                get_device_group_present_capabilities_khr: transmute(load(
                    c"vkGetDeviceGroupPresentCapabilitiesKHR",
                )),
                get_device_group_surface_present_modes_khr: transmute(load(
                    c"vkGetDeviceGroupSurfacePresentModesKHR",
                )),
                acquire_next_image2_khr: transmute(load(c"vkAcquireNextImage2KHR")),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_swapchain_khr(
        &self,
        device: Device,
        create_info: &SwapchainCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<SwapchainKHR> {
        unsafe {
            let mut swapchain = core::mem::MaybeUninit::uninit();
            let result = (self.create_swapchain_khr)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                swapchain.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(swapchain.assume_init()),
                err => Err(err),
            }
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
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                swapchain_images,
                |swapchain_image_count, swapchain_images| {
                    let result = (self.get_swapchain_images_khr)(
                        device,
                        swapchain,
                        swapchain_image_count,
                        swapchain_images as _,
                    );

                    match result {
                        VkResult::SUCCESS => Ok(()),
                        VkResult::INCOMPLETE => Ok(()),
                        err => Err(err),
                    }
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
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.acquire_next_image_khr)(
                device,
                swapchain,
                timeout,
                semaphore,
                fence,
                image_index,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::TIMEOUT => Ok(()),
                VkResult::NOT_READY => Ok(()),
                VkResult::SUBOPTIMAL_KHR => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn queue_present_khr(
        &self,
        queue: Queue,
        present_info: &PresentInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_present_khr)(queue, present_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::SUBOPTIMAL_KHR => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
        device: Device,
    ) -> crate::Result<DeviceGroupPresentCapabilitiesKHR> {
        unsafe {
            let mut device_group_present_capabilities = core::mem::MaybeUninit::uninit();
            let result = (self.get_device_group_present_capabilities_khr.unwrap())(
                device,
                device_group_present_capabilities.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(device_group_present_capabilities.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        device: Device,
        surface: SurfaceKHR,
    ) -> crate::Result<DeviceGroupPresentModeFlagsKHR> {
        unsafe {
            let mut modes = core::mem::MaybeUninit::uninit();
            let result = (self.get_device_group_surface_present_modes_khr.unwrap())(
                device,
                surface,
                modes.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(modes.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn acquire_next_image2_khr(
        &self,
        device: Device,
        acquire_info: &AcquireNextImageInfoKHR,
        image_index: &mut u32,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.acquire_next_image2_khr.unwrap())(device, acquire_info, image_index);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::TIMEOUT => Ok(()),
                VkResult::NOT_READY => Ok(()),
                VkResult::SUBOPTIMAL_KHR => Ok(()),
                err => Err(err),
            }
        }
    }
}
