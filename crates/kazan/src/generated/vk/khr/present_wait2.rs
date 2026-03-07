//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_present_wait2.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_present_wait2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentWait2InfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PresentWait2InfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub present_id: u64,
        pub timeout: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PresentWait2InfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PresentWait2InfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_id", &self.present_id)
                .field("timeout", &self.timeout)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PresentWait2InfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_WAIT_2_INFO_KHR;
    }

    impl Default for PresentWait2InfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                present_id: Default::default(),
                timeout: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PresentWait2InfoKHR<'a> {
        #[inline]
        pub fn present_id(mut self, present_id: u64) -> Self {
            self.present_id = present_id;
            self
        }

        #[inline]
        pub fn timeout(mut self, timeout: u64) -> Self {
            self.timeout = timeout;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePresentWait2FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePresentWait2FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_wait2: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePresentWait2FeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePresentWait2FeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_wait2", &self.present_wait2)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePresentWait2FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDevicePresentWait2FeaturesKHR<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevicePresentWait2FeaturesKHR<'a> {}

    impl Default for PhysicalDevicePresentWait2FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                present_wait2: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePresentWait2FeaturesKHR<'a> {
        #[inline]
        pub fn present_wait2(mut self, present_wait2: bool) -> Self {
            self.present_wait2 = present_wait2.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCapabilitiesPresentWait2KHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SurfaceCapabilitiesPresentWait2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_wait2_supported: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SurfaceCapabilitiesPresentWait2KHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SurfaceCapabilitiesPresentWait2KHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_wait2_supported", &self.present_wait2_supported)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfaceCapabilitiesPresentWait2KHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR;
    }

    unsafe impl<'a> Extends<SurfaceCapabilities2KHR<'a>> for SurfaceCapabilitiesPresentWait2KHR<'a> {}

    impl Default for SurfaceCapabilitiesPresentWait2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                present_wait2_supported: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfaceCapabilitiesPresentWait2KHR<'a> {
        #[inline]
        pub fn present_wait2_supported(mut self, present_wait2_supported: bool) -> Self {
            self.present_wait2_supported = present_wait2_supported.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkWaitForPresent2KHR.html>
    pub type PFN_vkWaitForPresent2KHR = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_present_wait2_info: *const PresentWait2InfoKHR<'_>,
    ) -> vk::Result;
}

pub struct DeviceFn {
    wait_for_present2_khr: PFN_vkWaitForPresent2KHR,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                wait_for_present2_khr: transmute(
                    load(c"vkWaitForPresent2KHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkWaitForPresent2KHR.html>
    #[inline]
    pub unsafe fn wait_for_present2_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        present_wait2_info: &PresentWait2InfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.wait_for_present2_khr)(device, swapchain, present_wait2_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
