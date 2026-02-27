#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SwapchainKHR(u64);
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for SwapchainCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            surface: Default::default(),
            min_image_count: Default::default(),
            image_format: Default::default(),
            image_color_space: Default::default(),
            image_extent: Default::default(),
            image_array_layers: Default::default(),
            image_usage: Default::default(),
            image_sharing_mode: Default::default(),
            queue_family_index_count: Default::default(),
            p_queue_family_indices: core::ptr::null(),
            pre_transform: Default::default(),
            composite_alpha: Default::default(),
            present_mode: Default::default(),
            clipped: Default::default(),
            old_swapchain: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SwapchainCreateInfoKHR<'a> {
    pub fn flags(mut self, flags: SwapchainCreateFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    pub fn surface(mut self, surface: SurfaceKHR) -> Self {
        self.surface = surface;
        self
    }
    pub fn min_image_count(mut self, min_image_count: u32) -> Self {
        self.min_image_count = min_image_count;
        self
    }
    pub fn image_format(mut self, image_format: Format) -> Self {
        self.image_format = image_format;
        self
    }
    pub fn image_color_space(mut self, image_color_space: ColorSpaceKHR) -> Self {
        self.image_color_space = image_color_space;
        self
    }
    pub fn image_extent(mut self, image_extent: Extent2D) -> Self {
        self.image_extent = image_extent;
        self
    }
    pub fn image_array_layers(mut self, image_array_layers: u32) -> Self {
        self.image_array_layers = image_array_layers;
        self
    }
    pub fn image_usage(mut self, image_usage: ImageUsageFlags) -> Self {
        self.image_usage = image_usage;
        self
    }
    pub fn image_sharing_mode(mut self, image_sharing_mode: SharingMode) -> Self {
        self.image_sharing_mode = image_sharing_mode;
        self
    }
    pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
        self.queue_family_index_count = queue_family_indices.len().try_into().unwrap();
        self.p_queue_family_indices = queue_family_indices.as_ptr();
        self
    }
    pub fn pre_transform(mut self, pre_transform: SurfaceTransformFlagBitsKHR) -> Self {
        self.pre_transform = pre_transform;
        self
    }
    pub fn composite_alpha(mut self, composite_alpha: CompositeAlphaFlagBitsKHR) -> Self {
        self.composite_alpha = composite_alpha;
        self
    }
    pub fn present_mode(mut self, present_mode: PresentModeKHR) -> Self {
        self.present_mode = present_mode;
        self
    }
    pub fn clipped(mut self, clipped: Bool32) -> Self {
        self.clipped = clipped;
        self
    }
    pub fn old_swapchain(mut self, old_swapchain: SwapchainKHR) -> Self {
        self.old_swapchain = old_swapchain;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for PresentInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PRESENT_INFO_KHR,
            p_next: core::ptr::null(),
            wait_semaphore_count: Default::default(),
            p_wait_semaphores: core::ptr::null(),
            swapchain_count: Default::default(),
            p_swapchains: core::ptr::null(),
            p_image_indices: core::ptr::null(),
            p_results: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PresentInfoKHR<'a> {
    pub fn wait_semaphores(mut self, wait_semaphores: &'a [Semaphore]) -> Self {
        self.wait_semaphore_count = wait_semaphores.len().try_into().unwrap();
        self.p_wait_semaphores = wait_semaphores.as_ptr();
        self
    }
    pub fn swapchains(mut self, swapchains: &'a [SwapchainKHR]) -> Self {
        self.swapchain_count = swapchains.len().try_into().unwrap();
        self.p_swapchains = swapchains.as_ptr();
        self
    }
    pub fn image_indices(mut self, image_indices: &'a [u32]) -> Self {
        self.swapchain_count = image_indices.len().try_into().unwrap();
        self.p_image_indices = image_indices.as_ptr();
        self
    }
    pub fn results(mut self, results: &'a mut [Result]) -> Self {
        self.swapchain_count = results.len().try_into().unwrap();
        self.p_results = results.as_mut_ptr();
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupPresentCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_mask: [u32; MAX_DEVICE_GROUP_SIZE as usize],
    pub modes: DeviceGroupPresentModeFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceGroupPresentCapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            present_mask: [Default::default(); _],
            modes: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DeviceGroupPresentCapabilitiesKHR<'a> {
    pub fn present_mask(mut self, present_mask: [u32; MAX_DEVICE_GROUP_SIZE as usize]) -> Self {
        self.present_mask = present_mask;
        self
    }
    pub fn modes(mut self, modes: DeviceGroupPresentModeFlagsKHR) -> Self {
        self.modes = modes;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSwapchainCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageSwapchainCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            swapchain: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ImageSwapchainCreateInfoKHR<'a> {
    pub fn swapchain(mut self, swapchain: SwapchainKHR) -> Self {
        self.swapchain = swapchain;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImageMemorySwapchainInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub image_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BindImageMemorySwapchainInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR,
            p_next: core::ptr::null(),
            swapchain: Default::default(),
            image_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> BindImageMemorySwapchainInfoKHR<'a> {
    pub fn swapchain(mut self, swapchain: SwapchainKHR) -> Self {
        self.swapchain = swapchain;
        self
    }
    pub fn image_index(mut self, image_index: u32) -> Self {
        self.image_index = image_index;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for AcquireNextImageInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACQUIRE_NEXT_IMAGE_INFO_KHR,
            p_next: core::ptr::null(),
            swapchain: Default::default(),
            timeout: Default::default(),
            semaphore: Default::default(),
            fence: Default::default(),
            device_mask: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> AcquireNextImageInfoKHR<'a> {
    pub fn swapchain(mut self, swapchain: SwapchainKHR) -> Self {
        self.swapchain = swapchain;
        self
    }
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }
    pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
        self.semaphore = semaphore;
        self
    }
    pub fn fence(mut self, fence: Fence) -> Self {
        self.fence = fence;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.device_mask = device_mask;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupPresentInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_device_masks: *const u32,
    pub mode: DeviceGroupPresentModeFlagBitsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceGroupPresentInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_GROUP_PRESENT_INFO_KHR,
            p_next: core::ptr::null(),
            swapchain_count: Default::default(),
            p_device_masks: core::ptr::null(),
            mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DeviceGroupPresentInfoKHR<'a> {
    pub fn device_masks(mut self, device_masks: &'a [u32]) -> Self {
        self.swapchain_count = device_masks.len().try_into().unwrap();
        self.p_device_masks = device_masks.as_ptr();
        self
    }
    pub fn mode(mut self, mode: DeviceGroupPresentModeFlagBitsKHR) -> Self {
        self.mode = mode;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupSwapchainCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub modes: DeviceGroupPresentModeFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceGroupSwapchainCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            modes: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DeviceGroupSwapchainCreateInfoKHR<'a> {
    pub fn modes(mut self, modes: DeviceGroupPresentModeFlagsKHR) -> Self {
        self.modes = modes;
        self
    }
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
