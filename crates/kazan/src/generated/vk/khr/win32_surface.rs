//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_win32_surface.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_win32_surface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWin32SurfaceCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct Win32SurfaceCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: Win32SurfaceCreateFlagsKHR,
        pub hinstance: HINSTANCE,
        pub hwnd: HWND,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for Win32SurfaceCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Win32SurfaceCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("hinstance", &self.hinstance)
                .field("hwnd", &self.hwnd)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for Win32SurfaceCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::WIN32_SURFACE_CREATE_INFO_KHR;
    }

    impl Default for Win32SurfaceCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                hinstance: Default::default(),
                hwnd: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> Win32SurfaceCreateInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: Win32SurfaceCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn hinstance(mut self, hinstance: HINSTANCE) -> Self {
            self.hinstance = hinstance;
            self
        }

        #[inline]
        pub fn hwnd(mut self, hwnd: HWND) -> Self {
            self.hwnd = hwnd;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWin32SurfaceCreateFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Win32SurfaceCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(Win32SurfaceCreateFlagsKHR, Flags);

    impl fmt::Debug for Win32SurfaceCreateFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateWin32SurfaceKHR.html>
    pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const Win32SurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html>
    pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
        ) -> Bool32;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkWin32SurfaceCreateInfoKHR = Win32SurfaceCreateInfoKHR<'static>;
    pub type VkWin32SurfaceCreateFlagsKHR = Win32SurfaceCreateFlagsKHR;
    impl Win32SurfaceCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkWin32SurfaceCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    create_win32_surface_khr: PFN_vkCreateWin32SurfaceKHR,
    get_physical_device_win32_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_win32_surface_khr: transmute(
                    load(c"vkCreateWin32SurfaceKHR").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_win32_presentation_support_khr: transmute(
                    load(c"vkGetPhysicalDeviceWin32PresentationSupportKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateWin32SurfaceKHR.html>
    #[inline]
    pub unsafe fn create_win32_surface_khr(
        &self,
        instance: Instance,
        create_info: &Win32SurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_win32_surface_khr)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                surface.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(surface.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceWin32PresentationSupportKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_win32_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> bool {
        unsafe {
            (self.get_physical_device_win32_presentation_support_khr)(
                physical_device,
                queue_family_index,
            ) != 0
        }
    }
}
