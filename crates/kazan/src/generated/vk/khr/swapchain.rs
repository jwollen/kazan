//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_swapchain.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_swapchain";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        SwapchainKHR,
        SWAPCHAIN_KHR,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainKHR.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for SwapchainCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwapchainCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("surface", &self.surface)
                .field("min_image_count", &self.min_image_count)
                .field("image_format", &self.image_format)
                .field("image_color_space", &self.image_color_space)
                .field("image_extent", &self.image_extent)
                .field("image_array_layers", &self.image_array_layers)
                .field("image_usage", &self.image_usage)
                .field("image_sharing_mode", &self.image_sharing_mode)
                .field("queue_family_index_count", &self.queue_family_index_count)
                .field("p_queue_family_indices", &self.p_queue_family_indices)
                .field("pre_transform", &self.pre_transform)
                .field("composite_alpha", &self.composite_alpha)
                .field("present_mode", &self.present_mode)
                .field("clipped", &self.clipped)
                .field("old_swapchain", &self.old_swapchain)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SwapchainCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SWAPCHAIN_CREATE_INFO_KHR;
    }

    impl Default for SwapchainCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
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
                p_queue_family_indices: ptr::null(),
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
        #[inline]
        pub fn flags(mut self, flags: SwapchainCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn surface(mut self, surface: SurfaceKHR) -> Self {
            self.surface = surface;
            self
        }

        #[inline]
        pub fn min_image_count(mut self, min_image_count: u32) -> Self {
            self.min_image_count = min_image_count;
            self
        }

        #[inline]
        pub fn image_format(mut self, image_format: Format) -> Self {
            self.image_format = image_format;
            self
        }

        #[inline]
        pub fn image_color_space(mut self, image_color_space: ColorSpaceKHR) -> Self {
            self.image_color_space = image_color_space;
            self
        }

        #[inline]
        pub fn image_extent(mut self, image_extent: Extent2D) -> Self {
            self.image_extent = image_extent;
            self
        }

        #[inline]
        pub fn image_array_layers(mut self, image_array_layers: u32) -> Self {
            self.image_array_layers = image_array_layers;
            self
        }

        #[inline]
        pub fn image_usage(mut self, image_usage: ImageUsageFlags) -> Self {
            self.image_usage = image_usage;
            self
        }

        #[inline]
        pub fn image_sharing_mode(mut self, image_sharing_mode: SharingMode) -> Self {
            self.image_sharing_mode = image_sharing_mode;
            self
        }

        #[inline]
        pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
            self.queue_family_index_count = queue_family_indices.len().try_into().unwrap();
            self.p_queue_family_indices = queue_family_indices.as_ptr() as _;
            self
        }

        #[inline]
        pub fn pre_transform(mut self, pre_transform: SurfaceTransformFlagBitsKHR) -> Self {
            self.pre_transform = pre_transform;
            self
        }

        #[inline]
        pub fn composite_alpha(mut self, composite_alpha: CompositeAlphaFlagBitsKHR) -> Self {
            self.composite_alpha = composite_alpha;
            self
        }

        #[inline]
        pub fn present_mode(mut self, present_mode: PresentModeKHR) -> Self {
            self.present_mode = present_mode;
            self
        }

        #[inline]
        pub fn clipped(mut self, clipped: bool) -> Self {
            self.clipped = clipped.into();
            self
        }

        #[inline]
        pub fn old_swapchain(mut self, old_swapchain: SwapchainKHR) -> Self {
            self.old_swapchain = old_swapchain;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PresentInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub wait_semaphore_count: u32,
        pub p_wait_semaphores: *const Semaphore,
        pub swapchain_count: u32,
        pub p_swapchains: *const SwapchainKHR,
        pub p_image_indices: *const u32,
        pub p_results: *mut vk::Result,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PresentInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PresentInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("wait_semaphore_count", &self.wait_semaphore_count)
                .field("p_wait_semaphores", &self.p_wait_semaphores)
                .field("swapchain_count", &self.swapchain_count)
                .field("p_swapchains", &self.p_swapchains)
                .field("p_image_indices", &self.p_image_indices)
                .field("p_results", &self.p_results)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PresentInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_INFO_KHR;
    }

    impl Default for PresentInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                wait_semaphore_count: Default::default(),
                p_wait_semaphores: ptr::null(),
                swapchain_count: Default::default(),
                p_swapchains: ptr::null(),
                p_image_indices: ptr::null(),
                p_results: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PresentInfoKHR<'a> {
        #[inline]
        pub fn wait_semaphores(mut self, wait_semaphores: &'a [Semaphore]) -> Self {
            self.wait_semaphore_count = wait_semaphores.len().try_into().unwrap();
            self.p_wait_semaphores = wait_semaphores.as_ptr() as _;
            self
        }

        #[inline]
        pub fn swapchains(
            mut self,
            swapchains: &'a [SwapchainKHR],
            image_indices: &'a [u32],
            results: Option<&'a mut [vk::Result]>,
        ) -> Self {
            self.swapchain_count = swapchains.len().try_into().unwrap();
            assert_eq!(image_indices.len(), self.swapchain_count as usize);
            if let Some(s) = &results {
                assert_eq!(s.len(), self.swapchain_count as usize);
            }
            self.p_swapchains = swapchains.as_ptr() as _;
            self.p_image_indices = image_indices.as_ptr() as _;
            self.p_results = results.map_or(ptr::null_mut(), |s| s.as_mut_ptr() as _);
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupPresentCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceGroupPresentCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_mask: [u32; MAX_DEVICE_GROUP_SIZE as usize],
        pub modes: DeviceGroupPresentModeFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceGroupPresentCapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceGroupPresentCapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_mask", &self.present_mask)
                .field("modes", &self.modes)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceGroupPresentCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR;
    }

    impl Default for DeviceGroupPresentCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                present_mask: [Default::default(); _],
                modes: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceGroupPresentCapabilitiesKHR<'a> {
        #[inline]
        pub fn present_mask(mut self, present_mask: [u32; MAX_DEVICE_GROUP_SIZE as usize]) -> Self {
            self.present_mask = present_mask;
            self
        }

        #[inline]
        pub fn modes(mut self, modes: DeviceGroupPresentModeFlagsKHR) -> Self {
            self.modes = modes;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageSwapchainCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageSwapchainCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain: SwapchainKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageSwapchainCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageSwapchainCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain", &self.swapchain)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageSwapchainCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHR;
    }

    unsafe impl Extends<ImageCreateInfo<'_>> for ImageSwapchainCreateInfoKHR<'_> {}

    impl Default for ImageSwapchainCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                swapchain: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageSwapchainCreateInfoKHR<'a> {
        #[inline]
        pub fn swapchain(mut self, swapchain: SwapchainKHR) -> Self {
            self.swapchain = swapchain;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindImageMemorySwapchainInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindImageMemorySwapchainInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain: SwapchainKHR,
        pub image_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindImageMemorySwapchainInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindImageMemorySwapchainInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain", &self.swapchain)
                .field("image_index", &self.image_index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindImageMemorySwapchainInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR;
    }

    unsafe impl Extends<BindImageMemoryInfo<'_>> for BindImageMemorySwapchainInfoKHR<'_> {}

    impl Default for BindImageMemorySwapchainInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                swapchain: Default::default(),
                image_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindImageMemorySwapchainInfoKHR<'a> {
        #[inline]
        pub fn swapchain(mut self, swapchain: SwapchainKHR) -> Self {
            self.swapchain = swapchain;
            self
        }

        #[inline]
        pub fn image_index(mut self, image_index: u32) -> Self {
            self.image_index = image_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAcquireNextImageInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for AcquireNextImageInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AcquireNextImageInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain", &self.swapchain)
                .field("timeout", &self.timeout)
                .field("semaphore", &self.semaphore)
                .field("fence", &self.fence)
                .field("device_mask", &self.device_mask)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AcquireNextImageInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ACQUIRE_NEXT_IMAGE_INFO_KHR;
    }

    impl Default for AcquireNextImageInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
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
        #[inline]
        pub fn swapchain(mut self, swapchain: SwapchainKHR) -> Self {
            self.swapchain = swapchain;
            self
        }

        #[inline]
        pub fn timeout(mut self, timeout: u64) -> Self {
            self.timeout = timeout;
            self
        }

        #[inline]
        pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
            self.semaphore = semaphore;
            self
        }

        #[inline]
        pub fn fence(mut self, fence: Fence) -> Self {
            self.fence = fence;
            self
        }

        #[inline]
        pub fn device_mask(mut self, device_mask: u32) -> Self {
            self.device_mask = device_mask;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupPresentInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceGroupPresentInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain_count: u32,
        pub p_device_masks: *const u32,
        pub mode: DeviceGroupPresentModeFlagBitsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceGroupPresentInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceGroupPresentInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain_count", &self.swapchain_count)
                .field("p_device_masks", &self.p_device_masks)
                .field("mode", &self.mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceGroupPresentInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_GROUP_PRESENT_INFO_KHR;
    }

    unsafe impl Extends<PresentInfoKHR<'_>> for DeviceGroupPresentInfoKHR<'_> {}

    impl Default for DeviceGroupPresentInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                swapchain_count: Default::default(),
                p_device_masks: ptr::null(),
                mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceGroupPresentInfoKHR<'a> {
        #[inline]
        pub fn device_masks(mut self, device_masks: &'a [u32]) -> Self {
            self.swapchain_count = device_masks.len().try_into().unwrap();
            self.p_device_masks = device_masks.as_ptr() as _;
            self
        }

        #[inline]
        pub fn mode(mut self, mode: DeviceGroupPresentModeFlagBitsKHR) -> Self {
            self.mode = mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceGroupSwapchainCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub modes: DeviceGroupPresentModeFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceGroupSwapchainCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceGroupSwapchainCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("modes", &self.modes)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceGroupSwapchainCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR;
    }

    unsafe impl Extends<SwapchainCreateInfoKHR<'_>> for DeviceGroupSwapchainCreateInfoKHR<'_> {}

    impl Default for DeviceGroupSwapchainCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                modes: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceGroupSwapchainCreateInfoKHR<'a> {
        #[inline]
        pub fn modes(mut self, modes: DeviceGroupPresentModeFlagsKHR) -> Self {
            self.modes = modes;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainCreateFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SwapchainCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(SwapchainCreateFlagsKHR, Flags, SwapchainCreateFlagBitsKHR);

    impl fmt::Debug for SwapchainCreateFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    SwapchainCreateFlagBitsKHR::PRESENT_TIMING_EXT.0,
                    "PRESENT_TIMING_EXT",
                ),
                (
                    SwapchainCreateFlagBitsKHR::SPLIT_INSTANCE_BIND_REGIONS_KHR.0,
                    "SPLIT_INSTANCE_BIND_REGIONS_KHR",
                ),
                (
                    SwapchainCreateFlagBitsKHR::PRESENT_ID_2_KHR.0,
                    "PRESENT_ID_2_KHR",
                ),
                (
                    SwapchainCreateFlagBitsKHR::PRESENT_WAIT_2_KHR.0,
                    "PRESENT_WAIT_2_KHR",
                ),
                (SwapchainCreateFlagBitsKHR::PROTECTED_KHR.0, "PROTECTED_KHR"),
                (
                    SwapchainCreateFlagBitsKHR::DEFERRED_MEMORY_ALLOCATION_KHR.0,
                    "DEFERRED_MEMORY_ALLOCATION_KHR",
                ),
                (
                    SwapchainCreateFlagBitsKHR::MUTABLE_FORMAT_KHR.0,
                    "MUTABLE_FORMAT_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainCreateFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SwapchainCreateFlagBitsKHR(u32);

    impl SwapchainCreateFlagBitsKHR {
        // VK_EXT_present_timing
        pub const PRESENT_TIMING_EXT: Self = Self(1 << 9);

        // VK_EXT_swapchain_maintenance1
        pub const DEFERRED_MEMORY_ALLOCATION_EXT: Self = Self::DEFERRED_MEMORY_ALLOCATION_KHR;

        // VK_KHR_device_group
        /// Allow images with VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT
        pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self(1 << 0);

        // VK_KHR_present_id2
        /// Allow use of VK_KHR_present_id2 with this swapchain
        pub const PRESENT_ID_2_KHR: Self = Self(1 << 6);

        // VK_KHR_present_wait2
        /// Allow use of VK_KHR_present_wait2 with this swapchain
        pub const PRESENT_WAIT_2_KHR: Self = Self(1 << 7);

        // VK_KHR_swapchain
        /// Swapchain is protected
        pub const PROTECTED_KHR: Self = Self(1 << 1);

        // VK_KHR_swapchain_maintenance1
        pub const DEFERRED_MEMORY_ALLOCATION_KHR: Self = Self(1 << 3);

        // VK_KHR_swapchain_mutable_format
        pub const MUTABLE_FORMAT_KHR: Self = Self(1 << 2);
    }

    impl fmt::Debug for SwapchainCreateFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PRESENT_TIMING_EXT => Some("PRESENT_TIMING_EXT"),
                Self::SPLIT_INSTANCE_BIND_REGIONS_KHR => Some("SPLIT_INSTANCE_BIND_REGIONS_KHR"),
                Self::PRESENT_ID_2_KHR => Some("PRESENT_ID_2_KHR"),
                Self::PRESENT_WAIT_2_KHR => Some("PRESENT_WAIT_2_KHR"),
                Self::PROTECTED_KHR => Some("PROTECTED_KHR"),
                Self::DEFERRED_MEMORY_ALLOCATION_KHR => Some("DEFERRED_MEMORY_ALLOCATION_KHR"),
                Self::MUTABLE_FORMAT_KHR => Some("MUTABLE_FORMAT_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupPresentModeFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DeviceGroupPresentModeFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        DeviceGroupPresentModeFlagsKHR,
        Flags,
        DeviceGroupPresentModeFlagBitsKHR
    );

    impl fmt::Debug for DeviceGroupPresentModeFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (DeviceGroupPresentModeFlagBitsKHR::LOCAL_KHR.0, "LOCAL_KHR"),
                (
                    DeviceGroupPresentModeFlagBitsKHR::REMOTE_KHR.0,
                    "REMOTE_KHR",
                ),
                (DeviceGroupPresentModeFlagBitsKHR::SUM_KHR.0, "SUM_KHR"),
                (
                    DeviceGroupPresentModeFlagBitsKHR::LOCAL_MULTI_DEVICE_KHR.0,
                    "LOCAL_MULTI_DEVICE_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DeviceGroupPresentModeFlagBitsKHR(u32);

    impl DeviceGroupPresentModeFlagBitsKHR {
        /// Present from local memory
        pub const LOCAL_KHR: Self = Self(1 << 0);
        /// Present from remote memory
        pub const REMOTE_KHR: Self = Self(1 << 1);
        /// Present sum of local and/or remote memory
        pub const SUM_KHR: Self = Self(1 << 2);
        /// Each physical device presents from local memory
        pub const LOCAL_MULTI_DEVICE_KHR: Self = Self(1 << 3);
    }

    impl fmt::Debug for DeviceGroupPresentModeFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::LOCAL_KHR => Some("LOCAL_KHR"),
                Self::REMOTE_KHR => Some("REMOTE_KHR"),
                Self::SUM_KHR => Some("SUM_KHR"),
                Self::LOCAL_MULTI_DEVICE_KHR => Some("LOCAL_MULTI_DEVICE_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateSwapchainKHR.html>
    pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const SwapchainCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_swapchain: *mut SwapchainKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroySwapchainKHR.html>
    pub type PFN_vkDestroySwapchainKHR = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSwapchainImagesKHR.html>
    pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut Image,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireNextImageKHR.html>
    pub type PFN_vkAcquireNextImageKHR = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
        p_image_index: *mut u32,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueuePresentKHR.html>
    pub type PFN_vkQueuePresentKHR = unsafe extern "system" fn(
        queue: Queue,
        p_present_info: *const PresentInfoKHR<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html>
    pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "system" fn(
        device: Device,
        p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR<'_>,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html>
    pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "system" fn(
        device: Device,
        surface: SurfaceKHR,
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireNextImage2KHR.html>
    pub type PFN_vkAcquireNextImage2KHR = unsafe extern "system" fn(
        device: Device,
        p_acquire_info: *const AcquireNextImageInfoKHR<'_>,
        p_image_index: *mut u32,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html>
    pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_rect_count: *mut u32,
        p_rects: *mut Rect2D,
    )
        -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkSwapchainKHR = SwapchainKHR;
    pub type VkSwapchainCreateInfoKHR = SwapchainCreateInfoKHR<'static>;
    pub type VkPresentInfoKHR = PresentInfoKHR<'static>;
    pub type VkDeviceGroupPresentCapabilitiesKHR = DeviceGroupPresentCapabilitiesKHR<'static>;
    pub type VkImageSwapchainCreateInfoKHR = ImageSwapchainCreateInfoKHR<'static>;
    pub type VkBindImageMemorySwapchainInfoKHR = BindImageMemorySwapchainInfoKHR<'static>;
    pub type VkAcquireNextImageInfoKHR = AcquireNextImageInfoKHR<'static>;
    pub type VkDeviceGroupPresentInfoKHR = DeviceGroupPresentInfoKHR<'static>;
    pub type VkDeviceGroupSwapchainCreateInfoKHR = DeviceGroupSwapchainCreateInfoKHR<'static>;
    pub type VkSwapchainCreateFlagsKHR = SwapchainCreateFlagsKHR;
    pub type VkSwapchainCreateFlagBitsKHR = SwapchainCreateFlagBitsKHR;
    pub type VkDeviceGroupPresentModeFlagsKHR = DeviceGroupPresentModeFlagsKHR;
    pub type VkDeviceGroupPresentModeFlagBitsKHR = DeviceGroupPresentModeFlagBitsKHR;
    impl SwapchainCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSwapchainCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PresentInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPresentInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceGroupPresentCapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceGroupPresentCapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageSwapchainCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageSwapchainCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BindImageMemorySwapchainInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBindImageMemorySwapchainInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AcquireNextImageInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAcquireNextImageInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceGroupPresentInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceGroupPresentInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceGroupSwapchainCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceGroupSwapchainCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_present_rectangles: Option<PFN_vkGetPhysicalDevicePresentRectanglesKHR>,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_present_rectangles: transmute(load(
                    c"vkGetPhysicalDevicePresentRectanglesKHR",
                )),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_present_rectangles(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        mut rects: impl ExtendUninit<Rect2D>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |rect_count, rects| {
                let result = (self.get_physical_device_present_rectangles.unwrap())(
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
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let rects_buf = rects.reserve(capacity);
            len = rects_buf.len().try_into().unwrap();
            let result = call(&mut len, rects_buf.as_mut_ptr() as *mut _)?;
            rects.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

pub struct DeviceFn {
    create_swapchain: PFN_vkCreateSwapchainKHR,
    destroy_swapchain: PFN_vkDestroySwapchainKHR,
    get_swapchain_images: PFN_vkGetSwapchainImagesKHR,
    acquire_next_image: PFN_vkAcquireNextImageKHR,
    queue_present: PFN_vkQueuePresentKHR,
    get_device_group_present_capabilities: Option<PFN_vkGetDeviceGroupPresentCapabilitiesKHR>,
    get_device_group_surface_present_modes: Option<PFN_vkGetDeviceGroupSurfacePresentModesKHR>,
    acquire_next_image2: Option<PFN_vkAcquireNextImage2KHR>,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_swapchain: transmute(
                    load(c"vkCreateSwapchainKHR").ok_or(MissingEntryPointError)?,
                ),
                destroy_swapchain: transmute(
                    load(c"vkDestroySwapchainKHR").ok_or(MissingEntryPointError)?,
                ),
                get_swapchain_images: transmute(
                    load(c"vkGetSwapchainImagesKHR").ok_or(MissingEntryPointError)?,
                ),
                acquire_next_image: transmute(
                    load(c"vkAcquireNextImageKHR").ok_or(MissingEntryPointError)?,
                ),
                queue_present: transmute(load(c"vkQueuePresentKHR").ok_or(MissingEntryPointError)?),
                get_device_group_present_capabilities: transmute(load(
                    c"vkGetDeviceGroupPresentCapabilitiesKHR",
                )),
                get_device_group_surface_present_modes: transmute(load(
                    c"vkGetDeviceGroupSurfacePresentModesKHR",
                )),
                acquire_next_image2: transmute(load(c"vkAcquireNextImage2KHR")),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateSwapchainKHR.html>
    #[inline]
    pub unsafe fn create_swapchain(
        &self,
        device: Device,
        create_info: &SwapchainCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SwapchainKHR> {
        unsafe {
            let mut swapchain = core::mem::MaybeUninit::uninit();
            let result = (self.create_swapchain)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroySwapchainKHR.html>
    #[inline]
    pub unsafe fn destroy_swapchain(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_swapchain)(device, swapchain, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSwapchainImagesKHR.html>
    #[inline]
    pub unsafe fn get_swapchain_images(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        mut swapchain_images: impl ExtendUninit<Image>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |swapchain_image_count, swapchain_images| {
                let result = (self.get_swapchain_images)(
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
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let swapchain_images_buf = swapchain_images.reserve(capacity);
            len = swapchain_images_buf.len().try_into().unwrap();
            let result = call(&mut len, swapchain_images_buf.as_mut_ptr() as *mut _)?;
            swapchain_images.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireNextImageKHR.html>
    #[inline]
    pub unsafe fn acquire_next_image(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
    ) -> crate::Result<(u32, bool)> {
        unsafe {
            let mut image_index = core::mem::MaybeUninit::uninit();
            let result = (self.acquire_next_image)(
                device,
                swapchain,
                timeout,
                semaphore,
                fence,
                image_index.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok((image_index.assume_init(), false)),
                VkResult::SUBOPTIMAL_KHR => Ok((image_index.assume_init(), true)),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueuePresentKHR.html>
    #[inline]
    pub unsafe fn queue_present(
        &self,
        queue: Queue,
        present_info: &PresentInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_present)(queue, present_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html>
    #[inline]
    pub unsafe fn get_device_group_present_capabilities(
        &self,
        device: Device,
        device_group_present_capabilities: &mut DeviceGroupPresentCapabilitiesKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_device_group_present_capabilities.unwrap())(
                device,
                device_group_present_capabilities,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html>
    #[inline]
    pub unsafe fn get_device_group_surface_present_modes(
        &self,
        device: Device,
        surface: SurfaceKHR,
    ) -> crate::Result<DeviceGroupPresentModeFlagsKHR> {
        unsafe {
            let mut modes = core::mem::MaybeUninit::uninit();
            let result = (self.get_device_group_surface_present_modes.unwrap())(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireNextImage2KHR.html>
    #[inline]
    pub unsafe fn acquire_next_image2(
        &self,
        device: Device,
        acquire_info: &AcquireNextImageInfoKHR<'_>,
    ) -> crate::Result<(u32, bool)> {
        unsafe {
            let mut image_index = core::mem::MaybeUninit::uninit();
            let result =
                (self.acquire_next_image2.unwrap())(device, acquire_info, image_index.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok((image_index.assume_init(), false)),
                VkResult::SUBOPTIMAL_KHR => Ok((image_index.assume_init(), true)),
                err => Err(err),
            }
        }
    }
}
