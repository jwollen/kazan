#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_present_barrier";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePresentBarrierFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePresentBarrierFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_barrier: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDevicePresentBarrierFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePresentBarrierFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_barrier", &self.present_barrier)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePresentBarrierFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePresentBarrierFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevicePresentBarrierFeaturesNV<'a> {}

    impl Default for PhysicalDevicePresentBarrierFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                present_barrier: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePresentBarrierFeaturesNV<'a> {
        pub fn present_barrier(mut self, present_barrier: bool) -> Self {
            self.present_barrier = present_barrier.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCapabilitiesPresentBarrierNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SurfaceCapabilitiesPresentBarrierNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_barrier_supported: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for SurfaceCapabilitiesPresentBarrierNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SurfaceCapabilitiesPresentBarrierNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_barrier_supported", &self.present_barrier_supported)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfaceCapabilitiesPresentBarrierNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SURFACE_CAPABILITIES_PRESENT_BARRIER_NV;
    }

    unsafe impl<'a> Extends<SurfaceCapabilities2KHR<'a>> for SurfaceCapabilitiesPresentBarrierNV<'a> {}

    impl Default for SurfaceCapabilitiesPresentBarrierNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                present_barrier_supported: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfaceCapabilitiesPresentBarrierNV<'a> {
        pub fn present_barrier_supported(mut self, present_barrier_supported: bool) -> Self {
            self.present_barrier_supported = present_barrier_supported.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainPresentBarrierCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SwapchainPresentBarrierCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_barrier_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for SwapchainPresentBarrierCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwapchainPresentBarrierCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_barrier_enable", &self.present_barrier_enable)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SwapchainPresentBarrierCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<SwapchainCreateInfoKHR<'a>> for SwapchainPresentBarrierCreateInfoNV<'a> {}

    impl Default for SwapchainPresentBarrierCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                present_barrier_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SwapchainPresentBarrierCreateInfoNV<'a> {
        pub fn present_barrier_enable(mut self, present_barrier_enable: bool) -> Self {
            self.present_barrier_enable = present_barrier_enable.into();
            self
        }
    }
}
