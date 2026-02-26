#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SwapchainKHR(u64);
#[repr(C)]
pub struct SwapchainCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SwapchainCreateFlagsKHR,
    pub surface: SurfaceKHR,
    pub min_image_count: u32,
    pub image_format: Format,
    pub image_color_space: ColorSpaceKHR,
    pub image_extent: Extent2D,
    pub image_array_layers: u32,
    pub image_usage: ImageUsageFlags,
    pub image_sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub pre_transform: SurfaceTransformFlagBitsKHR,
    pub composite_alpha: CompositeAlphaFlagBitsKHR,
    pub present_mode: PresentModeKHR,
    pub clipped: Bool32,
    pub old_swapchain: SwapchainKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PresentInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub swapchain_count: u32,
    pub p_swapchains: *const SwapchainKHR,
    pub p_image_indices: *const u32,
    pub p_results: *mut Result,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DeviceGroupPresentCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_mask: [u32; MAX_DEVICE_GROUP_SIZE as usize],
    pub modes: DeviceGroupPresentModeFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImageSwapchainCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BindImageMemorySwapchainInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub image_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct AcquireNextImageInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub timeout: u64,
    pub semaphore: Semaphore,
    pub fence: Fence,
    pub device_mask: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DeviceGroupPresentInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_device_masks: *const u32,
    pub mode: DeviceGroupPresentModeFlagBitsKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DeviceGroupSwapchainCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub modes: DeviceGroupPresentModeFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SwapchainCreateFlagsKHR: Flags {
        const SPLIT_INSTANCE_BIND_REGIONS_KHR = SwapchainCreateFlagBitsKHR::SPLIT_INSTANCE_BIND_REGIONS_KHR.0;
        const PROTECTED_KHR = SwapchainCreateFlagBitsKHR::PROTECTED_KHR.0;
        const MUTABLE_FORMAT_KHR = SwapchainCreateFlagBitsKHR::MUTABLE_FORMAT_KHR.0;
        const DEFERRED_MEMORY_ALLOCATION_KHR = SwapchainCreateFlagBitsKHR::DEFERRED_MEMORY_ALLOCATION_KHR.0;
        const PRESENT_ID_2_KHR = SwapchainCreateFlagBitsKHR::PRESENT_ID_2_KHR.0;
        const PRESENT_WAIT_2_KHR = SwapchainCreateFlagBitsKHR::PRESENT_WAIT_2_KHR.0;
        const PRESENT_TIMING_EXT = SwapchainCreateFlagBitsKHR::PRESENT_TIMING_EXT.0;
        const DEFERRED_MEMORY_ALLOCATION_EXT = Self::DEFERRED_MEMORY_ALLOCATION_KHR.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwapchainCreateFlagBitsKHR(u32);
impl SwapchainCreateFlagBitsKHR {
    pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self(1 << 0);
    pub const PROTECTED_KHR: Self = Self(1 << 1);
    pub const MUTABLE_FORMAT_KHR: Self = Self(1 << 2);
    pub const DEFERRED_MEMORY_ALLOCATION_KHR: Self = Self(1 << 3);
    pub const PRESENT_ID_2_KHR: Self = Self(1 << 6);
    pub const PRESENT_WAIT_2_KHR: Self = Self(1 << 7);
    pub const PRESENT_TIMING_EXT: Self = Self(1 << 9);
    pub const DEFERRED_MEMORY_ALLOCATION_EXT: Self = Self::DEFERRED_MEMORY_ALLOCATION_KHR;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceGroupPresentModeFlagsKHR: Flags {
        const LOCAL_KHR = DeviceGroupPresentModeFlagBitsKHR::LOCAL_KHR.0;
        const REMOTE_KHR = DeviceGroupPresentModeFlagBitsKHR::REMOTE_KHR.0;
        const SUM_KHR = DeviceGroupPresentModeFlagBitsKHR::SUM_KHR.0;
        const LOCAL_MULTI_DEVICE_KHR = DeviceGroupPresentModeFlagBitsKHR::LOCAL_MULTI_DEVICE_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceGroupPresentModeFlagBitsKHR(u32);
impl DeviceGroupPresentModeFlagBitsKHR {
    pub const LOCAL_KHR: Self = Self(1 << 0);
    pub const REMOTE_KHR: Self = Self(1 << 1);
    pub const SUM_KHR: Self = Self(1 << 2);
    pub const LOCAL_MULTI_DEVICE_KHR: Self = Self(1 << 3);
}
pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SwapchainCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_swapchain: *mut SwapchainKHR,
) -> Result;
pub type PFN_vkDestroySwapchainKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut Image,
) -> Result;
pub type PFN_vkAcquireNextImageKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    timeout: u64,
    semaphore: Semaphore,
    fence: Fence,
    p_image_index: *mut u32,
) -> Result;
pub type PFN_vkQueuePresentKHR =
    unsafe extern "system" fn(queue: Queue, p_present_info: *const PresentInfoKHR<'_>) -> Result;
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "system" fn(
    device: Device,
    p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR<'_>,
) -> Result;
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "system" fn(
    device: Device,
    surface: SurfaceKHR,
    p_modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> Result;
pub type PFN_vkAcquireNextImage2KHR = unsafe extern "system" fn(
    device: Device,
    p_acquire_info: *const AcquireNextImageInfoKHR<'_>,
    p_image_index: *mut u32,
) -> Result;
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_rect_count: *mut u32,
    p_rects: *mut Rect2D,
) -> Result;
