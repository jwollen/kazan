//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_swapchain_maintenance1.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_swapchain_maintenance1";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub swapchain_maintenance1: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSwapchainMaintenance1FeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain_maintenance1", &self.swapchain_maintenance1)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'_> {}

    impl Default for PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                swapchain_maintenance1: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'a> {
        #[inline]
        pub fn swapchain_maintenance1(mut self, swapchain_maintenance1: bool) -> Self {
            self.swapchain_maintenance1 = swapchain_maintenance1.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainPresentFenceInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SwapchainPresentFenceInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain_count: u32,
        pub p_fences: *const Fence,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SwapchainPresentFenceInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwapchainPresentFenceInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain_count", &self.swapchain_count)
                .field("p_fences", &self.p_fences)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SwapchainPresentFenceInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SWAPCHAIN_PRESENT_FENCE_INFO_KHR;
    }

    unsafe impl Extends<PresentInfoKHR<'_>> for SwapchainPresentFenceInfoKHR<'_> {}

    impl Default for SwapchainPresentFenceInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                swapchain_count: Default::default(),
                p_fences: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SwapchainPresentFenceInfoKHR<'a> {
        #[inline]
        pub fn fences(mut self, fences: &'a [Fence]) -> Self {
            self.swapchain_count = fences.len().try_into().unwrap();
            self.p_fences = fences.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainPresentModesCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SwapchainPresentModesCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub present_mode_count: u32,
        pub p_present_modes: *const PresentModeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SwapchainPresentModesCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwapchainPresentModesCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_mode_count", &self.present_mode_count)
                .field("p_present_modes", &self.p_present_modes)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SwapchainPresentModesCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR;
    }

    unsafe impl Extends<SwapchainCreateInfoKHR<'_>> for SwapchainPresentModesCreateInfoKHR<'_> {}

    impl Default for SwapchainPresentModesCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                present_mode_count: Default::default(),
                p_present_modes: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SwapchainPresentModesCreateInfoKHR<'a> {
        #[inline]
        pub fn present_modes(mut self, present_modes: &'a [PresentModeKHR]) -> Self {
            self.present_mode_count = present_modes.len().try_into().unwrap();
            self.p_present_modes = present_modes.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainPresentModeInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SwapchainPresentModeInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain_count: u32,
        pub p_present_modes: *const PresentModeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SwapchainPresentModeInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwapchainPresentModeInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain_count", &self.swapchain_count)
                .field("p_present_modes", &self.p_present_modes)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SwapchainPresentModeInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SWAPCHAIN_PRESENT_MODE_INFO_KHR;
    }

    unsafe impl Extends<PresentInfoKHR<'_>> for SwapchainPresentModeInfoKHR<'_> {}

    impl Default for SwapchainPresentModeInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                swapchain_count: Default::default(),
                p_present_modes: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SwapchainPresentModeInfoKHR<'a> {
        #[inline]
        pub fn present_modes(mut self, present_modes: &'a [PresentModeKHR]) -> Self {
            self.swapchain_count = present_modes.len().try_into().unwrap();
            self.p_present_modes = present_modes.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainPresentScalingCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SwapchainPresentScalingCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub scaling_behavior: PresentScalingFlagsKHR,
        pub present_gravity_x: PresentGravityFlagsKHR,
        pub present_gravity_y: PresentGravityFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SwapchainPresentScalingCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwapchainPresentScalingCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("scaling_behavior", &self.scaling_behavior)
                .field("present_gravity_x", &self.present_gravity_x)
                .field("present_gravity_y", &self.present_gravity_y)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SwapchainPresentScalingCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR;
    }

    unsafe impl Extends<SwapchainCreateInfoKHR<'_>> for SwapchainPresentScalingCreateInfoKHR<'_> {}

    impl Default for SwapchainPresentScalingCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                scaling_behavior: Default::default(),
                present_gravity_x: Default::default(),
                present_gravity_y: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SwapchainPresentScalingCreateInfoKHR<'a> {
        #[inline]
        pub fn scaling_behavior(mut self, scaling_behavior: PresentScalingFlagsKHR) -> Self {
            self.scaling_behavior = scaling_behavior;
            self
        }

        #[inline]
        pub fn present_gravity_x(mut self, present_gravity_x: PresentGravityFlagsKHR) -> Self {
            self.present_gravity_x = present_gravity_x;
            self
        }

        #[inline]
        pub fn present_gravity_y(mut self, present_gravity_y: PresentGravityFlagsKHR) -> Self {
            self.present_gravity_y = present_gravity_y;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkReleaseSwapchainImagesInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ReleaseSwapchainImagesInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain: SwapchainKHR,
        pub image_index_count: u32,
        pub p_image_indices: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ReleaseSwapchainImagesInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ReleaseSwapchainImagesInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain", &self.swapchain)
                .field("image_index_count", &self.image_index_count)
                .field("p_image_indices", &self.p_image_indices)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ReleaseSwapchainImagesInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RELEASE_SWAPCHAIN_IMAGES_INFO_KHR;
    }

    impl Default for ReleaseSwapchainImagesInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                swapchain: Default::default(),
                image_index_count: Default::default(),
                p_image_indices: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ReleaseSwapchainImagesInfoKHR<'a> {
        #[inline]
        pub fn swapchain(mut self, swapchain: SwapchainKHR) -> Self {
            self.swapchain = swapchain;
            self
        }

        #[inline]
        pub fn image_indices(mut self, image_indices: &'a [u32]) -> Self {
            self.image_index_count = image_indices.len().try_into().unwrap();
            self.p_image_indices = image_indices.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleaseSwapchainImagesKHR.html>
    pub type PFN_vkReleaseSwapchainImagesKHR = unsafe extern "system" fn(
        device: Device,
        p_release_info: *const ReleaseSwapchainImagesInfoKHR<'_>,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR =
        PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'static>;
    pub type VkSwapchainPresentFenceInfoKHR = SwapchainPresentFenceInfoKHR<'static>;
    pub type VkSwapchainPresentModesCreateInfoKHR = SwapchainPresentModesCreateInfoKHR<'static>;
    pub type VkSwapchainPresentModeInfoKHR = SwapchainPresentModeInfoKHR<'static>;
    pub type VkSwapchainPresentScalingCreateInfoKHR = SwapchainPresentScalingCreateInfoKHR<'static>;
    pub type VkReleaseSwapchainImagesInfoKHR = ReleaseSwapchainImagesInfoKHR<'static>;
    impl PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SwapchainPresentFenceInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSwapchainPresentFenceInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SwapchainPresentModesCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSwapchainPresentModesCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SwapchainPresentModeInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSwapchainPresentModeInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SwapchainPresentScalingCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSwapchainPresentScalingCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ReleaseSwapchainImagesInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkReleaseSwapchainImagesInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    release_swapchain_images_khr: PFN_vkReleaseSwapchainImagesKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                release_swapchain_images_khr: transmute(
                    load(c"vkReleaseSwapchainImagesKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleaseSwapchainImagesKHR.html>
    #[inline]
    pub unsafe fn release_swapchain_images_khr(
        &self,
        device: Device,
        release_info: &ReleaseSwapchainImagesInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.release_swapchain_images_khr)(device, release_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
