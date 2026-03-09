//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_shared_presentable_image.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_shared_presentable_image";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSharedPresentSurfaceCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SharedPresentSurfaceCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shared_present_supported_usage_flags: ImageUsageFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SharedPresentSurfaceCapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SharedPresentSurfaceCapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shared_present_supported_usage_flags",
                    &self.shared_present_supported_usage_flags,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SharedPresentSurfaceCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<SurfaceCapabilities2KHR<'a>> for SharedPresentSurfaceCapabilitiesKHR<'a> {}

    impl Default for SharedPresentSurfaceCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shared_present_supported_usage_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SharedPresentSurfaceCapabilitiesKHR<'a> {
        #[inline]
        pub fn shared_present_supported_usage_flags(
            mut self,
            shared_present_supported_usage_flags: ImageUsageFlags,
        ) -> Self {
            self.shared_present_supported_usage_flags = shared_present_supported_usage_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSwapchainStatusKHR.html>
    pub type PFN_vkGetSwapchainStatusKHR =
        unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkSharedPresentSurfaceCapabilitiesKHR = SharedPresentSurfaceCapabilitiesKHR<'static>;
    impl SharedPresentSurfaceCapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSharedPresentSurfaceCapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_swapchain_status_khr: PFN_vkGetSwapchainStatusKHR,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_swapchain_status_khr: transmute(
                    load(c"vkGetSwapchainStatusKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSwapchainStatusKHR.html>
    #[inline]
    pub unsafe fn get_swapchain_status_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_swapchain_status_khr)(device, swapchain);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
