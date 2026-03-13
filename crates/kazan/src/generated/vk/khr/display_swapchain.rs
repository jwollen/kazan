//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_display_swapchain.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_display_swapchain";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayPresentInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DisplayPresentInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_rect: Rect2D,
        pub dst_rect: Rect2D,
        pub persistent: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DisplayPresentInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DisplayPresentInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_rect", &self.src_rect)
                .field("dst_rect", &self.dst_rect)
                .field("persistent", &self.persistent)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DisplayPresentInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_PRESENT_INFO_KHR;
    }

    unsafe impl Extends<PresentInfoKHR<'_>> for DisplayPresentInfoKHR<'_> {}

    impl Default for DisplayPresentInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_rect: Default::default(),
                dst_rect: Default::default(),
                persistent: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DisplayPresentInfoKHR<'a> {
        #[inline]
        pub fn src_rect(mut self, src_rect: Rect2D) -> Self {
            self.src_rect = src_rect;
            self
        }

        #[inline]
        pub fn dst_rect(mut self, dst_rect: Rect2D) -> Self {
            self.dst_rect = dst_rect;
            self
        }

        #[inline]
        pub fn persistent(mut self, persistent: bool) -> Self {
            self.persistent = persistent.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateSharedSwapchainsKHR.html>
    pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(
        device: Device,
        swapchain_count: u32,
        p_create_infos: *const SwapchainCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_swapchains: *mut SwapchainKHR,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDisplayPresentInfoKHR = DisplayPresentInfoKHR<'static>;
    impl DisplayPresentInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDisplayPresentInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    create_shared_swapchains: PFN_vkCreateSharedSwapchainsKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_shared_swapchains: transmute(
                    load(c"vkCreateSharedSwapchainsKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateSharedSwapchainsKHR.html>
    #[inline]
    pub unsafe fn create_shared_swapchains(
        &self,
        device: Device,
        create_infos: &[SwapchainCreateInfoKHR<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
        swapchains: &mut [SwapchainKHR],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.create_shared_swapchains)(
                device,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                swapchains.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
