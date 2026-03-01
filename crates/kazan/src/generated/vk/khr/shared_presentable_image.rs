#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SharedPresentSurfaceCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shared_present_supported_usage_flags: ImageUsageFlags,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                shared_present_supported_usage_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SharedPresentSurfaceCapabilitiesKHR<'a> {
        pub fn shared_present_supported_usage_flags(
            mut self,
            shared_present_supported_usage_flags: ImageUsageFlags,
        ) -> Self {
            self.shared_present_supported_usage_flags = shared_present_supported_usage_flags;
            self
        }
    }
    pub type PFN_vkGetSwapchainStatusKHR =
        unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> vk::Result;
}
pub struct DeviceFn {
    get_swapchain_status_khr: PFN_vkGetSwapchainStatusKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_swapchain_status_khr: transmute(
                    load(c"vkGetSwapchainStatusKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_swapchain_status_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_swapchain_status_khr)(device, swapchain);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::SUBOPTIMAL_KHR => Ok(()),
                err => Err(err),
            }
        }
    }
}
